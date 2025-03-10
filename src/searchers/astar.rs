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

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam::channel::Sender;
use log::{debug, trace};

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::CheckerTypes;
use crate::cli_pretty_printing;
use crate::config::get_config;
use crate::decoders::interface::Crack;
use crate::filtration_system::{get_all_decoders, get_non_decoder_tagged_decoders, Decoders, MyResults};
use crate::searchers::helper_functions;
use crate::storage::wait_athena_storage;
use crate::DecoderResult;
use crate::cli_pretty_printing::decoded_how_many_times;

/// Threshold for pruning the seen_strings HashSet to prevent excessive memory usage
const PRUNE_THRESHOLD: usize = 100000;

/// Initial pruning threshold for dynamic adjustment
const INITIAL_PRUNE_THRESHOLD: usize = PRUNE_THRESHOLD;

/// Maximum depth for search (used for dynamic threshold adjustment)
const MAX_DEPTH: u32 = 100;

/// Depth penalty factor for A* search
/// Higher values will more strongly discourage deep paths
const DEPTH_PENALTY_FACTOR: f32 = 0.15;

/// Add decoder type weights
const COMMON_DECODER_WEIGHT: f32 = 0.7;
const ESOTERIC_DECODER_WEIGHT: f32 = 1.3;

/// Enum to differentiate between different types of decoders
#[derive(Debug, Clone, Copy)]
enum DecoderType {
    /// Standard decoder (previously "non-decoder-tagged")
    Standard,
    /// Tagged decoder (previously "decoder-tagged")
    Tagged,
}

/// Node in the A* search tree
/// Each node represents a state in the search space
/// with associated cost and heuristic values
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
    
    /// The next decoder to try. Can be None for the initial node.
    next_decoder: Option<Box<dyn Crack + Sync>>,
}

// Debug implementation for AStarNode that skips the next_decoder field
impl std::fmt::Debug for AStarNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AStarNode")
            .field("state", &self.state)
            .field("cost", &self.cost)
            .field("heuristic", &self.heuristic)
            .field("total_cost", &self.total_cost)
            .field("next_decoder", &"<dyn Crack + Sync>")
            .finish()
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

/// Create a new node with updated cost and heuristic
///
/// # Arguments
///
/// * `current_node` - The current node being expanded
/// * `result` - The result of applying a decoder
/// * `decoder_type` - The type of decoder used
fn create_node(
    current_node: &AStarNode,
    result: DecoderResult,
    decoder_type: DecoderType,
) -> AStarNode {
    let cost = current_node.cost + 1;
    let base_heuristic = helper_functions::generate_heuristic(&result.text, &result.path, &None);
    
    // Apply decoder-type specific weighting
    let weighted_heuristic = match decoder_type {
        DecoderType::Tagged => base_heuristic * COMMON_DECODER_WEIGHT,
        DecoderType::Standard => base_heuristic * ESOTERIC_DECODER_WEIGHT,
    };
    
    // Add historical success rate influence
    let adjusted_heuristic = if let Some(last_decoder) = result.path.last() {
        let success_rate = helper_functions::get_decoder_success_rate(last_decoder.decoder);
        weighted_heuristic * (1.0 + (1.0 - success_rate) * 0.5)
    } else {
        weighted_heuristic
    };
    
    // Calculate total cost (f = g + h)
    let total_cost = cost as f32 + adjusted_heuristic;
    
    AStarNode {
        state: result,
        cost,
        heuristic: adjusted_heuristic,
        total_cost,
        next_decoder: None,
    }
}

/// A* search algorithm implementation for finding the optimal decoding path
/// This function implements the A* search algorithm to find the optimal path
/// of decoders to apply to the input text to reach plaintext
///
/// # Arguments
/// * `input` - The input text to decode
/// * `result_sender` - Channel to send the result back to the caller
/// * `stop` - Atomic boolean to signal the search to stop
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    let initial = DecoderResult {
        text: input,
        path: vec![],
    };

    // Create initial node with no next_decoder (start with any decoder)
    let initial_node = AStarNode {
        state: initial,
        cost: 0,
        heuristic: 0.0,
        total_cost: 0.0,
        next_decoder: None,
    };

    // Set to track visited states to prevent cycles
    let mut seen_strings = HashSet::new();
    let mut seen_count = 0;

    // Priority queue for open set
    let mut open_set = BinaryHeap::new();

    // Add initial node to open set
    open_set.push(initial_node);

    let mut curr_depth: u32 = 1;

    let mut prune_threshold = INITIAL_PRUNE_THRESHOLD;

    // Main A* loop
    while !open_set.is_empty() && !stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth,
            open_set.len()
        );

        // Get the node with the lowest f value (total cost)
        let current_node = open_set.pop().unwrap();

        trace!(
            "Processing node with cost {}, heuristic {}, total cost {}",
            current_node.cost,
            current_node.heuristic,
            current_node.total_cost
        );

        // Check stop signal again before processing node
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // If there is a next_decoder, use it. Otherwise, get all decoders.
        let decoders = match &current_node.next_decoder {
            Some(decoder) => {
                // We used the decoder, so update its stats
                helper_functions::update_decoder_stats(decoder.get_name(), true);
                
                // Create a new Decoders struct with just this decoder
                // We need to get a new instance of the decoder since we can't clone the trait object
                let decoder_name = decoder.get_name();
                let all_decoders = get_all_decoders();
                let matching_decoders = all_decoders.components.into_iter()
                    .filter(|d| d.get_name() == decoder_name)
                    .collect();
                
                Decoders { components: matching_decoders }
            }
            None => get_all_decoders(),
        };

        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        let decoder_results = decoders.run(&current_node.state.text, checker);

        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                trace!("Found successful decoding with specific decoder");
                cli_pretty_printing::success(&format!(
                    "DEBUG: astar.rs - Decoder {} succeeded, short-circuiting",
                    res.decoder
                ));

                // Only exit if the result is truly successful (not rejected by human checker)
                if res.success {
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_else(|| vec!["".to_string()])[0].clone();
                    decoders_used.push(res.clone());
                    let result_text = DecoderResult {
                        text,
                        path: decoders_used,
                    };

                    decoded_how_many_times(curr_depth);
                    cli_pretty_printing::success(&format!("DEBUG: astar.rs -  Sending successful result with {} decoders", result_text.path.len()));

                    // If in top_results mode, store the result in the WaitAthena storage
                    if get_config().top_results {
                        // Get the last decoder used
                        let decoder_name =
                            if let Some(last_decoder) = result_text.path.last() {
                                last_decoder.decoder.to_string()
                            } else {
                                "Unknown".to_string()
                            };

                        // Get the checker name from the last decoder
                        let checker_name =
                            if let Some(last_decoder) = result_text.path.last() {
                                last_decoder.checker_name.to_string()
                            } else {
                                "Unknown".to_string()
                            };

                        // Only store results that have a valid checker name
                        if !checker_name.is_empty() && checker_name != "Unknown" {
                            trace!(
                                "Storing plaintext in WaitAthena storage: {} (decoder: {}, checker: {})",
                                result_text.text,
                                decoder_name,
                                checker_name
                            );
                            wait_athena_storage::add_plaintext_result(
                                result_text.text.clone(),
                                format!("Decoded successfully at depth {}", curr_depth),
                                checker_name,
                                decoder_name,
                            );
                        }
                    }

                    result_sender
                        .send(Some(result_text))
                        .expect("Should successfully send the result");

                    // Only stop if not in top_results mode
                    if !get_config().top_results {
                        // Stop further iterations
                        stop.store(true, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                    // In top_results mode, continue searching
                } else {
                    // If human checker rejected, continue the search
                    trace!("Human checker rejected the result, continuing search");
                }
            }
            MyResults::Continue(results_vec) => {
                // Process results and add to open set with heuristic prioritization
                trace!(
                    "Processing {} results from decoders",
                    results_vec.len()
                );

                for r in results_vec {
                    let decoders_used = current_node.state.path.clone();
                    let text = r.unencrypted_text.clone().unwrap_or_else(|| vec!["".to_string()])[0].clone();

                    // Make sure this string passes the checks
                    if helper_functions::check_if_string_cant_be_decoded(&text) {
                        // Add stats update for failed decoding
                        helper_functions::update_decoder_stats(r.decoder, false);
                        continue;
                    }

                    if seen_strings.insert(text.clone()) {
                        seen_count += 1;

                        // Prune the HashSet if it gets too large
                        if seen_count > prune_threshold {
                            debug!(
                                "Pruning seen_strings HashSet (size: {})",
                                seen_strings.len()
                            );
                            seen_strings.retain(|s| s.len() < 100);
                            debug!("Pruned to {} entries", seen_strings.len());
                            prune_threshold = INITIAL_PRUNE_THRESHOLD
                                - (curr_depth * 1000) as usize; // Decrease threshold by 1000 per depth
                            debug!("Setting new prune threshold to {}", prune_threshold);
                        }
                    } else {
                        // This string has been seen before, don't process it
                        continue;
                    }

                    // Remove decoded texts from crack result
                    // Because all it holds is unencrypted text
                    let mut r_clone = r.clone();
                    r_clone.unencrypted_text = None;

                    let mut new_decoders_used = decoders_used.clone();
                    new_decoders_used.push(r_clone);

                    // For each decoder, create a new node with that decoder as the next_decoder
                    let non_decoder_decoders = get_non_decoder_tagged_decoders(&DecoderResult {
                        text: text.clone(),
                        path: new_decoders_used.clone(),
                    });

                    for decoder in non_decoder_decoders.components {
                        // We can't clone the trait object directly, so we need to use a different approach
                        // Get the decoder name and use it to calculate the heuristic
                        let decoder_name = decoder.get_name();
                        let decoder_tags = decoder.get_tags().clone();
                        
                        // Create a custom heuristic calculation without needing to clone the decoder
                        let mut base_score = 0.0;
                        
                        if decoder_tags.contains(&"cipher") {
                            let (_, score) = helper_functions::get_cipher_identifier_score(&text);
                            base_score += 1.0 - (score / 100.0) as f32;
                        } else {
                            let success_rate = helper_functions::get_decoder_success_rate(decoder_name);
                            base_score += 1.0 - success_rate;
                        }
                        
                        // Add depth penalty
                        base_score += (0.1 * new_decoders_used.len() as f32).powi(2);
                        
                        // Add penalty for uncommon sequences
                        if let Some(previous_decoder) = new_decoders_used.last() {
                            if !helper_functions::is_common_sequence(previous_decoder.decoder, decoder_name) {
                                base_score += 0.25;
                            }
                        }
                        
                        let heuristic_value = base_score;
                        
                        let new_node = AStarNode {
                            state: DecoderResult {
                                text: text.clone(),
                                path: new_decoders_used.clone(),
                            },
                            cost: current_node.cost + 1,
                            heuristic: heuristic_value,
                            total_cost: current_node.cost as f32 + heuristic_value,
                            next_decoder: Some(decoder),
                        };

                        open_set.push(new_node);
                    }
                }
            }
        }

        curr_depth += 1;
    }

    // Check if we were stopped or if we genuinely couldn't find a solution
    if stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!("A* search stopped by external signal");
    } else {
        trace!("A* search completed without finding a solution");
        result_sender.try_send(None).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::bounded;

    #[test]
    fn astar_handles_empty_input() {
        // Test that A* handles empty input gracefully
        let (tx, rx) = bounded::<Option<DecoderResult>>(1);
        let stopper = Arc::new(AtomicBool::new(false));
        astar("".into(), tx, stopper);
        let result = rx.recv().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn astar_prevents_cycles() {
        // Test that the algorithm doesn't revisit states
        // We'll use a string that could potentially cause cycles
        let (tx, rx) = bounded::<Option<DecoderResult>>(1);
        let stopper = Arc::new(AtomicBool::new(false));

        // This is a base64 encoding of "hello" that when decoded and re-encoded
        // could potentially cause cycles if not handled properly
        astar("aGVsbG8=".into(), tx, stopper);

        // The algorithm should complete without hanging
        let result = rx.recv().unwrap();
        assert!(result.is_some());
    }
}
