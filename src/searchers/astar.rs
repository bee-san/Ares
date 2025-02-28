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
//! but this could be improved with more sophisticated cipher identification.

use crate::cli_pretty_printing::decoded_how_many_times;
use crate::filtration_system::{
    get_decoder_tagged_decoders, get_non_decoder_tagged_decoders, MyResults,
};
use crossbeam::channel::Sender;

use log::{trace, warn};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::CheckerTypes;
use crate::DecoderResult;

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

/// Threshold for pruning the seen_strings HashSet to prevent excessive memory usage
const PRUNE_THRESHOLD: usize = 10000;

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
    let initial = DecoderResult {
        text: vec![input],
        path: vec![],
    };

    // Set to track visited states to prevent cycles
    let mut seen_strings = HashSet::new();
    let mut seen_count = 0;

    // Priority queue for open set
    let mut open_set = BinaryHeap::new();

    // Add initial node to open set
    open_set.push(AStarNode {
        state: initial,
        cost: 0,
        heuristic: 0.0,
        total_cost: 0.0,
    });

    let mut curr_depth: u32 = 1;

    // Main A* loop
    while !open_set.is_empty() && !stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth,
            open_set.len()
        );

        // Get the node with the lowest f value
        let current_node = open_set.pop().unwrap();

        trace!(
            "Processing node with cost {}, heuristic {}, total cost {}",
            current_node.cost,
            current_node.heuristic,
            current_node.total_cost
        );

        // First, execute all "decoder"-tagged decoders immediately
        let mut decoder_tagged_decoders = get_decoder_tagged_decoders(&current_node.state);

        // Prevent reciprocal decoders from being applied consecutively
        if let Some(last_decoder) = current_node.state.path.last() {
            if last_decoder.decoder_tags.contains(&"reciprocal".to_string()) {
                let excluded_name = last_decoder.decoder_name.clone();
                decoder_tagged_decoders.components.retain(|d| d.get_name() != excluded_name);
            }
        }

        if !decoder_tagged_decoders.components.is_empty() {
            trace!(
                "Found {} decoder-tagged decoders to execute immediately",
                decoder_tagged_decoders.components.len()
            );

            let athena_checker = Checker::<Athena>::new();
            let checker = CheckerTypes::CheckAthena(athena_checker);
            let decoder_results = decoder_tagged_decoders.run(&current_node.state.text[0], checker);

            // Process decoder results
            match decoder_results {
                MyResults::Break(res) => {
                    // Handle successful decoding
                    trace!("Found successful decoding with decoder-tagged decoder");
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    decoders_used.push(res);
                    let result_text = DecoderResult {
                        text,
                        path: decoders_used,
                    };

                    decoded_how_many_times(curr_depth);
                    result_sender
                        .send(Some(result_text))
                        .expect("Should successfully send the result");

                    // Stop further iterations
                    stop.store(true, std::sync::atomic::Ordering::Relaxed);
                    return;
                }
                MyResults::Continue(results_vec) => {
                    // Process results and add to open set
                    trace!(
                        "Processing {} results from decoder-tagged decoders",
                        results_vec.len()
                    );

                    for mut r in results_vec {
                        let mut decoders_used = current_node.state.path.clone();
                        let mut text = r.unencrypted_text.take().unwrap_or_default();

                        // Filter out strings that can't be decoded or have been seen before
                        text.retain(|s| {
                            if check_if_string_cant_be_decoded(s) {
                                return false;
                            }

                            if seen_strings.insert(s.clone()) {
                                seen_count += 1;

                                // Prune the HashSet if it gets too large
                                if seen_count > PRUNE_THRESHOLD {
                                    warn!(
                                        "Pruning seen_strings HashSet (size: {})",
                                        seen_strings.len()
                                    );

                                    // Keep strings that are more likely to lead to a solution
                                    // This heuristic keeps shorter strings as they're often more valuable
                                    let before_size = seen_strings.len();
                                    seen_strings.retain(|s| s.len() < 100);
                                    let after_size = seen_strings.len();

                                    warn!(
                                        "Pruned {} entries from seen_strings HashSet",
                                        before_size - after_size
                                    );
                                    seen_count = after_size;
                                }

                                true
                            } else {
                                false
                            }
                        });

                        if text.is_empty() {
                            continue;
                        }

                        decoders_used.push(r);

                        // Create new node with updated cost and heuristic
                        let cost = current_node.cost + 1;
                        let heuristic = generate_heuristic();
                        let total_cost = cost as f32 + heuristic;

                        let new_node = AStarNode {
                            state: DecoderResult {
                                text,
                                path: decoders_used,
                            },
                            cost,
                            heuristic,
                            total_cost,
                        };

                        // Add to open set
                        open_set.push(new_node);
                    }
                }
            }
        }

        // Then, process non-"decoder"-tagged decoders with heuristic prioritization
        let mut non_decoder_decoders = get_non_decoder_tagged_decoders(&current_node.state);

        // Prevent reciprocal decoders from being applied consecutively
        if let Some(last_decoder) = current_node.state.path.last() {
            if last_decoder.decoder_tags.contains(&"reciprocal".to_string()) {
                let excluded_name = last_decoder.decoder_name.clone();
                non_decoder_decoders.components.retain(|d| d.get_name() != excluded_name);
            }
        }

        if !non_decoder_decoders.components.is_empty() {
            trace!(
                "Processing {} non-decoder-tagged decoders",
                non_decoder_decoders.components.len()
            );

            let athena_checker = Checker::<Athena>::new();
            let checker = CheckerTypes::CheckAthena(athena_checker);
            let decoder_results = non_decoder_decoders.run(&current_node.state.text[0], checker);

            // Process decoder results
            match decoder_results {
                MyResults::Break(res) => {
                    // Handle successful decoding
                    trace!("Found successful decoding with non-decoder-tagged decoder");
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    decoders_used.push(res);
                    let result_text = DecoderResult {
                        text,
                        path: decoders_used,
                    };

                    decoded_how_many_times(curr_depth);
                    result_sender
                        .send(Some(result_text))
                        .expect("Should successfully send the result");

                    // Stop further iterations
                    stop.store(true, std::sync::atomic::Ordering::Relaxed);
                    return;
                }
                MyResults::Continue(results_vec) => {
                    // Process results and add to open set with heuristic prioritization
                    trace!(
                        "Processing {} results from non-decoder-tagged decoders",
                        results_vec.len()
                    );

                    for mut r in results_vec {
                        let mut decoders_used = current_node.state.path.clone();
                        let mut text = r.unencrypted_text.take().unwrap_or_default();

                        // Filter out strings that can't be decoded or have been seen before
                        text.retain(|s| {
                            if check_if_string_cant_be_decoded(s) {
                                return false;
                            }

                            if seen_strings.insert(s.clone()) {
                                seen_count += 1;

                                // Prune the HashSet if it gets too large
                                if seen_count > PRUNE_THRESHOLD {
                                    warn!(
                                        "Pruning seen_strings HashSet (size: {})",
                                        seen_strings.len()
                                    );

                                    // Keep strings that are more likely to lead to a solution
                                    // This heuristic keeps shorter strings as they're often more valuable
                                    let before_size = seen_strings.len();
                                    seen_strings.retain(|s| s.len() < 100);
                                    let after_size = seen_strings.len();

                                    warn!(
                                        "Pruned {} entries from seen_strings HashSet",
                                        before_size - after_size
                                    );
                                    seen_count = after_size;
                                }

                                true
                            } else {
                                false
                            }
                        });

                        if text.is_empty() {
                            continue;
                        }

                        decoders_used.push(r);

                        // Create new node with updated cost and heuristic
                        let cost = current_node.cost + 1;
                        let heuristic = generate_heuristic();
                        let total_cost = cost as f32 + heuristic;

                        let new_node = AStarNode {
                            state: DecoderResult {
                                text,
                                path: decoders_used,
                            },
                            cost,
                            heuristic,
                            total_cost,
                        };

                        // Add to open set
                        open_set.push(new_node);
                    }
                }
            }
        }

        curr_depth += 1;
    }

    trace!("A* search completed without finding a solution");
    result_sender.try_send(None).ok();
}

/// Generate a heuristic value for A* search prioritization
///
/// The heuristic estimates how close a state is to being plaintext.
/// A lower value indicates a more promising state.
///
/// ## Current Implementation
///
/// Currently returns a constant value of 1.0 as a placeholder.
///
/// ## Future Improvements
///
/// TODO: Use cipher identifier from SkeletalDemise to generate more
/// accurate heuristics based on:
/// - Character frequency analysis
/// - N-gram statistics
/// - Entropy measurements
/// - Language detection scores
///
/// A more sophisticated heuristic would significantly improve search efficiency.
fn generate_heuristic() -> f32 {
    1.0
}

/// Determines if a string is too short to be meaningfully decoded
///
/// ## Decision Criteria
///
/// A string is considered undecodeble if:
/// - It has 2 or fewer characters
///
/// ## Rationale
///
/// 1. The gibberish_or_not library requires at least 3 characters to work effectively
/// 2. LemmeKnow and other pattern matchers perform poorly on very short strings
/// 3. Most encoding schemes produce output of at least 3 characters
///
/// Filtering out these strings early saves computational resources and
/// prevents the search from exploring unproductive paths.
fn check_if_string_cant_be_decoded(text: &str) -> bool {
    text.len() <= 2
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
