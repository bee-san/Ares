use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{info, trace};

///! Atbash Decoder
pub struct AtbashDecoder;

impl Crack for Decoder<AtbashDecoder> {
    fn new() -> Decoder<AtbashDecoder> {
        Decoder {
            name: "atbash",
            description: "Atbash is a monoalphabetic substitution cipher originally used to encrypt the Hebrew alphabet. It can be modified for use with any known writing system with a standard collating order.",
            link: "https://en.wikipedia.org/wiki/Atbash",
            tags: vec!["atbash", "substitution", "decoder"],
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 1.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying atbash with text {:?}", text);
        let decoded_text = atbash_to_alphabet(text);

        trace!("Decoded text for atbash: {:?}", decoded_text);
        let mut results = CrackResult::new(self, text.to_string());

        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode atbash because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(decoded_text);

        results.update_checker(&checker_result);

        results
    }
}

/// Maps atbash to the alphabet
fn atbash_to_alphabet(text: &str) -> String {
    text.chars()
        .map(|char| match char {
            letter @ 'a'..='z' => (b'a' + b'z' - letter as u8) as char,
            letter @ 'A'..='Z' => (b'A' + b'Z' - letter as u8) as char,
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::athena::Athena;
    use crate::checkers::checker_type::{Check, Checker};
    use crate::checkers::CheckerTypes;
    use crate::decoders::interface::Crack;

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn test_atbash() {
        let decoder = Decoder::<AtbashDecoder>::new();
        let result = decoder.crack("svool dliow", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap(), "hello world");
    }
}
