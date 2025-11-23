//! Decode Affine Cipher
//! Performs error handling and returns a string
//! Brute forces all possible keys.

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;
use gibberish_or_not::Sensitivity;

use log::{debug, info, trace};

pub struct AffineCipherDecoder;

impl Crack for Decoder<AffineCipherDecoder> {
    fn new() -> Decoder<AffineCipherDecoder> {
        Decoder {
            name: "Affine Cipher",
            description: "The Affine cipher is a type of monoalphabetic substitution cipher. It uses a mathematical function E(x) = (ax + b) mod 26.",
            link: "https://en.wikipedia.org/wiki/Affine_cipher",
            tags: vec!["affine", "substitution", "decoder", "classic"],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Affine Cipher with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // Coprimes to 26: 1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25
        let coprimes = vec![1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
        let mut best_candidates = Vec::new();

        // Calculate modular multiplicative inverses for valid 'a'
        // a * a_inv = 1 mod 26
        // 1->1, 3->9, 5->21, 7->15, 9->3, 11->19, 15->7, 17->23, 19->11, 21->5, 23->17, 25->25
        let inverses: Vec<(i32, i32)> = vec![
            (1, 1), (3, 9), (5, 21), (7, 15), (9, 3), (11, 19),
            (15, 7), (17, 23), (19, 11), (21, 5), (23, 17), (25, 25)
        ];

        let checker_with_sensitivity = checker.with_sensitivity(Sensitivity::Medium);

        for (a, a_inv) in inverses {
            for b in 0..26 {
                let decoded = decrypt_affine(text, a_inv, b);
                if check_string_success(&decoded, text) {
                     // Check if it looks like English
                     let check_res = checker_with_sensitivity.check(&decoded);
                     if check_res.is_identified {
                         best_candidates.push(decoded);
                         // If we find a very good match, maybe stop? But short strings might match multiple.
                         // Affine space is small (312), checking all is fast.
                     }
                }
            }
        }

        if !best_candidates.is_empty() {
             // Use first one for update_checker but return all?
             let checker_result = checker.check(&best_candidates[0]);
             results.unencrypted_text = Some(best_candidates);
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

fn decrypt_affine(text: &str, a_inv: i32, b: i32) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' } as i32;
            let y = c as i32 - base;
            // D(x) = a_inv * (y - b) mod 26
            let val = (a_inv * (y - b)).rem_euclid(26);
            ((base + val) as u8) as char
        } else {
            c
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::AffineCipherDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn affine_basic() {
        // "AFFINE CIPHER" -> a=5, b=8.
        // A(0) -> 5*0+8 = 8(I).
        // F(5) -> 5*5+8 = 33 -> 7(H).
        // ... "IHHWVC SWFRCP"
        let decoder = Decoder::<AffineCipherDecoder>::new();
        let result = decoder.crack("IHHWVC SWFRCP", &get_checker());
        assert!(result.unencrypted_text.is_some());
        assert!(result.unencrypted_text.unwrap().contains(&"AFFINE CIPHER".to_string()));
    }
}
