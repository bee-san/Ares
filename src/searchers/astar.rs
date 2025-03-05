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

use crate::cli_pretty_printing;
use crate::cli_pretty_printing::decoded_how_many_times;
use crate::filtration_system::{
    get_decoder_tagged_decoders, get_non_decoder_tagged_decoders, MyResults,
};
use crossbeam::channel::Sender;

use log::{trace, warn};
use once_cell::sync::Lazy;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::CheckerTypes;
use crate::CrackResult;
use crate::DecoderResult;

/// Threshold for pruning the seen_strings HashSet to prevent excessive memory usage
const PRUNE_THRESHOLD: usize = 10000;

/// Initial pruning threshold for dynamic adjustment
const INITIAL_PRUNE_THRESHOLD: usize = PRUNE_THRESHOLD;

/// Maximum depth for search (used for dynamic threshold adjustment)
const MAX_DEPTH: u32 = 100;

/// Mapping between Cipher Identifier's cipher names and Ares decoder names
///
/// This static mapping allows us to translate between the cipher types identified by
/// Cipher Identifier and the corresponding decoders available in Ares.
///
/// For example:
/// - "fractionatedMorse" maps to "MorseCodeDecoder"
/// - "atbash" maps to "AtbashDecoder"
/// - "caesar" maps to "CaesarDecoder"
static CIPHER_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("fractionatedMorse", "morseCode");
    map.insert("atbash", "atbash");
    map.insert("caesar", "caesar");
    map.insert("railfence", "railfence");
    map.insert("rot47", "rot47");
    map.insert("a1z26", "a1z26");
    map.insert("simplesubstitution", "simplesubstitution");
    // Add more mappings as needed
    map
});

/// Track decoder success rates for adaptive learning
static DECODER_SUCCESS_RATES: Lazy<Mutex<HashMap<String, (usize, usize)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Update decoder statistics based on success or failure
///
/// # Arguments
///
/// * `decoder` - The name of the decoder
/// * `success` - Whether the decoder was successful
fn update_decoder_stats(decoder: &str, success: bool) {
    let mut stats = DECODER_SUCCESS_RATES.lock().unwrap();
    let (successes, total) = stats.entry(decoder.to_string()).or_insert((0, 0));

    if success {
        *successes += 1;
    }
    *total += 1;

    // TODO: Write this data to a file for persistence
}

/// Get the success rate of a decoder
///
/// # Arguments
///
/// * `decoder` - The name of the decoder
///
/// # Returns
///
/// * The success rate as a float between 0.0 and 1.0
fn get_decoder_success_rate(decoder: &str) -> f32 {
    let stats = DECODER_SUCCESS_RATES.lock().unwrap();
    if let Some((successes, total)) = stats.get(decoder) {
        if *total > 0 {
            return *successes as f32 / *total as f32;
        }
    }

    // Default for unknown decoders
    0.5
}

/// Get the cipher identification score for a text
///
/// # Arguments
///
/// * `text` - The text to analyze
///
/// # Returns
///
/// * A tuple containing the identified cipher and its score
fn get_cipher_identifier_score(text: &str) -> (String, f32) {
    let results = cipher_identifier::identify_cipher::identify_cipher(text, 5, None);

    for (cipher, score) in results {
        if let Some(_decoder) = CIPHER_MAPPING.get(cipher.as_str()) {
            return (cipher, (score / 10.0) as f32);
        }
    }

    // Default if no match
    let mut rng = rand::thread_rng();
    ("unknown".to_string(), rng.gen_range(0.5..1.0) as f32)
}

/// Check if a decoder and cipher form a common sequence
///
/// # Arguments
///
/// * `prev_decoder` - The name of the previous decoder
/// * `current_cipher` - The name of the current cipher
///
/// # Returns
///
/// * `true` if the sequence is common, `false` otherwise
fn is_common_sequence(prev_decoder: &str, current_cipher: &str) -> bool {
    // Define common sequences focusing on base decoders
    match (prev_decoder, current_cipher) {
        // Base64 commonly followed by other encodings
        ("Base64Decoder", "Base32Decoder") => true,
        ("Base64Decoder", "Base58Decoder") => true,
        ("Base64Decoder", "Base85Decoder") => true,
        ("Base64Decoder", "Base64Decoder") => true,

        // Base32 sequences
        ("Base32Decoder", "Base64Decoder") => true,
        ("Base32Decoder", "Base85Decoder") => true,
        ("Base32Decoder", "Base32Decoder") => true,

        // Base58 sequences
        ("Base58Decoder", "Base64Decoder") => true,
        ("Base58Decoder", "Base32Decoder") => true,
        ("Base58Decoder", "Base58Decoder") => true,

        // Base85 sequences
        ("Base85Decoder", "Base64Decoder") => true,
        ("Base85Decoder", "Base32Decoder") => true,
        ("Base85Decoder", "Base85Decoder") => true,
        // No match found
        _ => false,
    }
}

/// Calculate the quality of a string for pruning
///
/// # Arguments
///
/// * `s` - The string to evaluate
///
/// # Returns
///
/// * A quality score between 0.0 and 1.0
fn calculate_string_quality(s: &str) -> f32 {
    // Factors to consider:
    // 1. Length (not too short, not too long
    if s.len() < 3 {
        0.1
    } else if s.len() > 5000 {
        0.3
    } else {
        1.0 - (s.len() as f32 - 100.0).abs() / 900.0
    }
}

/// Calculate the ratio of non-printable characters in a string
/// Returns a value between 0.0 (all printable) and 1.0 (all non-printable)
fn calculate_non_printable_ratio(text: &str) -> f32 {
    if text.is_empty() {
        return 1.0;
    }

    let non_printable_count = text
        .chars()
        .filter(|&c| {
            // Same criteria as before for non-printable chars
            (c.is_control() && c != '\n' && c != '\r' && c != '\t')
                || !c.is_ascii_graphic() && !c.is_ascii_whitespace() && !c.is_ascii_punctuation()
        })
        .count();

    non_printable_count as f32 / text.len() as f32
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
    let initial_heuristic = generate_heuristic(&input, &[]);

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
        heuristic: initial_heuristic,
        total_cost: 0.0,
    });

    let mut curr_depth: u32 = 1;

    let mut prune_threshold = INITIAL_PRUNE_THRESHOLD;

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

        // Check stop signal again before processing node
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // First, execute all "decoder"-tagged decoders immediately
        let mut decoder_tagged_decoders = get_decoder_tagged_decoders(&current_node.state);

        // Prevent reciprocal decoders from being applied consecutively
        if let Some(last_decoder) = current_node.state.path.last() {
            if last_decoder.checker_description.contains("reciprocal") {
                let excluded_name = last_decoder.decoder;
                decoder_tagged_decoders
                    .components
                    .retain(|d| d.get_name() != excluded_name);
            }
        }

        if !decoder_tagged_decoders.components.is_empty() {
            trace!(
                "Found {} decoder-tagged decoders to execute immediately",
                decoder_tagged_decoders.components.len()
            );

            // Check stop signal before processing decoders
            if stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let athena_checker = Checker::<Athena>::new();
            let checker = CheckerTypes::CheckAthena(athena_checker);
            let decoder_results = decoder_tagged_decoders.run(&current_node.state.text[0], checker);

            // Process decoder results
            match decoder_results {
                MyResults::Break(res) => {
                    // Handle successful decoding
                    trace!("Found successful decoding with decoder-tagged decoder");
                    cli_pretty_printing::success(&format!(
                        "DEBUG: astar.rs - decoder-tagged decoder - res.success: {}",
                        res.success
                    ));

                    // Only exit if the result is truly successful (not rejected by human checker)
                    if res.success {
                        let mut decoders_used = current_node.state.path.clone();
                        let text = res.unencrypted_text.clone().unwrap_or_default();
                        decoders_used.push(res.clone());
                        let result_text = DecoderResult {
                            text,
                            path: decoders_used,
                        };

                        decoded_how_many_times(curr_depth);
                        cli_pretty_printing::success(&format!("DEBUG: astar.rs - decoder-tagged decoder - Sending successful result with {} decoders", result_text.path.len()));
                        result_sender
                            .send(Some(result_text))
                            .expect("Should successfully send the result");

                        // Stop further iterations
                        stop.store(true, std::sync::atomic::Ordering::Relaxed);
                        return;
                    } else {
                        // If human checker rejected, continue the search
                        trace!("Human checker rejected the result, continuing search");
                    }
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
                                // Add stats update for failed decoding
                                update_decoder_stats(r.decoder, false);
                                return false;
                            }

                            if seen_strings.insert(s.clone()) {
                                seen_count += 1;

                                // Prune the HashSet if it gets too large
                                if seen_count > prune_threshold {
                                    warn!(
                                        "Pruning seen_strings HashSet (size: {})",
                                        seen_strings.len()
                                    );

                                    // Calculate quality scores for all strings
                                    let mut quality_scores: Vec<(String, f32)> = seen_strings
                                        .iter()
                                        .map(|s| (s.clone(), calculate_string_quality(s)))
                                        .collect();

                                    // Sort by quality (higher is better)
                                    quality_scores.sort_by(|a, b| {
                                        b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
                                    });

                                    // Keep only the top 50% highest quality strings
                                    let keep_count = seen_strings.len() / 2;
                                    let strings_to_keep: HashSet<String> = quality_scores
                                        .into_iter()
                                        .take(keep_count)
                                        .map(|(s, _)| s)
                                        .collect();

                                    seen_strings = strings_to_keep;
                                    seen_count = seen_strings.len();

                                    // Adjust threshold based on search progress
                                    let progress_factor = curr_depth as f32 / MAX_DEPTH as f32;
                                    prune_threshold = INITIAL_PRUNE_THRESHOLD
                                        - (progress_factor * 5000.0) as usize;

                                    warn!(
                                        "Pruned to {} high-quality entries (new threshold: {})",
                                        seen_count, prune_threshold
                                    );
                                }

                                true
                            } else {
                                false
                            }
                        });

                        if text.is_empty() {
                            // Add stats update for failed decoding (no valid outputs)
                            update_decoder_stats(r.decoder, false);
                            continue;
                        }

                        decoders_used.push(r.clone());

                        // Create new node with updated cost and heuristic
                        let cost = current_node.cost + 1;
                        let heuristic = generate_heuristic(&text[0], &decoders_used);
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

                        // Update decoder stats - mark as successful since it produced valid output
                        update_decoder_stats(r.decoder, true);
                    }
                }
            }
        }

        // Then, process non-"decoder"-tagged decoders with heuristic prioritization
        let mut non_decoder_decoders = get_non_decoder_tagged_decoders(&current_node.state);

        // Prevent reciprocal decoders from being applied consecutively
        if let Some(last_decoder) = current_node.state.path.last() {
            if last_decoder.checker_description.contains("reciprocal") {
                let excluded_name = last_decoder.decoder;
                non_decoder_decoders
                    .components
                    .retain(|d| d.get_name() != excluded_name);
            }
        }

        if !non_decoder_decoders.components.is_empty() {
            trace!(
                "Processing {} non-decoder-tagged decoders",
                non_decoder_decoders.components.len()
            );

            // Check stop signal before processing decoders
            if stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let athena_checker = Checker::<Athena>::new();
            let checker = CheckerTypes::CheckAthena(athena_checker);
            let decoder_results = non_decoder_decoders.run(&current_node.state.text[0], checker);

            // Process decoder results
            match decoder_results {
                MyResults::Break(res) => {
                    // Handle successful decoding
                    trace!("Found successful decoding with non-decoder-tagged decoder");
                    cli_pretty_printing::success(&format!(
                        "DEBUG: astar.rs - non-decoder-tagged decoder - res.success: {}",
                        res.success
                    ));

                    // Only exit if the result is truly successful (not rejected by human checker)
                    if res.success {
                        let mut decoders_used = current_node.state.path.clone();
                        let text = res.unencrypted_text.clone().unwrap_or_default();
                        decoders_used.push(res.clone());
                        let result_text = DecoderResult {
                            text,
                            path: decoders_used,
                        };

                        decoded_how_many_times(curr_depth);
                        cli_pretty_printing::success(&format!("DEBUG: astar.rs - non-decoder-tagged decoder - Sending successful result with {} decoders", result_text.path.len()));
                        result_sender
                            .send(Some(result_text))
                            .expect("Should successfully send the result");

                        // Stop further iterations
                        stop.store(true, std::sync::atomic::Ordering::Relaxed);
                        return;
                    } else {
                        // If human checker rejected, continue the search
                        trace!("Human checker rejected the result, continuing search");
                    }
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
                                // Add stats update for failed decoding
                                update_decoder_stats(r.decoder, false);
                                return false;
                            }

                            if seen_strings.insert(s.clone()) {
                                seen_count += 1;

                                // Prune the HashSet if it gets too large
                                if seen_count > prune_threshold {
                                    warn!(
                                        "Pruning seen_strings HashSet (size: {})",
                                        seen_strings.len()
                                    );

                                    // Calculate quality scores for all strings
                                    let mut quality_scores: Vec<(String, f32)> = seen_strings
                                        .iter()
                                        .map(|s| (s.clone(), calculate_string_quality(s)))
                                        .collect();

                                    // Sort by quality (higher is better)
                                    quality_scores.sort_by(|a, b| {
                                        b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
                                    });

                                    // Keep only the top 50% highest quality strings
                                    let keep_count = seen_strings.len() / 2;
                                    let strings_to_keep: HashSet<String> = quality_scores
                                        .into_iter()
                                        .take(keep_count)
                                        .map(|(s, _)| s)
                                        .collect();

                                    seen_strings = strings_to_keep;
                                    seen_count = seen_strings.len();

                                    // Adjust threshold based on search progress
                                    let progress_factor = curr_depth as f32 / MAX_DEPTH as f32;
                                    prune_threshold = INITIAL_PRUNE_THRESHOLD
                                        - (progress_factor * 5000.0) as usize;

                                    warn!(
                                        "Pruned to {} high-quality entries (new threshold: {})",
                                        seen_count, prune_threshold
                                    );
                                }

                                true
                            } else {
                                false
                            }
                        });

                        if text.is_empty() {
                            // Add stats update for failed decoding (no valid outputs)
                            update_decoder_stats(r.decoder, false);
                            continue;
                        }

                        decoders_used.push(r.clone());

                        // Create new node with updated cost and heuristic
                        let cost = current_node.cost + 1;
                        let heuristic = generate_heuristic(&text[0], &decoders_used);
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

                        // Update decoder stats - mark as successful since it produced valid output
                        update_decoder_stats(r.decoder, true);
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

/// Get the popularity rating of a decoder by its name
/// Popularity is a value between 0.0 and 1.0 where higher values indicate more common decoders
fn get_decoder_popularity(decoder: &str) -> f32 {
    // This is a static mapping of decoder names to their popularity
    // In a more sophisticated implementation, this could be loaded from a configuration file
    // or stored in a database
    match decoder {
        "Base64" => 1.0,
        "Hexadecimal" => 1.0,
        "Binary" => 1.0,
        "rot13" => 1.0,
        "rot47" => 1.0,
        "Base32" => 0.8,
        "Vigenere" => 0.8,
        "Base58" => 0.7,
        "Base85" => 0.5,
        "simplesubstitution" => 0.5,
        "Base91" => 0.3,
        "Citrix Ctx1" => 0.1,
        // Default for unknown decoders
        _ => 0.5,
    }
}

/// Generate a heuristic value for A* search prioritization
///
/// The heuristic estimates how close a state is to being plaintext.
/// A lower value indicates a more promising state. This implementation uses
/// Cipher Identifier to identify the most likely ciphers for the given text.
///
/// # Parameters
///
/// * `text` - The text to analyze for cipher identification
/// * `path` - The path of decoders used to reach the current state
///
/// # Returns
/// A float value representing the heuristic cost (lower is better)
fn generate_heuristic(text: &str, path: &[CrackResult]) -> f32 {
    let (cipher, base_score) = get_cipher_identifier_score(text);
    let mut final_score = base_score;

    if let Some(last_result) = path.last() {
        // Penalize uncommon sequences instead of rewarding common ones
        if !is_common_sequence(last_result.decoder, &cipher) {
            final_score *= 1.25; // 25% penalty for uncommon sequences
        }

        // Penalize low success rates instead of rewarding high ones
        let success_rate = get_decoder_success_rate(last_result.decoder);
        final_score *= 1.0 + (1.0 - success_rate); // Penalty scales with failure rate

        // Penalize decoders with low popularity
        let popularity = get_decoder_popularity(last_result.decoder);
        // Apply a significant penalty for unpopular decoders
        // The penalty is inversely proportional to the popularity
        final_score *= 1.0 + (2.0 * (1.0 - popularity)); // Penalty scales with unpopularity
    }

    // Penalize low quality strings
    final_score *= 1.0 + (1.0 - calculate_string_quality(text));

    // Keep the non-printable penalty as is since it's already using a penalty approach
    let non_printable_ratio = calculate_non_printable_ratio(text);
    if non_printable_ratio > 0.0 {
        final_score *= 1.0 + (non_printable_ratio * 100.0).exp();
    }

    final_score
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
    text.len() <= 2 // Only check length now, non-printable chars handled by heuristic
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Decoder;
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

    #[test]
    fn test_generate_heuristic() {
        // Test with normal text (should have relatively low score)
        let normal_h = generate_heuristic("Hello World", &[]);

        // Test with suspicious text (should have higher score)
        let suspicious_h = generate_heuristic("H\u{0}ll\u{1} W\u{2}rld", &[]);

        // Test with all non-printable (should have highest score)
        let nonprint_h = generate_heuristic("\u{0}\u{1}\u{2}", &[]);

        // Verify that penalties create appropriate ordering
        assert!(normal_h < suspicious_h);
        assert!(suspicious_h < nonprint_h);

        // Verify base case isn't negative
        assert!(normal_h >= 0.0);
    }

    #[test]
    fn test_calculate_non_printable_ratio() {
        // Test normal text
        assert_eq!(calculate_non_printable_ratio("Hello World"), 0.0);
        assert_eq!(calculate_non_printable_ratio("123!@#\n\t"), 0.0);

        // Test mixed content
        let mixed = "Hello\u{0}World\u{1}".to_string(); // 2 non-printable in 12 chars
        assert!((calculate_non_printable_ratio(&mixed) - 0.1666).abs() < 0.001);

        // Test all non-printable
        assert_eq!(calculate_non_printable_ratio("\u{0}\u{1}\u{2}"), 1.0);

        // Test empty string
        assert_eq!(calculate_non_printable_ratio(""), 1.0);
    }

    #[test]
    fn test_heuristic_with_non_printable() {
        // Test normal text
        let normal = generate_heuristic("Hello World", &[]);

        // Test text with some non-printable chars
        let with_non_printable = generate_heuristic("Hello\u{0}World", &[]);

        // Test text with all non-printable chars
        let all_non_printable = generate_heuristic("\u{0}\u{1}\u{2}", &[]);

        // Verify that more non-printable chars result in higher (worse) scores
        assert!(normal < with_non_printable);
        assert!(with_non_printable < all_non_printable);
        assert!(all_non_printable > 100.0); // Should be very high for all non-printable
    }

    #[test]
    fn test_popularity_affects_heuristic() {
        // Create two identical paths but with different decoders
        let popular_decoder = "Base64"; // Popularity 1.0
        let unpopular_decoder = "Citrix Ctx1"; // Popularity 0.1

        // Create CrackResults with different decoders
        let mut popular_result = CrackResult::new(&Decoder::default(), "test".to_string());
        popular_result.decoder = popular_decoder;

        let mut unpopular_result = CrackResult::new(&Decoder::default(), "test".to_string());
        unpopular_result.decoder = unpopular_decoder;

        // Generate heuristics for both paths
        let popular_heuristic = generate_heuristic("test", &[popular_result]);
        let unpopular_heuristic = generate_heuristic("test", &[unpopular_result]);

        // The unpopular decoder should have a higher heuristic (worse score)
        assert!(
            unpopular_heuristic > popular_heuristic,
            "Unpopular decoder should have a higher (worse) heuristic score. \
            Popular: {}, Unpopular: {}",
            popular_heuristic,
            unpopular_heuristic
        );

        // The difference should be significant
        assert!(
            unpopular_heuristic >= popular_heuristic * 1.5,
            "Unpopular decoder should have a significantly higher heuristic. \
            Popular: {}, Unpopular: {}",
            popular_heuristic,
            unpopular_heuristic
        );
    }
}
