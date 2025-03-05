//! # Helper Functions for A* Search
//!
//! This module contains helper functions used by the A* search algorithm
//! for decoding encrypted or encoded text.

use crate::CrackResult;
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;

/// Mapping between Cipher Identifier's cipher names and Ares decoder names
///
/// This static mapping allows us to translate between the cipher types identified by
/// Cipher Identifier and the corresponding decoders available in Ares.
///
/// For example:
/// - "fractionatedMorse" maps to "MorseCodeDecoder"
/// - "atbash" maps to "AtbashDecoder"
/// - "caesar" maps to "CaesarDecoder"
pub static CIPHER_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
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
pub static DECODER_SUCCESS_RATES: Lazy<Mutex<HashMap<String, (usize, usize)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Update decoder statistics based on success or failure
///
/// # Arguments
///
/// * `decoder` - The name of the decoder
/// * `success` - Whether the decoder was successful
pub fn update_decoder_stats(decoder: &str, success: bool) {
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
pub fn get_decoder_success_rate(decoder: &str) -> f32 {
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
pub fn get_cipher_identifier_score(text: &str) -> (String, f32) {
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
pub fn is_common_sequence(prev_decoder: &str, current_cipher: &str) -> bool {
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
pub fn calculate_string_quality(s: &str) -> f32 {
    // Check for high percentage of invisible characters
    let non_printable_ratio = calculate_non_printable_ratio(s);
    if non_printable_ratio > 0.5 {
        return 0.0; // Return lowest quality for strings with >50% invisible chars
    }

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
pub fn calculate_non_printable_ratio(text: &str) -> f32 {
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

/// Get the popularity rating of a decoder by its name
/// Popularity is a value between 0.0 and 1.0 where higher values indicate more common decoders
pub fn get_decoder_popularity(decoder: &str) -> f32 {
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
pub fn generate_heuristic(text: &str, path: &[CrackResult]) -> f32 {
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
pub fn check_if_string_cant_be_decoded(text: &str) -> bool {
    text.len() <= 2 // Only check length now, non-printable chars handled by heuristic
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Decoder;

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
}
