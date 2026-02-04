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
//!    - For each node, rank available decoders by estimated cost
//!    - Run only the top X decoders (configurable via decoder_batch_size)
//!    - Queue remaining decoders for later exploration
//! 3. For each successful decoding, create a new node and add it to the priority queue
//! 4. Continue until a plaintext is found or the search space is exhausted
//!
//! ## Node Prioritization
//!
//! Nodes are prioritized using an f-score where:
//! - f = g + h
//! - g = path complexity + depth penalty (cost so far)
//! - h = heuristic value (estimated cost to goal)
//!
//! The depth penalty ensures that shallow unexplored paths eventually become
//! competitive with deep explored paths, preventing the algorithm from
//! getting stuck exploring only one branch.
//!
//! ## Parallel Processing
//!
//! The implementation uses parallel node expansion to improve performance:
//! - Multiple nodes are processed simultaneously using Rayon
//! - Thread-safe data structures ensure correctness
//! - Batch processing extracts multiple nodes from the priority queue
//! - Special result nodes handle successful decodings in a thread-safe manner

use crate::cli_pretty_printing::decoded_how_many_times;
use crate::filtration_system::{get_all_decoders, get_decoder_by_name};
use crate::CrackResult;
use crossbeam::channel::Sender;

use log::{debug, trace};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};

// Add imports for parallel processing
use dashmap::DashSet;
use rayon::prelude::*;

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::get_human_confirmed_text;
use crate::checkers::CheckerTypes;
use crate::config::get_config;
use crate::searchers::helper_functions::{
    calculate_path_complexity, calculate_string_worth, generate_heuristic, rank_decoders,
    update_decoder_stats,
};
use crate::storage::wait_athena_storage;
use crate::DecoderResult;

/// Number of nodes to process in parallel
const PARALLEL_BATCH_SIZE: usize = 10;

/// Distinguishes between regular search nodes and successful result nodes
enum NodeType {
    /// Regular node to continue exploring
    Regular {
        /// The name of the next decoder to try when this node is expanded (if specific)
        next_decoder: Option<String>,
        /// Decoders that haven't been tried yet at this node (for batched exploration)
        /// If empty, all decoders have been tried
        untried_decoders: Vec<String>,
    },
    /// Node containing a successful decoding result
    Result,
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
///
/// ## Cost Calculation (Occam's Razor + Depth Penalty)
///
/// The cost (g) uses "path complexity" rather than raw depth, implementing
/// Occam's Razor by favoring simpler explanations:
/// - Repeated same-encoder applications are cheap (0.2 each)
/// - Different encoders cost more (0.7 each)
/// - Ciphers are expensive (2.0+, escalating for multiple)
///
/// Additionally, a depth penalty is added to ensure unexplored shallow paths
/// eventually become competitive with deep explored paths:
/// - cost = path_complexity + (depth * depth_penalty)
///
/// This means base64×10 is cheaper than caesar→vigenere→atbash,
/// but eventually trying caesar at depth 0 becomes competitive with
/// going deeper into base64 chains.
#[derive(Debug)]
struct AStarNode {
    /// Current state containing the decoded text and path of decoders used
    state: DecoderResult,

    /// Depth in the search tree (path length)
    depth: usize,

    /// Cost so far (g) - represents the path complexity + depth penalty
    /// Uses category-aware complexity rather than raw depth
    cost: f32,

    /// Heuristic value (h) - estimated cost to reach the goal
    /// Based on entropy (lower = more plaintext-like) and
    /// decoder success rates from adaptive learning
    heuristic: f32,

    /// Total cost (f = g + h) used for prioritization in the queue
    /// Nodes with lower total_cost are explored first
    total_cost: f32,

    /// The type of this node - either a regular search node or a result node
    node_type: NodeType,
}

// Implement Debug for NodeType manually since it's used in AStarNode
impl std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Regular {
                next_decoder,
                untried_decoders,
            } => f
                .debug_struct("Regular")
                .field("next_decoder", next_decoder)
                .field("untried_count", &untried_decoders.len())
                .finish(),
            NodeType::Result => write!(f, "Result"),
        }
    }
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

/// Extract a batch of nodes from the priority queue
fn extract_batch(queue: &Mutex<BinaryHeap<AStarNode>>, batch_size: usize) -> Vec<AStarNode> {
    let mut q = queue.lock().unwrap();
    let mut batch = Vec::with_capacity(batch_size);
    for _ in 0..batch_size {
        if let Some(node) = q.pop() {
            batch.push(node);
        } else {
            break;
        }
    }
    batch
}

/// Process a decoder result and create a new AStarNode if valid
fn create_node_from_result(
    result: &CrackResult,
    current_node: &AStarNode,
    seen_strings: &DashSet<String>,
    all_decoder_names: &[String],
) -> Option<AStarNode> {
    let text = result.unencrypted_text.clone()?;

    // Skip empty
    if text.is_empty() || text[0].is_empty() {
        update_decoder_stats(result.decoder, false);
        return None;
    }

    // Skip low-quality strings
    if !calculate_string_worth(&text[0]) {
        update_decoder_stats(result.decoder, false);
        return None;
    }

    // Skip already-seen
    let text_hash = calculate_hash(&text[0]);
    if !seen_strings.insert(text_hash) {
        update_decoder_stats(result.decoder, false);
        return None;
    }

    // Build path
    let mut path = current_node.state.path.clone();
    path.push(result.clone());

    let config = get_config();
    let depth = path.len();

    // Calculate cost using path complexity (Occam's Razor aware) + depth penalty
    // The depth penalty ensures shallow unexplored paths eventually become competitive
    let path_complexity = calculate_path_complexity(&path);
    let depth_penalty = depth as f32 * config.depth_penalty;
    let cost = path_complexity + depth_penalty;

    let heuristic = generate_heuristic(&text[0], &path, &None);

    update_decoder_stats(result.decoder, true);

    Some(AStarNode {
        state: DecoderResult { text, path },
        depth,
        cost,
        heuristic,
        total_cost: cost + heuristic,
        node_type: NodeType::Regular {
            // Don't specify a next decoder - let A* try all decoders
            // The heuristic will naturally prioritize the same decoder for nested encodings
            next_decoder: None,
            // All decoders are untried at a new node
            untried_decoders: all_decoder_names.to_vec(),
        },
    })
}

/// Expands a single node and returns a vector of new nodes
///
/// This function implements batched decoder exploration:
/// 1. If the node has untried decoders, rank them by estimated cost
/// 2. Take the top X decoders (decoder_batch_size from config)
/// 3. Run those decoders and create child nodes
/// 4. If there are remaining untried decoders, create a "continuation node"
///    that will try the next batch later
fn expand_node(
    current_node: &AStarNode,
    seen_strings: &DashSet<String>,
    stop: &Arc<AtomicBool>,
    all_decoder_names: &[String],
) -> Vec<AStarNode> {
    let mut new_nodes = Vec::new();

    // Check stop signal
    if stop.load(AtomicOrdering::Relaxed) {
        return new_nodes;
    }

    // Result nodes should not be expanded
    if matches!(current_node.node_type, NodeType::Result) {
        return new_nodes;
    }

    // Get the untried decoders and next decoder from the node type
    let (next_decoder_name, untried_decoders) = match &current_node.node_type {
        NodeType::Regular {
            next_decoder,
            untried_decoders,
        } => (next_decoder.clone(), untried_decoders.clone()),
        NodeType::Result => return new_nodes,
    };

    let config = get_config();
    let batch_size = config.decoder_batch_size;

    // Get decoders to run this iteration
    let decoders_to_run: Vec<String> = if let Some(decoder_name) = &next_decoder_name {
        // If we have a specific decoder name, only run that one
        trace!("Using specific decoder: {}", decoder_name);
        vec![decoder_name.clone()]
    } else if untried_decoders.is_empty() {
        // No more decoders to try at this node
        return new_nodes;
    } else {
        // Rank untried decoders and take top batch_size
        let ranked = rank_decoders(
            &current_node.state.text[0],
            &current_node.state.path,
            &untried_decoders,
        );

        trace!(
            "Depth {}: Ranked {} decoders, taking top {}",
            current_node.depth,
            ranked.len(),
            batch_size
        );

        ranked
            .into_iter()
            .take(batch_size)
            .map(|rd| rd.name)
            .collect()
    };

    // Calculate remaining decoders for continuation node
    let remaining_decoders: Vec<String> = untried_decoders
        .into_iter()
        .filter(|d| !decoders_to_run.contains(d))
        .collect();

    // Get the actual decoder components to run
    let mut decoders = crate::filtration_system::Decoders {
        components: Vec::new(),
    };

    for decoder_name in &decoders_to_run {
        let d = get_decoder_by_name(decoder_name);
        decoders.components.extend(d.components);
    }

    // Prevent reciprocal decoders from being applied consecutively
    if let Some(last_decoder) = current_node.state.path.last() {
        if last_decoder.checker_description.contains("reciprocal") {
            let excluded_name = &last_decoder.decoder;
            decoders
                .components
                .retain(|d| d.get_name() != *excluded_name);
        }
    }

    if decoders.components.is_empty() && remaining_decoders.is_empty() {
        return new_nodes;
    }

    // Check stop signal before processing
    if stop.load(AtomicOrdering::Relaxed) {
        return new_nodes;
    }

    // Create checker
    let checker = CheckerTypes::CheckAthena(Checker::<Athena>::new());

    // Run the batch of decoders
    if !decoders.components.is_empty() {
        let decoder_results = decoders.run(&current_node.state.text[0], checker);

        // Get all results - both successful and unsuccessful
        let results_to_process: Vec<CrackResult> = decoder_results.all_results();

        for r in results_to_process {
            if stop.load(AtomicOrdering::Relaxed) {
                break;
            }

            // If a decoder succeeded, create a result node
            if r.success {
                let mut decoders_used = current_node.state.path.clone();
                let text = r.unencrypted_text.clone().unwrap_or_default();
                decoders_used.push(r.clone());

                trace!(
                    "Result found at depth {}: decoder={}, text_preview='{}'",
                    decoders_used.len(),
                    r.decoder,
                    text.get(0)
                        .map(|t| t.chars().take(50).collect::<String>())
                        .unwrap_or_default()
                );

                let depth = decoders_used.len();
                let path_complexity = calculate_path_complexity(&decoders_used);
                let depth_penalty = depth as f32 * config.depth_penalty;
                let result_cost = path_complexity + depth_penalty;

                let result_node = AStarNode {
                    state: DecoderResult {
                        text: text.clone(),
                        path: decoders_used,
                    },
                    depth,
                    cost: result_cost,
                    heuristic: -1000.0, // Very negative to ensure highest priority
                    total_cost: -1000.0,
                    node_type: NodeType::Result,
                };
                new_nodes.push(result_node);
            } else if let Some(node) =
                create_node_from_result(&r, current_node, seen_strings, all_decoder_names)
            {
                // Create regular node for further exploration
                new_nodes.push(node);
            }
        }
    }

    // If there are remaining decoders, create a continuation node
    // This node represents "trying more decoders at the same input"
    if !remaining_decoders.is_empty() {
        trace!(
            "Creating continuation node with {} remaining decoders",
            remaining_decoders.len()
        );

        // The continuation node has slightly higher cost to prefer exploring
        // results from tried decoders first, but not too high
        let continuation_penalty = 0.05; // Small penalty per batch
        let continuation_cost = current_node.cost + continuation_penalty;

        let continuation_node = AStarNode {
            state: current_node.state.clone(),
            depth: current_node.depth,
            cost: continuation_cost,
            heuristic: current_node.heuristic,
            total_cost: continuation_cost + current_node.heuristic,
            node_type: NodeType::Regular {
                next_decoder: None,
                untried_decoders: remaining_decoders,
            },
        };
        new_nodes.push(continuation_node);
    }

    new_nodes
}

/// A* search implementation for finding the correct sequence of decoders
///
/// This algorithm uses A* search with Occam's Razor-based heuristics to find the
/// simplest decoding path. It explores decoders in batches (configurable via
/// decoder_batch_size), with the path complexity function naturally prioritizing:
/// - Encoders (cheap) over ciphers (expensive)
/// - Repeated same-encoder sequences (very cheap)
/// - Shorter paths over longer paths
///
/// A depth penalty ensures that unexplored shallow paths eventually become
/// competitive with deep explored paths.
///
/// ## Heuristic Design
///
/// The f-score (f = g + h) prioritizes exploration:
/// - g (path cost): path_complexity + (depth * depth_penalty)
/// - h (heuristic): Based on entropy, string quality, decoder success rates
///
/// ## Parameters
///
/// - `input`: The initial text to decode
/// - `result_sender`: Channel to send the result when found
/// - `stop`: Atomic boolean to signal when to stop the search
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    // Get all decoder names upfront for creating new nodes
    let all_decoders = get_all_decoders();
    let all_decoder_names: Vec<String> = all_decoders
        .components
        .iter()
        .map(|d| d.get_name().to_string())
        .collect();

    // Calculate heuristic before moving input
    let initial_heuristic = generate_heuristic(&input, &[], &None);

    let initial = DecoderResult {
        text: vec![input],
        path: vec![],
    };

    // Thread-safe set to track visited states to prevent cycles
    let seen_strings = DashSet::new();
    let seen_results = DashSet::new(); // Track unique results

    // Thread-safe priority queue for open set
    let open_set: Mutex<BinaryHeap<AStarNode>> = Mutex::new(BinaryHeap::new());

    // Add initial node to open set
    open_set.lock().unwrap().push(AStarNode {
        state: initial,
        depth: 0,
        cost: 0.0,
        heuristic: initial_heuristic,
        total_cost: initial_heuristic, // f = g + h = 0 + h
        node_type: NodeType::Regular {
            next_decoder: None,
            untried_decoders: all_decoder_names.clone(),
        },
    });

    let curr_depth = Arc::new(AtomicU32::new(1));

    // Main A* loop
    while !open_set.lock().unwrap().is_empty() && !stop.load(AtomicOrdering::Relaxed) {
        let queue_len = open_set.lock().unwrap().len();
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth.load(AtomicOrdering::Relaxed),
            queue_len
        );

        // Extract a batch of nodes to process in parallel
        let batch_size = std::cmp::min(PARALLEL_BATCH_SIZE, queue_len);
        let batch = extract_batch(&open_set, batch_size);

        trace!("Processing batch of {} nodes in parallel", batch.len());

        // Process nodes in parallel
        let new_nodes: Vec<AStarNode> = batch
            .par_iter()
            .flat_map(|node| expand_node(node, &seen_strings, &stop, &all_decoder_names))
            .collect();

        // Collect result nodes and regular nodes separately
        let mut regular_nodes: Vec<AStarNode> = Vec::new();

        for node in new_nodes {
            if matches!(node.node_type, NodeType::Result) {
                // Check if we've already processed this result
                if let Some(text) = node.state.text.first() {
                    let result_hash = calculate_hash(text);
                    if seen_results.insert(result_hash) {
                        // New result - check human confirmation if needed
                        let should_include =
                            if let Some(confirmed_text) = get_human_confirmed_text() {
                                let normalized_result = text
                                    .to_ascii_lowercase()
                                    .chars()
                                    .filter(|c| !c.is_ascii_punctuation())
                                    .collect::<String>();
                                let normalized_confirmed = confirmed_text
                                    .to_ascii_lowercase()
                                    .chars()
                                    .filter(|c| !c.is_ascii_punctuation())
                                    .collect::<String>();
                                normalized_result == normalized_confirmed
                            } else {
                                true
                            };

                        if should_include {
                            // We need to store the node for later sorting
                            // For now, add to regular_nodes and mark as result
                            regular_nodes.push(node);
                        }
                    }
                }
            } else {
                regular_nodes.push(node);
            }
        }

        // Separate out result nodes from regular nodes for sorting
        let (mut collected_results, non_results): (Vec<_>, Vec<_>) = regular_nodes
            .into_iter()
            .partition(|n| matches!(n.node_type, NodeType::Result));

        // If we have result nodes, sort by path complexity and pick the best one
        if !collected_results.is_empty() {
            // Sort by cost (path complexity) - lower is better
            collected_results.sort_by(|a, b| {
                a.cost
                    .partial_cmp(&b.cost)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Get the best result (lowest cost)
            let best_result = collected_results.remove(0);

            trace!(
                "Found {} result nodes, best cost: {} (decoder path: {:?})",
                collected_results.len() + 1,
                best_result.cost,
                best_result
                    .state
                    .path
                    .iter()
                    .map(|p| p.decoder)
                    .collect::<Vec<_>>()
            );

            // Found a result node
            decoded_how_many_times(curr_depth.load(AtomicOrdering::Relaxed));

            // If in top_results mode, store the result in the WaitAthena storage
            if get_config().top_results {
                // Store the first text in the vector (there should only be one)
                if let Some(plaintext) = best_result.state.text.first() {
                    debug!(
                        "DEBUG: Processing result in top_results mode with plaintext: {}",
                        plaintext
                    );
                    // Get the last decoder used
                    let decoder_name = if let Some(last_decoder) = best_result.state.path.last() {
                        last_decoder.decoder.to_string()
                    } else {
                        "Unknown".to_string()
                    };

                    // Get the checker name from the last decoder
                    let checker_name = if let Some(last_decoder) = best_result.state.path.last() {
                        last_decoder.checker_name.to_string()
                    } else {
                        "Unknown".to_string()
                    };

                    // Only store results that have a valid checker name
                    if !checker_name.is_empty() && checker_name != "Unknown" {
                        log::trace!(
                            "Storing plaintext in WaitAthena storage: {} (decoder: {}, checker: {})",
                            plaintext,
                            decoder_name,
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

            // Send the result
            result_sender
                .send(Some(best_result.state.clone()))
                .expect("Should successfully send the result");

            // Only stop if not in top_results mode
            if !get_config().top_results {
                // Stop further iterations
                stop.store(true, AtomicOrdering::Relaxed);
                return;
            }
            // In top_results mode, continue searching
        }

        // Add remaining regular nodes to open set
        for node in non_results {
            // Track max depth (actual path length, not complexity score)
            curr_depth.fetch_max(node.state.path.len() as u32, AtomicOrdering::Relaxed);
            open_set.lock().unwrap().push(node);
        }
    }

    // If we get here, we've exhausted all possibilities without finding a solution
    if !stop.load(AtomicOrdering::Relaxed) {
        result_sender
            .send(None)
            .expect("Should successfully send the result");
    }
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
}
