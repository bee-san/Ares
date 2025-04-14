//! Vigenère cipher decoder with automated key detection
//! Uses Index of Coincidence (IoC) for key length detection and frequency analysis for key discovery
//! Returns Option<String> with the decrypted text if successful
//! Uses Medium sensitivity for gibberish detection as the default.

use super::crack_results::CrackResult;
use super::interface::{Crack, Decoder};
use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::storage::ENGLISH_FREQS;
use gibberish_or_not::Sensitivity;
use log::{debug, info, trace};

/// Expected Index of Coincidence for English text
const EXPECTED_IOC: f64 = 0.0667;

/// The Vigenère decoder struct
pub struct VigenereDecoder;

impl Crack for Decoder<VigenereDecoder> {
    fn new() -> Decoder<VigenereDecoder> {
        Decoder {
            name: "Vigenere",
            description: "A polyalphabetic substitution cipher using a keyword to shift each letter. This implementation automatically detects the key length and breaks the cipher. Uses Medium sensitivity for gibberish detection.",
            link: "https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher",
            tags: vec!["substitution", "classical"],
            popularity: 0.6,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Attempting Vigenère decryption on text: {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // Clean the input text (remove non-alphabetic characters)
        let clean_text: String = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();

        if clean_text.is_empty() {
            debug!("No valid characters found in input text");
            return results;
        }

        // Try key lengths from 1 to 20 (typical Vigenère key length range)
        let mut best_key_length = 0;
        let mut best_ioc = 0.0;

        for key_length in 1..=20 {
            let ioc = calculate_average_ioc(&clean_text, key_length);
            if (ioc - EXPECTED_IOC).abs() < (best_ioc - EXPECTED_IOC).abs() {
                best_ioc = ioc;
                best_key_length = key_length;
            }
        }

        if best_key_length == 0 {
            debug!("Failed to determine key length");
            return results;
        }

        // Find the key using frequency analysis
        let key = find_key(&clean_text, best_key_length);

        // Decrypt using the found key
        let decrypted = decrypt(&clean_text, &key);

        // Reconstruct original formatting
        let final_text = reconstruct_formatting(text, &decrypted);

        if !check_string_success(&final_text, text) {
            info!("Failed Vigenère decoding validation");
            return results;
        }

        // Use Medium sensitivity for Vigenere decoder
        let checker_with_sensitivity = checker.with_sensitivity(Sensitivity::Medium);
        let checker_result = checker_with_sensitivity.check(&final_text);

        results.unencrypted_text = Some(vec![final_text]);
        results.key = Some(key);
        results.update_checker(&checker_result);

        results
    }

    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

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

/// Calculate Index of Coincidence for text split into key_length columns
fn calculate_average_ioc(text: &str, key_length: usize) -> f64 {
    let mut total_ioc = 0.0;
    let text_bytes: Vec<u8> = text.bytes().collect();

    for i in 0..key_length {
        let mut freqs = [0; 26];
        let mut count = 0;

        for j in (i..text_bytes.len()).step_by(key_length) {
            if text_bytes[j].is_ascii_uppercase() {
                freqs[(text_bytes[j] - b'A') as usize] += 1;
                count += 1;
            } else if text_bytes[j].is_ascii_lowercase() {
                freqs[(text_bytes[j] - b'a') as usize] += 1;
                count += 1;
            }
        }

        if count > 1 {
            let mut column_ioc = 0.0;
            for freq in freqs.iter() {
                column_ioc += (*freq as f64) * ((*freq - 1) as f64);
            }
            column_ioc /= (count * (count - 1)) as f64;
            total_ioc += column_ioc;
        }
    }

    total_ioc / key_length as f64
}

/// Find the encryption key using frequency analysis
fn find_key(text: &str, key_length: usize) -> String {
    let mut key = String::with_capacity(key_length);
    let text_bytes: Vec<u8> = text.bytes().collect();

    for i in 0..key_length {
        let mut freqs = [0.0; 26];
        let mut count = 0;

        // Calculate frequency distribution for this column
        for j in (i..text_bytes.len()).step_by(key_length) {
            if text_bytes[j].is_ascii_alphabetic() {
                let idx = (text_bytes[j].to_ascii_uppercase() - b'A') as usize;
                freqs[idx] += 1.0;
                count += 1;
            }
        }

        // Normalize frequencies
        if count > 0 {
            for freq in freqs.iter_mut() {
                *freq /= count as f64;
            }
        }

        // Try each possible shift and calculate chi-squared statistic
        let mut best_shift = 0;
        let mut best_chi_squared = f64::MAX;

        for shift in 0..26 {
            let mut chi_squared = 0.0;
            for j in 0..26 {
                let expected = ENGLISH_FREQS[j];
                let observed = freqs[(j + shift) % 26];
                let diff = observed - expected;
                chi_squared += diff * diff / expected;
            }
            if chi_squared < best_chi_squared {
                best_chi_squared = chi_squared;
                best_shift = shift;
            }
        }

        key.push((b'A' + best_shift as u8) as char);
    }

    key
}

/// Decrypt text using the found key
fn decrypt(text: &str, key: &str) -> String {
    let key_bytes: Vec<u8> = key.bytes().collect();
    let mut result = String::with_capacity(text.len());
    let mut key_idx = 0;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let shift = (key_bytes[key_idx % key_bytes.len()] - b'A') as i8;
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let pos = ((c as u8) - base) as i8;
            let new_pos = ((pos - shift + 26) % 26) as u8;
            result.push((base + new_pos) as char);
            key_idx += 1;
        } else {
            result.push(c);
        }
    }

    result
}

/// Reconstruct original text formatting
fn reconstruct_formatting(original: &str, decrypted: &str) -> String {
    let mut result = String::with_capacity(original.len());
    let mut dec_iter = decrypted.chars().filter(|c| c.is_ascii_alphabetic());

    for c in original.chars() {
        if c.is_ascii_alphabetic() {
            if let Some(dec_char) = dec_iter.next() {
                result.push(dec_char);
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::{
        athena::Athena,
        checker_type::{Check, Checker},
        CheckerTypes,
    };

    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn test_vigenere_decoding() {
        let vigenere_decoder = Decoder::<VigenereDecoder>::new();
        let result = vigenere_decoder
            .crack(
                "Vyc fnqkm spdpv nqo hjfxa qmcg 13 eiha umvl.",
                &get_athena_checker(),
            )
            .unencrypted_text;

        assert!(result.is_some());
        let _decoded_text = &result.as_ref().unwrap()[0];
    }

    #[test]
    fn test_vigenere_with_special_chars() {
        let vigenere_decoder = Decoder::<VigenereDecoder>::new();
        let result = vigenere_decoder
            .crack(
                "Jvjah Asgccihva! Vycgx'a i ffe xg ug ecmhxb",
                &get_athena_checker(),
            )
            .unencrypted_text;

        assert!(result.is_some());
        let _decoded_text = &result.as_ref().unwrap()[0];
    }

    #[test]
    fn test_empty_input() {
        let vigenere_decoder = Decoder::<VigenereDecoder>::new();
        let result = vigenere_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn test_non_alphabetic_input() {
        let vigenere_decoder = Decoder::<VigenereDecoder>::new();
        let result = vigenere_decoder
            .crack("12345!@#$%", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
