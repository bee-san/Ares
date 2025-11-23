//! Decode Beaufort Cipher
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;
use gibberish_or_not::Sensitivity;
use once_cell::sync::Lazy;

use log::{debug, info, trace};

/// English bigrams for determining fitness (reused from Vigenere/shared location conceptually)
/// We use include_str! to embed the file content at compile time for portability.
static ENGLISH_BIGRAMS: Lazy<Vec<Vec<i64>>> = Lazy::new(|| {
    let mut bigrams_vec = vec![vec![0; 26]; 26];

    // Load the file content at compile time
    let content = include_str!("../storage/ngrams/english_bigrams.txt");

    let content_lines = content.split('\n');
    for line in content_lines {
        if line.is_empty() { continue; }
        let line_split: Vec<&str> = line.split_ascii_whitespace().collect();
        if line_split.is_empty() { continue; }
        let mut chars_itr = line_split[0].chars();
        let char1: char = chars_itr.next().expect("char1").to_ascii_uppercase();
        let char2: char = chars_itr.next().expect("char2").to_ascii_uppercase();

        let fitness = line_split[1].parse::<i64>().expect("fitness");
        bigrams_vec[(char1 as u8 - b'A') as usize][(char2 as u8 - b'A') as usize] = fitness;
    }
    bigrams_vec
});

pub struct BeaufortDecoder;

impl Crack for Decoder<BeaufortDecoder> {
    fn new() -> Decoder<BeaufortDecoder> {
        Decoder {
            name: "Beaufort Cipher",
            description: "The Beaufort cipher is a polyalphabetic substitution cipher, similar to Vigenère, but using a slightly different tableau and encryption mechanism (reciprocal).",
            link: "https://en.wikipedia.org/wiki/Beaufort_cipher",
            tags: vec!["beaufort", "substitution", "decoder", "classic", "vigenere-variant"],
            popularity: 0.4,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Beaufort Cipher with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        let clean_text: String = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        if clean_text.is_empty() { return results; }

        let checker_with_sensitivity = checker.with_sensitivity(Sensitivity::Medium);

        for key_length in 3..20 { // Check reasonable key lengths
             let key = break_beaufort(text, key_length);
             if key.trim().is_empty() { continue; }

             let decoded = decrypt_beaufort(text, &key);
             let check_res = checker_with_sensitivity.check(&decoded);
             if check_res.is_identified {
                 results.unencrypted_text = Some(vec![decoded]);
                 results.key = Some(key);
                 results.update_checker(&check_res);
                 return results;
             }
        }

        results
    }

    fn get_tags(&self) -> &Vec<&str> { &self.tags }
    fn get_name(&self) -> &str { self.name }
    fn get_popularity(&self) -> f32 { self.popularity }
    fn get_description(&self) -> &str { self.description }
    fn get_link(&self) -> &str { self.link }
}

// Beaufort Decryption: M = (K - C) mod 26
// Vigenere was M = (C - K) mod 26
fn decrypt_beaufort(text: &str, key: &str) -> String {
    let key_bytes: Vec<u8> = key.bytes().collect();
    let mut result = String::with_capacity(text.len());
    let mut key_idx = 0;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let key_char = key_bytes[key_idx % key_bytes.len()].to_ascii_uppercase();
            let key_val = (key_char - b'A') as i32;

            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' } as i32;
            let cipher_val = (c as i32) - base;

            // M = (K - C) mod 26
            let plain_val = (key_val - cipher_val).rem_euclid(26);

            result.push(((base + plain_val) as u8) as char);
            key_idx += 1;
        } else {
            result.push(c);
        }
    }
    result
}

fn break_beaufort(text: &str, key_length: usize) -> String {
    // Similar to Vigenere breaker but optimized for Beaufort fitness
    // We assume key length is correct and try to find best key char for each position

    let mut cipher_text: Vec<usize> = Vec::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            cipher_text.push(((c.to_ascii_uppercase() as u8) - b'A') as usize);
        }
    }

    // Initialize key with A's
    let mut key = vec![0; key_length];

    // Iterative improvement (hill climbing-ish) like in vigenere decoder logic
    // We cycle through key positions and try all 26 chars, keeping best fitness

    // Since bigrams depend on adjacent characters, and adjacent chars are decrypted by adjacent key parts,
    // we can optimize pairs? Or just one by one.
    // The vigenere logic iterates pairs.

    // Ported/Adapted logic:
    let mut _best_fitness_global = 0;

    // We do a few passes to stabilize
    for _ in 0..2 {
        for key_idx in 0..key_length {
            let mut best_char_fitness = -1;
            let mut best_char = 0;

            for k_char in 0..26 {
                key[key_idx] = k_char;

                // Calculate fitness for this column (pairs involving this column)
                // Actually, full text fitness is better but slower.
                // Let's just sum bigram scores for the whole text with current key
                let mut fitness = 0;
                for i in 0..(cipher_text.len()-1) {
                    let k1 = key[i % key_length];
                    let k2 = key[(i+1) % key_length];
                    let c1 = cipher_text[i] as i32;
                    let c2 = cipher_text[i+1] as i32;

                    // M = (K - C) mod 26
                    let m1 = (k1 as i32 - c1).rem_euclid(26);
                    let m2 = (k2 as i32 - c2).rem_euclid(26);

                    fitness += ENGLISH_BIGRAMS[m1 as usize][m2 as usize];
                }

                if fitness > best_char_fitness {
                    best_char_fitness = fitness;
                    best_char = k_char;
                }
            }
            key[key_idx] = best_char;
        }
    }

    key.iter().map(|&b| ((b as u8) + b'A') as char).collect()
}

#[cfg(test)]
mod tests {
    use super::BeaufortDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn _get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn beaufort_basic() {
        let _decoder = Decoder::<BeaufortDecoder>::new();
        let _text = "The quick brown fox jumps over the lazy dog. This is a simple test sentence for the Beaufort cipher.";
        let _input = "DANZQ CWNNH"; // Too short.
    }
}
