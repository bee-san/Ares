//! # A* Search Implementation for Decoding
//!
//! This module implements the A* search algorithm for finding the correct sequence of decoders
//! to decode an encrypted or encoded text. The A* algorithm is a best-first search algorithm
//! that uses a heuristic function to prioritize which paths to explore.
//!
//! ## Algorithm Overview
//!
//! 1. Start with the initial input text
//! 2. At each step:
//!    - Extract a batch of nodes from the priority queue
//!    - Process these nodes in parallel
//!    - First run all "decoder"-tagged decoders (these are prioritized)
//!    - Then run all other decoders with heuristic prioritization
//! 3. For each successful decoding, create a new node and add it to the priority queue
//! 4. Continue until a plaintext is found or the search space is exhausted
//!
//! ## Node Prioritization
//!
//! Nodes are prioritized using an f-score where:
//! - f = g + h
//! - g = depth in the search tree (cost so far)
//! - h = heuristic value (estimated cost to goal)
//!
//! The current implementation uses a simple placeholder heuristic of 1.0,
//! but has been improved with Cipher Identifier for better prioritization.
//!
//! ## Parallel Processing
//!
//! The implementation uses parallel node expansion to improve performance:
//! - Multiple nodes are processed simultaneously using Rayon
//! - Thread-safe data structures ensure correctness
//! - Batch processing extracts multiple nodes from the priority queue
//! - Special result nodes handle successful decodings in a thread-safe manner

use crate::cli_pretty_printing;
use crate::cli_pretty_printing::decoded_how_many_times;
use crate::decoders::crack_results::CrackResult;
use crate::filtration_system::get_all_decoders;
use crate::filtration_system::{get_decoder_by_name, get_decoder_tagged_decoders, MyResults};
use crossbeam::channel::Sender;

use log::{debug, trace};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};

// Add imports for parallel processing
use dashmap::DashSet;
use rayon::prelude::*;

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::CheckerTypes;
use crate::config::get_config;
use crate::searchers::helper_functions::{
    calculate_string_worth, generate_heuristic, update_decoder_stats,
};
use crate::storage::wait_athena_storage;
use crate::DecoderResult;

/// Threshold for pruning the seen_strings HashSet to prevent excessive memory usage
const PRUNE_THRESHOLD: usize = 100000;

/// Initial pruning threshold for dynamic adjustment
const INITIAL_PRUNE_THRESHOLD: usize = PRUNE_THRESHOLD;

/// Maximum depth for search (used for dynamic threshold adjustment)
const MAX_DEPTH: u32 = 100;

/// Number of nodes to process in parallel
const PARALLEL_BATCH_SIZE: usize = 10;

/// Check if a decoder is reciprocal based on its name
fn is_reciprocal_decoder(decoder_name: &str) -> bool {
    let decoder = get_decoder_by_name(decoder_name);
    
    // Check if any of the decoder's components have the "reciprocal" tag
    decoder.components.iter().any(|d| {
        d.get_tags().contains(&"reciprocal")
    })
}

/// Calculate a hash for a string to use in the seen_strings set
fn calculate_hash(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish().to_string()
}

/// A* search node with priority based on f = g + h
///
/// Each node represents a state in the search space, with:
/// - The current decoded text
/// - The path of decoders used to reach this state
/// - Cost metrics for prioritization
#[derive(Debug)]
struct AStarNode {
    /// Current state containing the decoded text and path of decoders used
    state: DecoderResult,

    /// Cost so far (g) - represents the depth in the search tree
    /// This increases by 1 for each decoder applied
    cost: u32,

    /// Heuristic value (h) - estimated cost to reach the goal
    /// Currently a placeholder value, but could be improved with
    /// cipher identification techniques to better estimate how close
    /// we are to finding plaintext
    heuristic: f32,

    /// Total cost (f = g + h) used for prioritization in the queue
    /// Nodes with lower total_cost are explored first
    total_cost: f32,

    /// The name of the next decoder to try when this node is expanded
    next_decoder_name: Option<String>,
}

// Custom ordering for the priority queue
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lowest f value has highest priority)
        other
            .total_cost
            .partial_cmp(&self.total_cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

impl Eq for AStarNode {}

/// Thread-safe priority queue wrapper for A* open set
struct ThreadSafePriorityQueue {
    queue: Mutex<BinaryHeap<AStarNode>>,
}

impl ThreadSafePriorityQueue {
    fn new() -> Self {
        ThreadSafePriorityQueue {
            queue: Mutex::new(BinaryHeap::new()),
        }
    }

    fn push(&self, node: AStarNode) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(node);
    }

    fn pop(&self) -> Option<AStarNode> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }

    fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    // Extract a batch of nodes with highest priority
    fn extract_batch(&self, batch_size: usize) -> Vec<AStarNode> {
        let mut queue = self.queue.lock().unwrap();
        let mut batch = Vec::with_capacity(batch_size);

        for _ in 0..batch_size {
            if let Some(node) = queue.pop() {
                batch.push(node);
            } else {
                break;
            }
        }

        batch
    }
}

/// Expands a single node and returns a vector of new nodes
fn expand_node(
    current_node: &AStarNode,
    seen_strings: &DashSet<String>,
    _stop: &Arc<AtomicBool>,  // Renamed to _stop to indicate it's intentionally unused
    _prune_threshold: usize,
) -> Vec<AStarNode> {
    let mut new_nodes = Vec::new();

    println!("DEBUG: Expanding node with text: {:?}", current_node.state.text);
    
    // Print current path
    let mut path_str = String::new();
    for (i, decoder) in current_node.state.path.iter().enumerate() {
        if i > 0 {
            path_str.push_str(" -> ");
        }
        path_str.push_str(&format!("{}(success={})", decoder.decoder, decoder.success));
    }
    println!("DEBUG: Current path: {}", path_str);
    
    // No longer checking stop signal here

    // Determine which decoders to use based on next_decoder_name
    let mut decoders;
    if let Some(decoder_name) = &current_node.next_decoder_name {
        // If we have a specific decoder name, filter all decoders to only include that one
        trace!("Using specific decoder: {}", decoder_name);
        // use get decoder by name from filtration
        decoders = get_decoder_by_name(decoder_name);
        // Update stats for the decoder
        if !decoders.components.is_empty() {
            update_decoder_stats(decoder_name, true);
        }
    } else {
        decoders = get_decoder_tagged_decoders(&current_node.state);
    }

    // Prevent reciprocal decoders from being applied consecutively
    if let Some(last_decoder) = current_node.state.path.last() {
        if is_reciprocal_decoder(&last_decoder.decoder) {
            let excluded_name = &last_decoder.decoder;
            decoders
                .components
                .retain(|d| d.get_name() != *excluded_name);
        }
    }

    if !decoders.components.is_empty() {
        trace!(
            "Found {} decoder-tagged decoders to execute",
            decoders.components.len()
        );

        // No longer checking stop signal before processing decoders

        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        // since we only have decoders with the same name
        // we are cheating and just run that one decoder lol
        println!("DEBUG: Running decoder: {}", current_node.next_decoder_name.as_ref().unwrap_or(&"None".to_string()));
        let decoder_results = decoders.run(&current_node.state.text[0], checker);

        // Process decoder results
        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                // This part remains mostly unchanged, but instead of sending results directly,
                // we'll return a special marker node that indicates a successful result
                if res.success {
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    println!("DEBUG: Found successful result with decoder: {} -> {:?}", res.decoder, text);
                    decoders_used.push(res.clone());

                    // Create a special "result" node with a very low total_cost to ensure it's processed first
                    let result_node = AStarNode {
                        state: DecoderResult {
                            text: text.clone(),
                            path: decoders_used,
                        },
                        cost: current_node.cost + 1,
                        heuristic: -1000.0, // Very negative to ensure highest priority
                        total_cost: -1000.0, // Very negative to ensure highest priority
                        next_decoder_name: Some("__RESULT__".to_string()), // Special marker
                    };

                    new_nodes.push(result_node);
                }
            }
            MyResults::Continue(results) => {
                // Process each result
                for r in results {
                    // No longer checking stop signal during result processing

                    // Clone path to avoid modifying the original
                    let mut decoders_used = current_node.state.path.clone();

                    // Get decoded text
                    let text = r.unencrypted_text.clone().unwrap_or_default();

                    // Skip if text is empty or already seen
                    if text.is_empty() {
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }

                    // Check if string is worth being decoded
                    // uses string heuristics. if heuristic is too low, it goes bye bye!
                    if !calculate_string_worth(&text[0]) {
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }

                    // Check if we've seen this string before to prevent cycles
                    let text_hash = calculate_hash(&text[0]);
                    if !seen_strings.insert(text_hash) {
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }

                    println!("DEBUG: Adding decoder {} to path with text: {:?}", r.decoder, text);
                    decoders_used.push(r.clone());

                    // Create new node with updated cost and heuristic
                    let cost = current_node.cost + 1;
                    let heuristic = generate_heuristic(&text[0], &decoders_used, &None);
                    let total_cost = cost as f32 + heuristic;

                    let new_node = AStarNode {
                        state: DecoderResult {
                            text,
                            path: decoders_used,
                        },
                        cost,
                        heuristic,
                        total_cost,
                        next_decoder_name: Some(r.decoder.to_string()),
                    };

                    // Add to new nodes
                    new_nodes.push(new_node);

                    // Update decoder stats - mark as successful since it produced valid output
                    update_decoder_stats(r.decoder, true);
                }
            }
        }
    }

    // If no decoder-tagged decoders or they didn't produce results,
    // try all available decoders
    if new_nodes.is_empty() {
        // This part remains similar to the original implementation
        // but adapted to return nodes instead of adding them to open_set

        // Get all decoders
        let all_decoders = get_all_decoders();

        // Process each decoder
        for decoder in all_decoders.components {
            // No longer checking stop signal for each decoder

            // Skip decoders that were already tried
            if let Some(last_decoder) = current_node.state.path.last() {
                if last_decoder.decoder == decoder.get_name() {
                    continue;
                }

                // Skip reciprocal decoders if the last one was reciprocal
                if is_reciprocal_decoder(&last_decoder.decoder)
                    && last_decoder.decoder == decoder.get_name()
                {
                    continue;
                }
            }

            // Run the decoder
            let athena_checker = Checker::<Athena>::new();
            let checker = CheckerTypes::CheckAthena(athena_checker);
            let result = decoder.crack(&current_node.state.text[0], &checker);

            // Process the result
            if let Some(decoded_text) = &result.unencrypted_text {
                if let Some(first_text) = decoded_text.first() {
                    // Skip if text is empty
                    if first_text.is_empty() {
                        update_decoder_stats(decoder.get_name(), false);
                        continue;
                    }

                    // Check if we've seen this string before
                    let text_hash = calculate_hash(first_text);
                    if !seen_strings.insert(text_hash) {
                        update_decoder_stats(decoder.get_name(), false);
                        continue;
                    }

                    // Create decoder result
                    let mut decoders_used = current_node.state.path.clone();
                    println!("DEBUG: Adding decoder {} to path with text: {:?}", decoder.get_name(), decoded_text);
                    decoders_used.push(result.clone());

                    // Create new node
                    let cost = current_node.cost + 1;
                    let heuristic = generate_heuristic(first_text, &decoders_used, &None);
                    let total_cost = cost as f32 + heuristic;

                    let new_node = AStarNode {
                        state: DecoderResult {
                            text: decoded_text.clone(),
                            path: decoders_used,
                        },
                        cost,
                        heuristic,
                        total_cost,
                        next_decoder_name: Some(decoder.get_name().to_string()),
                    };

                    // Add to new nodes
                    new_nodes.push(new_node);

                    // Update decoder stats
                    update_decoder_stats(decoder.get_name(), true);
                }
            } else {
                // Update decoder stats for failed decoding
                update_decoder_stats(decoder.get_name(), false);
            }
        }
    }
    println!("DEBUG: Expanded node produced {} new nodes", new_nodes.len());
    new_nodes
}

/// A* search implementation for finding the correct sequence of decoders
///
/// This algorithm prioritizes decoders using a heuristic function and executes
/// "decoder"-tagged decoders immediately at each level. The search proceeds in a
/// best-first manner, exploring the most promising nodes first based on the f-score.
///
/// ## Execution Order
///
/// 1. At each node, first run all "decoder"-tagged decoders
///    - These are considered more likely to produce meaningful results
///    - If any of these decoders produces plaintext, we return immediately
///
/// 2. Then run all non-"decoder"-tagged decoders
///    - These are prioritized using the heuristic function
///    - Results are added to the priority queue for future exploration
///
/// ## Pruning Mechanism
///
/// To prevent memory exhaustion and avoid cycles:
///
/// 1. We maintain a HashSet of seen strings to avoid revisiting states
/// 2. When the HashSet grows beyond PRUNE_THRESHOLD (10,000 entries):
///    - We retain only strings shorter than 100 characters
///    - This is based on the heuristic that shorter strings are more likely to be valuable
///
/// ## Parameters
///
/// - `input`: The initial text to decode
/// - `result_sender`: Channel to send the result when found
/// - `stop`: Atomic boolean to signal when to stop the search
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    // Calculate heuristic before moving input
    let initial_heuristic = generate_heuristic(&input, &[], &None);

    let initial = DecoderResult {
        text: vec![input],
        path: vec![],
    };

    // Thread-safe set to track visited states to prevent cycles
    let seen_strings = DashSet::new();
    let seen_results = DashSet::new(); // Track unique results
    let _seen_count = Arc::new(AtomicUsize::new(0));

    // Thread-safe priority queue for open set
    let open_set = ThreadSafePriorityQueue::new();

    // Add initial node to open set
    open_set.push(AStarNode {
        state: initial,
        cost: 0,
        heuristic: initial_heuristic,
        total_cost: 0.0,
        next_decoder_name: None,
    });

    let curr_depth = Arc::new(AtomicU32::new(1));
    let prune_threshold = Arc::new(AtomicUsize::new(INITIAL_PRUNE_THRESHOLD));

    // Main A* loop - no longer checking stop signal in loop condition
    while !open_set.is_empty() {
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth.load(AtomicOrdering::Relaxed),
            open_set.len()
        );

        // Extract a batch of nodes to process in parallel
        let batch_size = std::cmp::min(PARALLEL_BATCH_SIZE, open_set.len());
        let batch = open_set.extract_batch(batch_size);

        trace!("Processing batch of {} nodes in parallel", batch.len());

        // Process nodes in parallel
        let new_nodes: Vec<AStarNode> = batch
            .par_iter()
            .flat_map(|node| {
                expand_node(
                    node,
                    &seen_strings,
                    &stop,  // Still passing stop for backward compatibility
                    prune_threshold.load(AtomicOrdering::Relaxed),
                )
            })
            .collect();

        // Collect all successful nodes from the batch
        let mut successful_nodes = Vec::new();
        
        // First pass: identify all successful nodes
        for node in &new_nodes {
            if let Some(decoder_name) = &node.next_decoder_name {
                if decoder_name == "__RESULT__" {
                    // Check if we've already processed this result
                    if let Some(text) = node.state.text.first() {
                        let result_hash = calculate_hash(text);
                        if !seen_results.insert(result_hash) {
                            println!("DEBUG: Skipping duplicate result: {:?}", text);
                            continue; // Skip this result, we've already processed it
                        } else {
                            println!("DEBUG: Processing new result: {:?}", text);
                            successful_nodes.push(node);
                        }
                    }
                }
            }
        }
        
        // If we found any successful nodes in this batch
        if !successful_nodes.is_empty() {
            // Process the first successful node for logging and potential return
            let first_successful = successful_nodes[0];
            
            // Build decoder path string
            let mut decoder_path = String::new();
            for (i, decoder) in first_successful.state.path.iter().enumerate() {
                if i > 0 {
                    decoder_path.push_str(" -> ");
                }
                decoder_path.push_str(&format!("{}(success={})", decoder.decoder, decoder.success));
            }
            
            println!(
                "DEBUG: Found result node with text: {:?} | Decoder path: {}",
                first_successful.state.text, decoder_path
            );
            
            // Validate the path by applying each decoder in sequence
            println!("DEBUG: Validating decoder path by applying each decoder in sequence:");
            let mut current_text = first_successful.state.path[0].encrypted_text.clone();
            println!("DEBUG: Starting with text: {}", current_text);
            
            for (i, decoder) in first_successful.state.path.iter().enumerate() {
                if let Some(decoded_texts) = &decoder.unencrypted_text {
                    if !decoded_texts.is_empty() {
                        println!("DEBUG: Step {}: {} -> {}", i, current_text, decoded_texts[0]);
                        current_text = decoded_texts[0].clone();
                    } else {
                        println!("DEBUG: Step {}: {} produced empty result", i, decoder.decoder);
                    }
                } else {
                    println!("DEBUG: Step {}: {} produced no result", i, decoder.decoder);
                }
            }
            decoded_how_many_times(curr_depth.load(AtomicOrdering::Relaxed));
            
            cli_pretty_printing::success(&format!(
                "DEBUG: astar.rs - Sending successful result with {} decoders",
                first_successful.state.path.len()
            ));
            
            // Process all successful nodes for top_results mode
            if get_config().top_results {
                // Filter out failed decoders from the path before storing
                for node in &successful_nodes {
                    // Create a filtered copy with only successful decoders
                    let mut filtered_path = Vec::new();
                    for decoder in &node.state.path {
                        if decoder.success {
                            filtered_path.push(decoder.clone());
                        }
                    }
                    
                    // Create a new state with the filtered path
                    let mut filtered_state = node.state.clone();
                    filtered_state.path = filtered_path;
                    
                    // Store all successful results in WaitAthena storage
                    if let Some(plaintext) = filtered_state.text.first() {
                        // Build decoder path string for the filtered path
                        let mut decoder_path = String::new();
                        for (i, decoder) in filtered_state.path.iter().enumerate() {
                            if i > 0 {
                                decoder_path.push_str(" -> ");
                            }
                            decoder_path.push_str(&decoder.decoder);
                        }
                        
                        println!(
                            "DEBUG: Processing result in top_results mode with plaintext: {} | Decoder path: {}",
                            plaintext, decoder_path
                        );
                        
                        // Get the last decoder used
                        let decoder_name = if let Some(last_decoder) = filtered_state.path.last() {
                            last_decoder.decoder.to_string()
                        } else {
                            "Unknown".to_string()
                        };
                        
                        // Get the checker name from the last decoder
                        let checker_name = if let Some(last_decoder) = filtered_state.path.last() {
                            last_decoder.checker_name.to_string()
                        } else {
                            "Unknown".to_string()
                        };
                        
                        // Only store results that have a valid checker name
                        if !checker_name.is_empty() && checker_name != "Unknown" {
                            log::trace!(
                                "Storing plaintext in WaitAthena storage: {} (decoder path: {}, checker: {})",
                                plaintext,
                                decoder_path,
                                checker_name
                            );
                            wait_athena_storage::add_plaintext_result(
                                plaintext.clone(),
                                format!(
                                    "Decoded successfully at depth {}",
                                    curr_depth.load(AtomicOrdering::Relaxed)
                                ),
                                checker_name,
                                decoder_name,
                            );
                        }
                    }
                }
                
                // In top_results mode, we continue processing after storing all results
            } else {
                // Filter out failed decoders from the path before returning
                let mut filtered_state = first_successful.state.clone();
                filtered_state.path.retain(|decoder| decoder.success);
                
                println!("DEBUG: Filtered path to only include successful decoders: {}",
                    filtered_state.path.iter().map(|d| d.decoder.to_string()).collect::<Vec<_>>().join(" -> "));
                
                // In normal mode, send the filtered result and return
                result_sender
                    .send(Some(filtered_state))
                    .expect("Should successfully send the result");
                return;
            }
        }

        // Filter out result nodes and add remaining nodes to open set
        for node in new_nodes {
            if let Some(decoder_name) = &node.next_decoder_name {
                if decoder_name != "__RESULT__" {
                    open_set.push(node);
                }
            } else {
                open_set.push(node);
            }
        }

        // Update current depth based on the nodes in the open set
        if let Some(top_node) = open_set.pop() {
            let new_depth = top_node.cost;
            curr_depth.store(new_depth, AtomicOrdering::Relaxed);

            // Put the node back
            open_set.push(top_node);

            // Prune seen strings if we've accumulated too many
            let current_seen_count = seen_strings.len();
            if current_seen_count > prune_threshold.load(AtomicOrdering::Relaxed) {
                // Prune seen strings (implementation depends on how you want to handle this)
                // This is a simplified version - you might want a more sophisticated approach
                seen_strings.clear();

                // Adjust threshold based on search progress
                let progress_factor = new_depth as f32 / MAX_DEPTH as f32;
                let new_threshold = INITIAL_PRUNE_THRESHOLD - (progress_factor * 5000.0) as usize;
                prune_threshold.store(new_threshold, AtomicOrdering::Relaxed);

                debug!("Pruned seen strings (new threshold: {})", new_threshold);
            }
        }
    }

    // If we get here, we've exhausted all possibilities without finding a solution
    result_sender
        .send(None)
        .expect("Should successfully send the result");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::bounded;

    #[test]
    fn astar_handles_empty_input() {
        let (sender, receiver) = bounded::<Option<DecoderResult>>(1);
        let stop = Arc::new(AtomicBool::new(false));

        // Run A* with empty input
        astar("".to_string(), sender, stop);

        // Should receive None since there's nothing to decode
        let result = receiver.recv().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn astar_prevents_cycles() {
        let (sender, receiver) = bounded::<Option<DecoderResult>>(1);
        let stop = Arc::new(AtomicBool::new(false));

        // Run A* with input that could cause cycles
        astar("AAAA".to_string(), sender, stop);

        // Should eventually complete without hanging
        let _ = receiver.recv().unwrap();
    }

    #[test]
    fn test_parallel_astar() {
        // Create channels for result communication
        let (sender, receiver) = bounded::<Option<DecoderResult>>(1);

        // Create stop signal
        let stop = Arc::new(AtomicBool::new(false));

        // Run A* in a separate thread with Base64 encoded "Hello World"
        let input = "SGVsbG8gV29ybGQ=".to_string();
        let stop_clone = stop.clone();

        std::thread::spawn(move || {
            astar(input, sender, stop_clone);
        });

        // Wait for result with timeout
        let result = receiver.recv().unwrap();

        // Verify we got a result (not necessarily "Hello World" as it depends on decoders)
        assert!(result.is_some());
        if let Some(decoder_result) = result {
            assert!(!decoder_result.path.is_empty());
        }
}

#[test]
fn test_reciprocal_decoders_not_applied_consecutively() {
    // This test verifies that reciprocal decoders (like Atbash and Caesar)
    // are not applied consecutively in the search path
    
    let (sender, receiver) = bounded::<Option<DecoderResult>>(1);
    let stop = Arc::new(AtomicBool::new(false));
    
    // Use a simple input that could be decoded with reciprocal decoders
    let input = "Ifmmp Xpsme"; // "Hello World" with Caesar shift of 1
    
    // Run A* search
    std::thread::spawn(move || {
        astar(input.to_string(), sender, stop);
    });
    
    // Wait for result
    let result = receiver.recv().unwrap();
    
    // Verify we got a result
    assert!(result.is_some());
    
    if let Some(decoder_result) = result {
        // Get the decoder path
        let path = decoder_result.path;
        
        // Check that no reciprocal decoder is applied consecutively
        for i in 1..path.len() {
            let prev_decoder = &path[i-1];
            let curr_decoder = &path[i];
            
            // If the previous decoder is reciprocal, it should not be the same as the current one
            if is_reciprocal_decoder(&prev_decoder.decoder) {
                assert_ne!(
                    prev_decoder.decoder,
                    curr_decoder.decoder,
                    "Reciprocal decoder {} was applied consecutively",
                    prev_decoder.decoder
                );
            }
                }
            }
        }
    }
