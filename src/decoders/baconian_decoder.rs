//! Baconian cipher decoder
//! The Baconian cipher is a steganographic cipher that encodes each letter as a 5-character
//! sequence using two distinct symbols (traditionally A and B).
//!
//! Example: "HELLO" → "AABBB AABAA ABABA ABABA ABBAB"

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};
use std::collections::HashSet;

/// Baconian Cipher Decoder
/// Decodes text encoded with the Baconian cipher (5-bit binary encoding using two symbols)
/// ```
/// use ciphey::decoders::baconian_decoder::BaconianDecoder;
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decoder = Decoder::<BaconianDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decoder.crack("AABBB AABAA ABABA ABABA ABBAB", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "HELLO");
/// ```
pub struct BaconianDecoder;

/// 24-letter Baconian alphabet (I/J and U/V combined - original Bacon cipher)
const BACONIAN_24: [&str; 24] = [
    "AAAAA", "AAAAB", "AAABA", "AAABB", "AABAA", "AABAB", "AABBA", "AABBB", // A-H
    "ABAAA", "ABAAB", "ABABA", "ABABB", "ABBAA", "ABBAB", "ABBBA", "ABBBB", // I-P
    "BAAAA", "BAAAB", "BAABA", "BAABB", "BABAA", "BABAB", "BABBA", "BABBB", // Q-X
];

/// Letters corresponding to 24-letter Baconian alphabet
const LETTERS_24: [char; 24] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'W', 'X', 'Y', 'Z',
];

/// 26-letter Baconian alphabet (modern variant with distinct I/J and U/V)
const BACONIAN_26: [&str; 26] = [
    "AAAAA", "AAAAB", "AAABA", "AAABB", "AABAA", "AABAB", "AABBA", "AABBB", // A-H
    "ABAAA", "ABAAB", "ABABA", "ABABB", "ABBAA", "ABBAB", "ABBBA", "ABBBB", // I-P
    "BAAAA", "BAAAB", "BAABA", "BAABB", "BABAA", "BABAB", "BABBA", "BABBB", // Q-X
    "BBAAA", "BBAAB", // Y-Z
];

/// Letters corresponding to 26-letter Baconian alphabet
const LETTERS_26: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Crack for Decoder<BaconianDecoder> {
    fn new() -> Decoder<BaconianDecoder> {
        Decoder {
            name: "Baconian",
            description: "The Baconian cipher is a steganographic cipher created by Francis Bacon. Each letter is encoded as a 5-character sequence using two distinct symbols (traditionally A and B). It can hide messages in plain sight using any two distinguishable elements.",
            link: "https://en.wikipedia.org/wiki/Bacon%27s_cipher",
            tags: vec!["baconian", "bacon", "steganography", "substitution", "decoder"],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Baconian with text {:?}", text);
        let decoded_text = decode_baconian(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode Baconian because decode_baconian returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode Baconian because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }

    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }

    /// Gets the description for the current decoder
    fn get_description(&self) -> &str {
        self.description
    }

    /// Gets the link for the current decoder
    fn get_link(&self) -> &str {
        self.link
    }
}

/// Decode Baconian cipher text to plain text
/// Returns None if input doesn't appear to be valid Baconian
fn decode_baconian(text: &str) -> Option<String> {
    if text.is_empty() {
        return None;
    }

    // Normalize the text: remove common delimiters, keep only potential Baconian characters
    let normalized = normalize_baconian_input(text);

    if normalized.is_empty() {
        return None;
    }

    // Find the two unique symbols used
    let unique_chars: Vec<char> = normalized.chars().collect::<HashSet<_>>().into_iter().collect();

    // Baconian requires 1-2 unique symbols (1 for patterns like AAAAA = A)
    if unique_chars.is_empty() || unique_chars.len() > 2 {
        return None;
    }

    // Text length must be divisible by 5 (each letter is 5 symbols)
    if !normalized.len().is_multiple_of(5) {
        return None;
    }

    // Minimum reasonable length (at least one letter = 5 chars)
    if normalized.len() < 5 {
        return None;
    }

    // Handle single-symbol case (e.g., "AAAAA" = A, "BBBBB" = invalid)
    if unique_chars.len() == 1 {
        let symbol = unique_chars[0];
        // Only all-A pattern is valid (decodes to all A's)
        return try_decode_with_mapping(&normalized, symbol, symbol);
    }

    // Try both possible symbol mappings (A→first, B→second AND A→second, B→first)
    let symbol_a = unique_chars[0];
    let symbol_b = unique_chars[1];

    // Try mapping 1: first symbol = A, second symbol = B
    if let Some(decoded) = try_decode_with_mapping(&normalized, symbol_a, symbol_b) {
        return Some(decoded);
    }

    // Try mapping 2: first symbol = B, second symbol = A
    if let Some(decoded) = try_decode_with_mapping(&normalized, symbol_b, symbol_a) {
        return Some(decoded);
    }

    None
}

/// Normalize Baconian input by removing delimiters and whitespace
fn normalize_baconian_input(text: &str) -> String {
    // Remove common delimiters (spaces, dashes, underscores, etc.)
    text.chars()
        .filter(|c| !c.is_whitespace() && !matches!(*c, '-' | '_' | ',' | ';' | ':' | '.' | '/'))
        .collect()
}

/// Try to decode with a specific symbol mapping
/// Returns Some(decoded) if successful, None otherwise
fn try_decode_with_mapping(normalized: &str, symbol_a: char, symbol_b: char) -> Option<String> {
    // Convert to standard A/B format
    let standardized: String = normalized
        .chars()
        .map(|c| {
            if c == symbol_a {
                'A'
            } else if c == symbol_b {
                'B'
            } else {
                c
            }
        })
        .collect();

    // Try 26-letter alphabet first (more common in modern usage)
    if let Some(decoded) = decode_with_alphabet(&standardized, &BACONIAN_26, &LETTERS_26) {
        // Check if the result looks reasonable (contains mostly letters)
        if is_reasonable_output(&decoded) {
            return Some(decoded);
        }
    }

    // Try 24-letter alphabet (original Bacon cipher)
    if let Some(decoded) = decode_with_alphabet(&standardized, &BACONIAN_24, &LETTERS_24) {
        if is_reasonable_output(&decoded) {
            return Some(decoded);
        }
    }

    None
}

/// Decode using a specific Baconian alphabet
fn decode_with_alphabet(
    standardized: &str,
    baconian_table: &[&str],
    letter_table: &[char],
) -> Option<String> {
    let mut result = String::new();

    for chunk in standardized.as_bytes().chunks(5) {
        if chunk.len() != 5 {
            return None;
        }

        let pattern = std::str::from_utf8(chunk).ok()?;

        // Find the corresponding letter
        let letter = baconian_table
            .iter()
            .position(|&p| p == pattern)
            .and_then(|idx| letter_table.get(idx).copied());

        match letter {
            Some(l) => result.push(l),
            None => return None, // Invalid pattern
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// Check if the decoded output looks reasonable
/// (mainly checking it's not empty and contains recognizable characters)
fn is_reasonable_output(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::BaconianDecoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn baconian_decodes_hello_successfully() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // HELLO in Baconian (26-letter): H=AABBB, E=AABAA, L=ABABB, L=ABABB, O=ABBBA
        let result = decoder.crack("AABBB AABAA ABABB ABABB ABBBA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_without_spaces() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder.crack("AABBBAABAAABABBABABBABBBA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_with_zeros_and_ones() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // HELLO using 0 for A and 1 for B: H=00111, E=00100, L=01011, L=01011, O=01110
        let result = decoder.crack("00111 00100 01011 01011 01110", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_with_custom_symbols() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // HELLO using X for A and Y for B: H=XXYYY, E=XXYXX, L=XYXYY, L=XYXYY, O=XYYYX
        let result = decoder.crack("XXYYY XXYXX XYXYY XYXYY XYYYX", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_lowercase() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder.crack("aabbb aabaa ababb ababb abbba", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_with_dashes() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder.crack("AABBB-AABAA-ABABB-ABABB-ABBBA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn baconian_decodes_single_letter_a() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // A = AAAAA (single symbol case)
        let result = decoder.crack("AAAAA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A");
    }

    #[test]
    fn baconian_decodes_single_letter_b() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // B = AAAAB
        let result = decoder.crack("AAAAB", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "B");
    }

    #[test]
    fn baconian_decodes_world() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // WORLD (26-letter): W=BABBA, O=ABBBA, R=BAAAB, L=ABABB, D=AAABB
        let result = decoder.crack("BABBA ABBBA BAAAB ABABB AAABB", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "WORLD");
    }

    #[test]
    fn baconian_decode_empty_string() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder.crack("", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_decode_invalid_length() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // Not divisible by 5
        let result = decoder.crack("AABB", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_decode_too_many_symbols() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // More than 2 unique symbols
        let result = decoder.crack("ABCDE", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_decode_single_symbol_valid() {
        let decoder = Decoder::<BaconianDecoder>::new();
        // Only 1 unique symbol - valid for 'A' pattern
        let result = decoder.crack("AAAAA", &get_athena_checker()).unencrypted_text;
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0], "A");
    }

    #[test]
    fn baconian_decode_handles_panics() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_handle_panic_if_emoji() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder.crack("😂", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_full_alphabet_26() {
        // Test a longer message that uses more of the alphabet
        let decoder = Decoder::<BaconianDecoder>::new();
        // "THE" = T=BAABB, H=AABBB, E=AABAA (26-letter)
        let result = decoder.crack("BAABB AABBB AABAA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "THE");
    }

    #[test]
    fn baconian_decode_whitespace_only() {
        let decoder = Decoder::<BaconianDecoder>::new();
        let result = decoder
            .crack("   \t\n  ", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn baconian_decodes_ctf_example() {
        // A typical CTF example with hidden message
        let decoder = Decoder::<BaconianDecoder>::new();
        // "CAT" = C=AAABA, A=AAAAA, T=BAABB (26-letter)
        // This is unambiguous because AAAAA only maps to A with the correct mapping
        let result = decoder.crack("AAABA AAAAA BAABB", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "CAT");
    }
}
