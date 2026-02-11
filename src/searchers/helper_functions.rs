//! # Helper Functions for A* Search
//!
//! This module contains helper functions used by the A* search algorithm
//! for decoding encrypted or encoded text.
//!
//! ## Heuristic Design (Occam's Razor)
//!
//! The heuristic is designed with Occam's Razor in mind: simpler explanations
//! (shorter decoding paths) are preferred. Key principles:
//!
//! 1. **Encoders vs Ciphers**: Repeated encoders (e.g., base64 × 5) are common
//!    and cheap. Ciphers are rare and expensive.
//! 2. **Path Complexity**: Rather than raw depth, we calculate "conceptual complexity"
//!    where repeated same-encoder applications are discounted.
//! 3. **Entropy**: Lower entropy text is more likely to be plaintext.

use crate::decoders::interface::Crack;
use crate::decoders::DECODER_MAP;
use crate::CrackResult;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

// ── A* Heuristic & Path Complexity Constants ──────────────────────────

/// Base cost for applying a different encoder (e.g. Base64 → Hex).
const ENCODER_BASE_COST: f32 = 0.7;

/// Cost for applying the same encoder again (e.g. base64 → base64).
/// Kept very low because nested same-encoding is common in CTFs.
const REPEATED_ENCODER_COST: f32 = 0.2;

/// Base cost multiplier for cipher decoders.
/// The actual cost is `CIPHER_BASE_COST * cipher_position * popularity_multiplier`.
const CIPHER_BASE_COST: f32 = 2.0;

/// Upper bound of the popularity multiplier range.
/// Multiplier = `POPULARITY_MULTIPLIER_CEILING - popularity`, giving range 1.0–2.0.
const POPULARITY_MULTIPLIER_CEILING: f32 = 2.0;

/// Default popularity score for decoders not found in `DECODER_MAP`.
const DEFAULT_POPULARITY: f32 = 0.5;

/// Weight applied to `(1 - success_rate)` as a penalty in decoder ranking.
const SUCCESS_PENALTY_WEIGHT: f32 = 0.3;

/// Entropy threshold above which encoders receive a bonus in ranking.
const HIGH_ENTROPY_THRESHOLD: f32 = 0.6;

/// Bonus (negative cost) given to encoders when input entropy exceeds the threshold.
const ENCODER_ENTROPY_BONUS: f32 = -0.1;

/// Penalty added to heuristic when the next decoder is unknown.
const UNKNOWN_DECODER_PENALTY: f32 = 0.25;

/// Default success rate returned for decoders with no recorded history.
const DEFAULT_SUCCESS_RATE: f32 = 0.5;

/// Weight of Shannon entropy in the heuristic score (multiplied by normalized entropy).
const ENTROPY_WEIGHT: f32 = 2.0;

/// Weight of decoder success rate in the heuristic score.
const DECODER_SUCCESS_WEIGHT: f32 = 0.5;

/// Weight of string quality in the heuristic score.
const QUALITY_WEIGHT: f32 = 0.5;

/// Maximum entropy normalization factor (~log₂(95) ≈ 6.57 for ASCII printable).
const ENTROPY_NORMALIZATION: f32 = 6.6;

// ── String Quality & Pruning Constants ────────────────────────────────

/// Maximum character count (exclusive) below which a string is considered too
/// short to decode. The gibberish_or_not library needs at least 3 characters.
const MIN_DECODABLE_LENGTH: usize = 2;

/// Maximum non-printable character ratio before a string is rejected for decoding.
const MAX_NON_PRINTABLE_RATIO: f32 = 0.3;

/// Non-printable ratio threshold above which `calculate_string_quality` returns 0.
const QUALITY_NON_PRINTABLE_THRESHOLD: f32 = 0.5;

/// Minimum quality score threshold; strings below this are considered undecodable.
const MIN_QUALITY_THRESHOLD: f32 = 0.2;

/// Minimum string length for a non-minimal quality score.
const MIN_LENGTH_FOR_QUALITY: usize = 3;

/// String length above which quality is capped at a reduced value.
const MAX_OPTIMAL_LENGTH: usize = 5000;

/// Ideal string length used as the center point for quality calculation.
const QUALITY_IDEAL_LENGTH: f32 = 100.0;

/// Divisor used to normalize the distance from `QUALITY_IDEAL_LENGTH`.
const QUALITY_LENGTH_DIVISOR: f32 = 900.0;

/// Quality score returned for very short strings (below `MIN_LENGTH_FOR_QUALITY`).
const SHORT_STRING_QUALITY: f32 = 0.1;

/// Quality score returned for very long strings (above `MAX_OPTIMAL_LENGTH`).
const LONG_STRING_QUALITY: f32 = 0.3;

/// Track decoder success rates for adaptive learning.
///
/// Uses `RwLock` instead of `Mutex` because reads (from `rank_decoders` on the
/// A* hot path) vastly outnumber writes, and `RwLock` allows concurrent reads.
pub(crate) static DECODER_SUCCESS_RATES: Lazy<RwLock<HashMap<String, (usize, usize)>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Represents a decoder with its estimated cost for ranking purposes
#[derive(Debug, Clone)]
pub(crate) struct RankedDecoder {
    /// Name of the decoder
    pub(crate) name: String,
    /// Estimated cost for trying this decoder (lower = better)
    pub(crate) estimated_cost: f32,
}

/// Rank decoders by their estimated cost for a given input text and current path
///
/// This function estimates which decoders are most likely to be useful based on:
/// 1. Path complexity (encoders are cheaper than ciphers)
/// 2. Historical success rates
/// 3. Input text characteristics (entropy)
///
/// # Arguments
///
/// * `text` - The input text to analyze
/// * `path` - The current path of decoders already applied
/// * `available_decoders` - Names of decoders to rank
///
/// # Returns
///
/// A vector of RankedDecoder sorted by estimated_cost (lowest first)
pub(crate) fn rank_decoders(
    text: &str,
    path: &[CrackResult],
    available_decoders: &[String],
) -> Vec<RankedDecoder> {
    // Pre-compute values that are the same for all decoders to avoid
    // redundant work inside the per-decoder loop.
    let entropy = calculate_entropy(text);
    let existing_cipher_count = path.iter().filter(|p| !is_encoder(p.decoder)).count();
    let last_decoder_name = path.last().map(|p| p.decoder);

    let mut ranked: Vec<RankedDecoder> = available_decoders
        .iter()
        .map(|name| {
            let is_enc = is_encoder(name);

            // Estimate the cost for this decoder based on:
            // 1. Base cost: encoders are cheap, ciphers are expensive
            // 2. Repeated same-encoder bonus: if last decoder was same encoder, very cheap
            // 3. Cipher penalty: escalating penalty for multiple ciphers in path

            let estimated_path_cost = if is_enc {
                if last_decoder_name == Some(name.as_str()) {
                    // Repeated same encoder - very cheap
                    REPEATED_ENCODER_COST
                } else {
                    // Different encoder
                    ENCODER_BASE_COST
                }
            } else {
                // Cipher cost - escalating penalty
                let cipher_position = existing_cipher_count + 1;
                let popularity = get_decoder_popularity(name);
                let popularity_multiplier = POPULARITY_MULTIPLIER_CEILING - popularity;
                CIPHER_BASE_COST * cipher_position as f32 * popularity_multiplier
            };

            // Add success rate modifier (historical learning)
            let success_rate = get_decoder_success_rate(name);
            let success_penalty = (1.0 - success_rate) * SUCCESS_PENALTY_WEIGHT;

            // Add entropy-based modifier for encoders
            // Higher entropy input is more likely to be encoded
            let entropy_bonus = if is_enc && entropy > HIGH_ENTROPY_THRESHOLD {
                ENCODER_ENTROPY_BONUS
            } else {
                0.0
            };

            let estimated_cost = estimated_path_cost + success_penalty + entropy_bonus;

            RankedDecoder {
                name: name.clone(),
                estimated_cost,
            }
        })
        .collect();

    // Sort by estimated cost (lowest first = best)
    ranked.sort_by(|a, b| {
        a.estimated_cost
            .partial_cmp(&b.estimated_cost)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    ranked
}

/// Calculate Shannon entropy of a string (normalized to 0-1 range)
///
/// Entropy measures the "randomness" or information density of text.
/// - Plaintext English typically has entropy ~0.4-0.5 (normalized)
/// - Base64 encoded text has entropy ~0.75-0.85 (normalized)
/// - Random/encrypted text has entropy ~0.95-1.0 (normalized)
///
/// Lower entropy suggests the text is more likely to be meaningful plaintext.
///
/// # Arguments
///
/// * `text` - The string to calculate entropy for
///
/// # Returns
///
/// * Normalized entropy value between 0.0 and 1.0
pub(crate) fn calculate_entropy(text: &str) -> f32 {
    if text.is_empty() {
        return 1.0; // Empty string is maximally uncertain
    }

    // Fast path: use a fixed-size byte frequency array for ASCII text.
    // This avoids allocating a HashMap on every call, which matters because
    // calculate_entropy is invoked for every A* node.
    if text.is_ascii() {
        let mut freq = [0u32; 256];
        let len = text.len() as f32;
        for &b in text.as_bytes() {
            freq[b as usize] += 1;
        }
        let entropy: f32 = freq
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f32 / len;
                -p * p.log2()
            })
            .sum();
        // Normalize: max entropy for ASCII printable is ~log2(95) ≈ 6.57
        return (entropy / ENTROPY_NORMALIZATION).min(1.0);
    }

    // Slow path: fall back to HashMap for non-ASCII (Unicode) text
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let len = text.chars().count() as f32;
    let entropy: f32 = freq
        .values()
        .map(|&count| {
            let p = count as f32 / len;
            if p > 0.0 {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum();

    (entropy / ENTROPY_NORMALIZATION).min(1.0)
}

/// Check if a decoder is an encoder (has "decoder" tag) vs a cipher
///
/// Encoders (Base64, Hex, etc.) can be nested many times.
/// Ciphers (Caesar, Vigenère, etc.) are typically used 0-1 times.
///
/// # Arguments
///
/// * `decoder_name` - The name of the decoder to check
///
/// # Returns
///
/// * `true` if the decoder is an encoder, `false` if it's a cipher
pub(crate) fn is_encoder(decoder_name: &str) -> bool {
    if let Some(decoder_box) = DECODER_MAP.get(decoder_name) {
        let decoder = decoder_box.get::<()>();
        decoder.get_tags().contains(&"decoder")
    } else {
        // Default to treating unknown decoders as encoders (safer assumption)
        true
    }
}

/// Get the popularity score for a decoder
///
/// Higher popularity means the cipher is more commonly used and should be
/// tried before less popular ciphers. For example, Caesar (0.8) should be
/// tried before Vigenere (0.6).
///
/// # Arguments
///
/// * `decoder_name` - The name of the decoder to check
///
/// # Returns
///
/// * The popularity score (0.0 to 1.0), defaults to `DEFAULT_POPULARITY` if unknown
pub(crate) fn get_decoder_popularity(decoder_name: &str) -> f32 {
    if let Some(decoder_box) = DECODER_MAP.get(decoder_name) {
        let decoder = decoder_box.get::<()>();
        decoder.get_popularity()
    } else {
        DEFAULT_POPULARITY
    }
}

/// Calculate category-aware path complexity for Occam's Razor
///
/// This function calculates the "conceptual complexity" of a decoding path,
/// implementing Occam's Razor by making simpler explanations cheaper:
///
/// - Repeated same-encoder applications are cheap (`REPEATED_ENCODER_COST` each after the first)
/// - Different encoders cost more (`ENCODER_BASE_COST` each)
/// - Ciphers are expensive (base cost `CIPHER_BASE_COST`, escalating for multiple ciphers)
/// - Cipher cost is adjusted by popularity: higher popularity = lower cost
///
/// # Arguments
///
/// * `path` - The path of CrackResults representing decoders applied
///
/// # Returns
///
/// * The complexity score (lower = simpler = better)
///
/// # Examples
///
/// | Path | Cost |
/// |------|------|
/// | base64 × 5 | 0.7 + 0.2×4 = 1.5 |
/// | base64 → base32 → hex | 0.7×3 = 2.1 |
/// | base64 × 3 → caesar (0.8 pop) | 0.7 + 0.2×2 + 2.0×1.2 = 3.5 |
/// | base64 × 3 → vigenere (0.6 pop) | 0.7 + 0.2×2 + 2.0×1.4 = 3.9 |
/// | caesar → vigenere | 2.0×1.2 + 4.0×1.4 = 7.8 |
pub(crate) fn calculate_path_complexity(path: &[CrackResult]) -> f32 {
    if path.is_empty() {
        return 0.0;
    }

    let mut complexity = 0.0;
    let mut cipher_count = 0;
    let mut prev_decoder: Option<&str> = None;

    for step in path {
        let is_enc = is_encoder(step.decoder);
        let is_repeated = prev_decoder == Some(step.decoder);

        if !is_enc {
            // It's a cipher - expensive, escalating penalty for multiple
            cipher_count += 1;

            // Get cipher popularity to adjust cost
            // More popular ciphers (like Caesar) get lower cost multiplier
            let popularity = get_decoder_popularity(step.decoder);
            let popularity_multiplier = POPULARITY_MULTIPLIER_CEILING - popularity;

            complexity += CIPHER_BASE_COST * cipher_count as f32 * popularity_multiplier;
        } else if is_repeated {
            // Repeated same encoder (e.g., base64 → base64) is common
            complexity += REPEATED_ENCODER_COST;
        } else {
            // Different encoder
            complexity += ENCODER_BASE_COST;
        }

        prev_decoder = Some(step.decoder);
    }

    complexity
}

/// Update decoder statistics based on success or failure
///
/// # Arguments
///
/// * `decoder` - The name of the decoder
/// * `success` - Whether the decoder was successful
///
/// # Panics
///
/// Panics if the `DECODER_SUCCESS_RATES` RwLock is poisoned, which indicates
/// a bug (another thread panicked while holding the write lock).
pub(crate) fn update_decoder_stats(decoder: &str, success: bool) {
    let mut stats = DECODER_SUCCESS_RATES.write().unwrap();
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
///
/// # Panics
///
/// Panics if the `DECODER_SUCCESS_RATES` RwLock is poisoned, which indicates
/// a bug (another thread panicked while holding the write lock).
pub(crate) fn get_decoder_success_rate(decoder: &str) -> f32 {
    let stats = DECODER_SUCCESS_RATES.read().unwrap();
    if let Some((successes, total)) = stats.get(decoder) {
        if *total > 0 {
            return *successes as f32 / *total as f32;
        }
    }

    DEFAULT_SUCCESS_RATE
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
pub(crate) fn calculate_string_quality(s: &str) -> f32 {
    // Check for high percentage of invisible characters
    let non_printable_ratio = calculate_non_printable_ratio(s);
    if non_printable_ratio > QUALITY_NON_PRINTABLE_THRESHOLD {
        return 0.0; // Return lowest quality for strings with >50% invisible chars
    }

    // Factors to consider:
    // 1. Length (not too short, not too long)
    if s.len() < MIN_LENGTH_FOR_QUALITY {
        SHORT_STRING_QUALITY
    } else if s.len() > MAX_OPTIMAL_LENGTH {
        LONG_STRING_QUALITY
    } else {
        1.0 - (s.len() as f32 - QUALITY_IDEAL_LENGTH).abs() / QUALITY_LENGTH_DIVISOR
    }
}

/// Check if string is worth being decoded
pub(crate) fn calculate_string_worth(s: &str) -> bool {
    if calculate_string_quality(s) < MIN_QUALITY_THRESHOLD {
        return false;
    }

    true
}

/// Calculate the ratio of non-printable characters in a string
/// Returns a value between 0.0 (all printable) and 1.0 (all non-printable)
pub(crate) fn calculate_non_printable_ratio(text: &str) -> f32 {
    if text.is_empty() {
        return 1.0;
    }

    let non_printable_count = text
        .chars()
        .filter(|&c| {
            // Only count control characters (except common whitespace) and non-ASCII as non-printable
            (c.is_control() && c != '\n' && c != '\r' && c != '\t') || !c.is_ascii()
        })
        .count();

    non_printable_count as f32 / text.len() as f32
}

/// Generate a heuristic value for A* search prioritization
///
/// The heuristic estimates how close a state is to being plaintext.
/// A lower value indicates a more promising state.
///
/// ## Design Philosophy (Occam's Razor)
///
/// This heuristic is designed with Occam's Razor in mind: simpler explanations
/// should be preferred. The heuristic estimates the remaining "distance" to
/// plaintext based on:
///
/// 1. **Entropy**: Lower entropy text is more likely to be plaintext
/// 2. **Decoder success rate**: Use learned statistics about which decoders work
/// 3. **String quality**: Penalize garbled or non-printable text
///
/// Note: Path complexity is handled separately in `calculate_path_complexity`
/// and used as the `g` (cost) component of A*, not the `h` (heuristic).
///
/// # Parameters
///
/// * `text` - The text to analyze
/// * `path` - The path of decoders used to reach the current state (unused in new impl)
/// * `next_decoder` - The next decoder to be applied (if any)
///
/// # Returns
/// A float value representing the heuristic cost (lower is better)
#[allow(unused_variables)]
pub(crate) fn generate_heuristic(
    text: &str,
    path: &[CrackResult],
    next_decoder: &Option<Box<dyn Crack + Sync>>,
) -> f32 {
    let mut score = 0.0;

    // 1. Entropy score: lower entropy = more plaintext-like = lower score
    // Entropy is normalized to 0-1, we scale it for importance
    let entropy = calculate_entropy(text);
    score += entropy * ENTROPY_WEIGHT; // Range: 0-2

    // 2. Decoder success rate prior (if we know what decoder might be next)
    // Higher success rate = lower penalty
    if let Some(decoder) = next_decoder {
        let success_rate = get_decoder_success_rate(decoder.get_name());
        score += (1.0 - success_rate) * DECODER_SUCCESS_WEIGHT; // Range: 0-0.5
    } else {
        // Unknown next decoder, moderate penalty
        score += UNKNOWN_DECODER_PENALTY;
    }

    // 3. String quality penalty
    let quality = calculate_string_quality(text);
    score += (1.0 - quality) * QUALITY_WEIGHT; // Range: 0-0.5

    score
}

/// Determines if a string is too short to be meaningfully decoded
/// or is of too low quality to be worth decoding
///
/// ## Decision Criteria
///
/// A string is considered undecodeble if:
/// - It has `MIN_DECODABLE_LENGTH` or fewer characters
/// - It has more than `MAX_NON_PRINTABLE_RATIO` non-printable characters
/// - Its overall quality score is below `MIN_QUALITY_THRESHOLD`
///
/// ## Rationale
///
/// 1. The gibberish_or_not library requires at least 3 characters to work effectively
/// 2. LemmeKnow and other pattern matchers perform poorly on very short strings
/// 3. Most encoding schemes produce output of at least 3 characters
/// 4. Strings with high percentages of non-printable characters are unlikely to be valid encodings
/// 5. Very low quality strings waste computational resources and rarely yield useful results
///
/// Filtering out these strings early saves computational resources and
/// prevents the search from exploring unproductive paths.
pub(crate) fn check_if_string_cant_be_decoded(text: &str) -> bool {
    // Check for strings that are too short
    if text.len() <= MIN_DECODABLE_LENGTH {
        return true;
    }

    // Check for strings with high non-printable character ratio
    let non_printable_ratio = calculate_non_printable_ratio(text);
    if non_printable_ratio > MAX_NON_PRINTABLE_RATIO {
        return true;
    }

    // Check for overall string quality
    let quality = calculate_string_quality(text);
    if quality < MIN_QUALITY_THRESHOLD {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Decoder;

    #[test]
    fn test_calculate_entropy() {
        // Empty string should have max entropy (uncertain)
        assert_eq!(calculate_entropy(""), 1.0);

        // String with single repeated character should have very low entropy
        let single_char = "aaaaaaaaaa";
        assert!(calculate_entropy(single_char) < 0.1);

        // Normal English text should have moderate entropy (~0.4-0.6)
        let english = "Hello World this is a test";
        let english_entropy = calculate_entropy(english);
        assert!(english_entropy > 0.3 && english_entropy < 0.7);

        // Base64 should have higher entropy than English
        let base64 = "SGVsbG8gV29ybGQhdGhpcyBpcyBhIHRlc3Q=";
        let base64_entropy = calculate_entropy(base64);
        assert!(base64_entropy > english_entropy);

        // Random-looking text should have high entropy
        let random = "x9Kj2mPq8Zf3Yw5Lr1Nt";
        let random_entropy = calculate_entropy(random);
        assert!(random_entropy > 0.6);
    }

    #[test]
    fn test_is_encoder() {
        // Base64 should be an encoder (has "decoder" tag)
        assert!(is_encoder("Base64"));

        // Caesar should NOT be an encoder (it's a cipher)
        assert!(!is_encoder("caesar"));

        // Unknown decoder defaults to encoder (safer assumption)
        assert!(is_encoder("UnknownDecoder12345"));
    }

    #[test]
    fn test_calculate_path_complexity() {
        // Empty path should have zero complexity
        assert_eq!(calculate_path_complexity(&[]), 0.0);

        // Create mock CrackResults for different decoder types
        let base64_result = {
            let mut r = CrackResult::new(&Decoder::default(), "test".to_string());
            r.decoder = "Base64"; // This is an encoder
            r
        };

        let caesar_result = {
            let mut r = CrackResult::new(&Decoder::default(), "test".to_string());
            r.decoder = "caesar"; // This is a cipher
            r
        };

        // Single encoder: ENCODER_BASE_COST (0.7)
        let single_encoder = vec![base64_result.clone()];
        assert!(
            (calculate_path_complexity(&single_encoder) - ENCODER_BASE_COST).abs() < 0.01
        );

        // Repeated encoder (base64 × 3): 0.7 + 0.2 + 0.2 = 1.1
        let repeated_encoder = vec![
            base64_result.clone(),
            base64_result.clone(),
            base64_result.clone(),
        ];
        assert!(
            (calculate_path_complexity(&repeated_encoder)
                - (ENCODER_BASE_COST + REPEATED_ENCODER_COST * 2.0))
                .abs()
                < 0.01
        );

        // Single cipher: CIPHER_BASE_COST * cipher_count * popularity_multiplier
        // Caesar (when looked up in DECODER_MAP) has popularity 0.8, multiplier = 1.2
        // But test uses mock CrackResult, which falls back to DEFAULT_POPULARITY (0.5)
        // So multiplier = 2.0 - 0.5 = 1.5, cost = 2.0 * 1 * 1.5 = 3.0
        let single_cipher = vec![caesar_result.clone()];
        let caesar_cost = calculate_path_complexity(&single_cipher);
        assert!(
            caesar_cost >= 2.0 && caesar_cost <= 4.0,
            "Single caesar should cost between 2.0 and 4.0, got {}",
            caesar_cost
        );

        // Two ciphers: escalating penalty
        // First cipher: 2.0 * 1 * 1.5 = 3.0
        // Second cipher: 2.0 * 2 * 1.5 = 6.0
        // Total: 9.0
        let two_ciphers = vec![caesar_result.clone(), caesar_result.clone()];
        let two_cipher_cost = calculate_path_complexity(&two_ciphers);
        assert!(
            two_cipher_cost >= 6.0 && two_cipher_cost <= 12.0,
            "Two caesars should cost between 6.0 and 12.0, got {}",
            two_cipher_cost
        );

        // Mixed: base64 × 3 + caesar = 1.1 + ~3.0 = ~4.1
        let mixed = vec![
            base64_result.clone(),
            base64_result.clone(),
            base64_result.clone(),
            caesar_result.clone(),
        ];
        let mixed_cost = calculate_path_complexity(&mixed);
        assert!(
            mixed_cost >= 3.0 && mixed_cost <= 6.0,
            "Mixed path should cost between 3.0 and 6.0, got {}",
            mixed_cost
        );
    }

    #[test]
    fn test_generate_heuristic() {
        // Test that heuristic is non-negative
        let h = generate_heuristic("test", &[], &None);
        assert!(h >= 0.0);

        // Test that lower entropy text has lower heuristic
        // "aaaa" has very low entropy (repeated chars)
        // "x9Kj2mPq" has higher entropy (looks random)
        let low_entropy_h = generate_heuristic("aaaaaaaaaaaaaaaa", &[], &None);
        let high_entropy_h = generate_heuristic("x9Kj2mPq8Zf3Yw5L", &[], &None);
        assert!(low_entropy_h < high_entropy_h);
    }

    #[test]
    fn test_path_complexity_occams_razor() {
        // This test verifies that Occam's Razor is respected:
        // base64 × 10 should be cheaper than caesar → vigenere → atbash

        let base64_result = {
            let mut r = CrackResult::new(&Decoder::default(), "test".to_string());
            r.decoder = "Base64";
            r
        };

        let caesar_result = {
            let mut r = CrackResult::new(&Decoder::default(), "test".to_string());
            r.decoder = "caesar";
            r
        };

        let vigenere_result = {
            let mut r = CrackResult::new(&Decoder::default(), "test".to_string());
            r.decoder = "Vigenere";
            r
        };

        // base64 × 10: 0.7 + 0.2×9 = 2.5
        let many_base64: Vec<CrackResult> = (0..10).map(|_| base64_result.clone()).collect();
        let base64_cost = calculate_path_complexity(&many_base64);

        // caesar → vigenere (different ciphers): 2.0 + 4.0 = 6.0
        let two_ciphers = vec![caesar_result.clone(), vigenere_result.clone()];
        let cipher_cost = calculate_path_complexity(&two_ciphers);

        // Verify Occam's Razor: 10 base64s < 2 different ciphers
        assert!(
            base64_cost < cipher_cost,
            "base64×10 ({}) should be cheaper than caesar→vigenere ({})",
            base64_cost,
            cipher_cost
        );
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
    fn test_calculate_string_quality_with_invisible_chars() {
        // Test normal text
        let normal_quality = calculate_string_quality("Hello World");
        assert!(normal_quality > 0.0);

        // Test text with 40% invisible characters
        let text_with_some_invisible = "Hello\u{0}\u{0}\u{0}\u{0}World"; // 4 out of 14 chars are invisible
        let some_invisible_quality = calculate_string_quality(text_with_some_invisible);
        assert!(some_invisible_quality > 0.0);

        // Test text with 60% invisible characters (should return 0.0)
        let text_with_many_invisible = "\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}Hello"; // 7 out of 12 chars are invisible
        let many_invisible_quality = calculate_string_quality(text_with_many_invisible);
        assert_eq!(many_invisible_quality, 0.0);

        // Test text with 100% invisible characters
        let all_invisible = "\u{0}\u{0}\u{0}\u{0}\u{0}";
        let all_invisible_quality = calculate_string_quality(all_invisible);
        assert_eq!(all_invisible_quality, 0.0);
    }

    #[test]
    fn test_check_if_string_cant_be_decoded() {
        // Test strings that are too short
        assert!(
            check_if_string_cant_be_decoded(""),
            "Empty string should be rejected"
        );
        assert!(
            check_if_string_cant_be_decoded("a"),
            "Single character should be rejected"
        );
        assert!(
            check_if_string_cant_be_decoded("ab"),
            "Two characters should be rejected"
        );

        // Test strings with high non-printable character ratio
        let high_non_printable = "abc\u{0}\u{1}\u{2}"; // 3 out of 6 chars are non-printable (50%)
        assert!(
            check_if_string_cant_be_decoded(high_non_printable),
            "String with 50% non-printable characters should be rejected"
        );

        // Test strings with low quality
        // Create a string with >50% non-printable characters to ensure quality is 0.0
        let low_quality = "\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}abc"; // 7 out of 10 chars are non-printable (70%)
        assert!(
            check_if_string_cant_be_decoded(low_quality),
            "Low quality string should be rejected"
        );

        // Test valid strings
        assert!(
            !check_if_string_cant_be_decoded("Hello World"),
            "Normal text should be accepted"
        );
        assert!(
            !check_if_string_cant_be_decoded("SGVsbG8gV29ybGQ="), // Base64 for "Hello World"
            "Valid Base64 should be accepted"
        );
    }
}