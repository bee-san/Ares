use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{info, trace};

/// Railfence Decoder
pub struct RailfenceDecoder;

impl Crack for Decoder<RailfenceDecoder> {
    fn new() -> Decoder<RailfenceDecoder> {
        Decoder {
            name: "railfence",
            description: "The rail fence cipher (also called a zigzag cipher) is a classical type of transposition cipher. It derives its name from the manner in which encryption is performed, in analogy to a fence built with horizontal rails.",
            link: "https://en.wikipedia.org/wiki/Rail_fence_cipher",
            tags: vec!["railfence", "cipher", "classic", "transposition"],
            popularity: 5.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying railfence with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let mut decoded_strings = Vec::new();

        for rails in 2..10 {
            // Should be less than (rail * 2 - 3). This is the max offset
            for offset in 0..=(rails * 2 - 3) {
                let decoded_text = railfence_decoder(text, rails, offset);
                decoded_strings.push(decoded_text);
                let borrowed_decoded_text = &decoded_strings[decoded_strings.len() - 1];
                if !check_string_success(borrowed_decoded_text, text) {
                    info!(
                    "Failed to decode railfence because check_string_success returned false on string {}. This means the string is 'funny' as it wasn't modified.",
                    borrowed_decoded_text
                );
                    return results;
                }
                let checker_result = checker.check(borrowed_decoded_text);
                if checker_result.is_identified {
                    trace!(
                        "Found a match with railfence {} rails and {} offset",
                        rails,
                        offset
                    );
                    results.unencrypted_text = Some(vec![borrowed_decoded_text.to_string()]);
                    results.update_checker(&checker_result);
                    return results;
                }
            }
        }
        results.unencrypted_text = Some(decoded_strings);
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
}

/// Decodes a text encoded with the Rail Fence Cipher with the specified number of rails and offset
fn railfence_decoder(text: &str, rails: usize, offset: usize) -> String {
    let mut indexes: Vec<_> = zigzag(rails, offset).zip(1..).take(text.len()).collect();
    indexes.sort();
    let mut char_with_index: Vec<_> = text
        .chars()
        .zip(indexes)
        .map(|(c, (_, i))| (i, c))
        .collect();
    char_with_index.sort();
    char_with_index.iter().map(|(_, c)| c).collect()
}

/// Returns an iterator that yields the indexes of a zigzag pattern with the specified number of rails and offset
fn zigzag(n: usize, offset: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle().skip(offset)
}

#[cfg(test)]
mod tests {
    use super::RailfenceDecoder;
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
    fn railfence_decodes_successfully() {
        // This tests if Railfence can decode Railfence successfully
        // Key is 5 rails and 3 offset
        let railfence_decoder = Decoder::<RailfenceDecoder>::new();
        let result = railfence_decoder.crack(
            "xcz n akt,emiol r gywShfbqajd op uuv",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "Sphinx of black quartz, judge my vow"
        );
    }

    #[test]
    fn railfence_handles_panic_if_empty_string() {
        // This tests if Railfence can handle an empty string
        // It should return None
        let railfence_decoder = Decoder::<RailfenceDecoder>::new();
        let result = railfence_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn railfence_handles_panic_if_emoji() {
        // This tests if Railfence can handle an emoji
        // It should return None
        let railfence_decoder = Decoder::<RailfenceDecoder>::new();
        let result = railfence_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
