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
//!    - run all other decoders with heuristic prioritization
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
    calculate_string_worth, generate_heuristic,
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

fn create_new_nodes_from_results(text: String, path: Vec<CrackResult>, cost: u32, decoders_used: Vec<CrackResult>) -> Vec<AStarNode> {
    // text, path, cost, heuristic, total_cost
    // create a vector of new nodes where the next_decoder_name is the name of the decoder
    // loop through every decoder possible and add them to next_decoder_name
    // and update the heuristics based on this
    let mut new_nodes = Vec::new();

    // get all decoders
    let all_decoders = get_all_decoders();
    // loop through all decoders and calculate heuristics
    for decoder in all_decoders.components {
        let decoder_name = decoder.get_name().to_string();
        let heuristic = generate_heuristic(&text, &decoders_used, &Some(decoder));
        let total_cost = cost as f32 + heuristic;
        
        // Create a new node and add it to our collection
        let new_node = AStarNode {
            state: DecoderResult {
                text: vec![text.clone()],
                path: path.clone(),
            },
            cost,
            heuristic,
            total_cost,
            next_decoder_name: Some(decoder_name),
        };
        
        new_nodes.push(new_node);
    }
    
    new_nodes
}

/// Expands a single node and returns a vector of new nodes
fn expand_node(
    current_node: &AStarNode,
    seen_strings: &DashSet<String>,
    _prune_threshold: usize,
    checker: &CheckerTypes, // Add checker parameter
) -> Vec<AStarNode> {
    let mut new_nodes = Vec::new();

    // Determine which decoders to use based on next_decoder_name
    let mut decoders;
    if let Some(decoder_name) = &current_node.next_decoder_name {
        // If we have a specific decoder name, filter all decoders to only include that one
        // this is 
        trace!("Using specific decoder: {}", decoder_name);
        // use get decoder by name from filtration
        decoders = get_decoder_by_name(decoder_name);
        // Update stats for the decoder
    } else {
        decoders = get_decoder_tagged_decoders(&current_node.state);
    }

    // Prevent reciprocal decoders from being applied consecutively
    // a reciprocal decoder is a decoder that can be used to encode and decode the same text
    // example is reverse. Reverse(cat) = tac
    // if we reverse it again Reverse(tac) = cat
    // so we don't want to apply it consecutively
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
        
        // Use the passed checker instead of creating a new one
        // since we only have decoders with the same name
        // we are cheating and just run that one decoder lol
        let decoder_results = decoders.run(&current_node.state.text[0], checker.clone());

        // Process decoder results
        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                // instead of sending results directly,
                // we'll return a special marker node that indicates a successful result
                // this is because we want to ensure that the result is processed first as its the success
                // its just a hacky way to ensure this works with multi processing
                if res.success {
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_default();
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
                    // Clone path to avoid modifying the original
                    let mut decoders_used = current_node.state.path.clone();

                    // Get decoded text
                    let text = r.unencrypted_text.clone().unwrap_or_default();

                    // Skip if text is empty
                    if text.is_empty() {
                        continue;
                    }

                    // Check if string is worth being decoded
                    // uses string heuristics. if heuristic is too low, it goes bye bye!
                    if !calculate_string_worth(&text[0]) {
                        continue;
                    }

                    // Check if we've seen this string before to prevent cycles
                    if !seen_strings.insert(text[0].clone()) {
                        continue;
                    }

                    decoders_used.push(r.clone());

                    // Create new node with updated cost and heuristic
                    // for every "level" node costs increase
                    // to encourage expanding nodes closer to the original text
                    // rather than say 100 levels deep
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

            // Run the decoder using the passed checker
            let result = decoder.crack(&current_node.state.text[0], checker);
            if result.success {
                let mut decoders_used = current_node.state.path.clone();
                let text = result.unencrypted_text.clone().unwrap_or_default();
                debug!("DEBUG: Found successful result with decoder: {} -> {:?}", result.decoder, text);
                decoders_used.push(result.clone());

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

            // Process the result
            if let Some(decoded_text) = &result.unencrypted_text {
                if let Some(first_text) = decoded_text.first() {
                    // Skip if text is empty
                    if first_text.is_empty() {
                        continue;
                    }

                    // Check if we've seen this string before
                    if !seen_strings.insert(first_text.to_string()) {
                        continue;
                    }

                    // Create decoder result
                    let mut decoders_used = current_node.state.path.clone();
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
                }
            }
        }
    }
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

    // Create the Athena checker once
    let athena_checker = Checker::<Athena>::new();
    let checker = CheckerTypes::CheckAthena(athena_checker);

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
                    prune_threshold.load(AtomicOrdering::Relaxed),
                    &checker, // Pass the checker
                )
            })
            .collect();

        // First, identify indices of successful nodes
        let mut successful_node_indices = Vec::new();
        
        for (i, node) in new_nodes.iter().enumerate() {
            if let Some(decoder_name) = &node.next_decoder_name {
                if decoder_name == "__RESULT__" {
                    // Check if we've already processed this result
                    if let Some(text) = node.state.text.first() {
                        if !seen_results.contains(text) {
                            successful_node_indices.push(i);
                        }
                    }
                }
            }
        }
        
        // Process successful nodes if any were found
        if !successful_node_indices.is_empty() {
            // Add the successful results to seen_results to avoid duplicates
            for &idx in &successful_node_indices {
                if let Some(text) = new_nodes[idx].state.text.first() {
                    seen_results.insert(text.clone());
                }
            }
            
            // Process the first successful node for logging and potential return
            let first_idx = successful_node_indices[0];
            let first_successful = &new_nodes[first_idx];
            
            decoded_how_many_times(curr_depth.load(AtomicOrdering::Relaxed));
            
            cli_pretty_printing::success(&format!(
                "Sending successful result with {} decoders",
                first_successful.state.path.len()
            ));
            
            // Process all successful nodes for top_results mode
            if get_config().top_results {
                // Filter out failed decoders from the path before storing
                for &idx in &successful_node_indices {
                    let node = &new_nodes[idx];
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
                let first_node = &new_nodes[first_idx];
                let mut filtered_state = first_node.state.clone();
                filtered_state.path.retain(|decoder| decoder.success);
                
                // In normal mode, send the filtered result and return
                result_sender
                    .send(Some(filtered_state))
                    .expect("Should successfully send the result");
                return;
            }
        }

        // Add non-result nodes to open set
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
