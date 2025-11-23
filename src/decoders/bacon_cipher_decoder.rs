//! Decode Bacon Cipher
//! Performs error handling and returns a string
//!
//! Note: Bacon cipher hides a message within another message using two typefaces.
//! In CTF context, usually represented as AAAAA BBBBB or ABaba...
//! We support two alphabets:
//! 1. Standard (24 letter): I=J, U=V. 0-25 map.
//! 2. Distinct (26 letter): Unique codes for all.

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use log::{debug, info, trace};

pub struct BaconCipherDecoder;

impl Crack for Decoder<BaconCipherDecoder> {
    fn new() -> Decoder<BaconCipherDecoder> {
        Decoder {
            name: "Bacon Cipher",
            description: "Bacon's cipher or the Baconian cipher is a method of steganography devised by Francis Bacon in 1605. A message is concealed in the presentation of text, rather than its content.",
            link: "https://en.wikipedia.org/wiki/Bacon%27s_cipher",
            tags: vec!["bacon", "steganography", "decoder", "classic"],
            popularity: 0.4,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Bacon Cipher with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // We try both alphabets
        let mut potential_results = Vec::new();

        if let Some(res) = decode_bacon(text, false) { // 24 letter
            potential_results.push(res);
        }
        if let Some(res) = decode_bacon(text, true) { // 26 letter
            potential_results.push(res);
        }

        if potential_results.is_empty() {
            return results;
        }

        // We check all results
        let mut valid_results = Vec::new();
        for res in potential_results {
            if check_string_success(&res, text) {
                valid_results.push(res);
            }
        }

        if !valid_results.is_empty() {
            let checker_result = checker.check(&valid_results[0]); // Check first
            results.unencrypted_text = Some(valid_results);
            results.update_checker(&checker_result);
        }

        results
    }

    fn get_tags(&self) -> &Vec<&str> { &self.tags }
    fn get_name(&self) -> &str { self.name }
    fn get_popularity(&self) -> f32 { self.popularity }
    fn get_description(&self) -> &str { self.description }
    fn get_link(&self) -> &str { self.link }
}

fn decode_bacon(text: &str, distinct_alphabet: bool) -> Option<String> {
    // 1. Normalize text to A and B
    // Bacon cipher usually uses two types.
    // Common inputs: "AAAAA BBBBB", "ABABA", or mixed case "AbCdE" -> A=Upper, B=Lower or vice versa.
    // Or Typeface A vs Typeface B.
    // In text-only context, we assume A/B or similar characters are provided.

    let clean_text: String = text.chars().filter(|c| c.is_alphabetic()).collect();
    if clean_text.len() < 5 { return None; }

    // Heuristic to determine what maps to A and B
    // If only A and B (case insensitive) present:
    //   Assume A=A, B=B.
    // If mixed case of other letters:
    //   Upper=B, Lower=A (or vice versa).
    // Let's try to detect "Bacon string" (only As and Bs)
    let is_ab_only = clean_text.chars().all(|c| {
        let u = c.to_ascii_uppercase();
        u == 'A' || u == 'B'
    });

    let binary_string = if is_ab_only {
        clean_text.chars().map(|c| c.to_ascii_uppercase()).collect::<String>()
    } else {
        // Convert Upper -> B, Lower -> A (Common steganography)
        clean_text.chars().map(|c| if c.is_uppercase() { 'B' } else { 'A' }).collect::<String>()
    };

    let mut output = String::new();
    let chars: Vec<char> = binary_string.chars().collect();

    for chunk in chars.chunks(5) {
        if chunk.len() < 5 { break; }

        // Decode chunk
        let mut val = 0;
        for &c in chunk {
            val <<= 1;
            if c == 'B' {
                val |= 1;
            }
        }

        let decoded_char = if distinct_alphabet {
            // 26 letter alphabet: A=0 ... Z=25
            if val < 26 {
                (b'A' + val as u8) as char
            } else {
                return None; // Invalid for this alphabet
            }
        } else {
            // 24 letter alphabet: A-I (0-8), J=I, K-U (9-19), V=U, W-Z (20-23)
            // J maps to I (8), V maps to U (19) usually in encoding.
            // In decoding, if we get 8, is it I or J? Usually I.
            // If we get > 23, invalid.

            if val <= 8 {
                (b'A' + val as u8) as char
            } else if val <= 19 {
                 (b'A' + val as u8 + 1) as char // Skip J
            } else if val <= 23 {
                 (b'A' + val as u8 + 2) as char // Skip J, V
            } else {
                // Some variants map rest?
                return None;
            }
        };

        output.push(decoded_char);
    }

    if output.is_empty() { return None; }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::BaconCipherDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn bacon_simple_24() {
        // "AAAAA" -> A
        let decoder = Decoder::<BaconCipherDecoder>::new();
        let result = decoder.crack("AAAAA", &get_checker());
        assert!(result.unencrypted_text.is_some());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A");
    }

    #[test]
    fn bacon_wikipedia_example() {
        // "stike" example.
        // We expect TUSIKE (24) and STRIJE (26).
        let decoder = Decoder::<BaconCipherDecoder>::new();
        let input = "BAABA BAABB BAAAB ABAAA ABAAB AABAA";
        let result = decoder.crack(input, &get_checker());
        let valid = result.unencrypted_text.unwrap();

        let found = valid.iter().any(|s| s == "TUSIKE" || s == "STRIJE");
        assert!(found, "Decoded list {:?} does not contain TUSIKE or STRIJE", valid);
    }
}
