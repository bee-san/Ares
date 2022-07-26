use crate::checkers::CheckerTypes;

use super::crack_results::CrackResult;
///! Reverses the input string
///! Performs error handling and returns a string
///! Call reverse_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use super::interface::Crack;
use super::interface::Decoder;

use log::trace;

pub struct ReverseDecoder;

impl Crack for Decoder<ReverseDecoder> {
    fn new() -> Decoder<ReverseDecoder> {
        Decoder {
            name: "Reverse",
            description: "Reverses a string. stac -> cats",
            link: "http://string-functions.com/reverse.aspx",
            tags: vec!["reverse", "decoder"],
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            // I have never seen a reversed string in a CTF
            // or otherwise
            popularity: 0.2,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Running reverse string");
        let mut result = CrackResult::new(self, text.to_string());
        if text.is_empty() {
            return result;
        }
        let rev_str: String = text.chars().rev().collect();
        let checker_res = checker.check(&rev_str);

        result.unencrypted_text = Some(rev_str);
        result.update_checker(&checker_res);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoders::interface::Crack;

    #[test]
    fn returns_success() {
        let reverse_decoder = Decoder::<ReverseDecoder>::new();
        let result = reverse_decoder.crack("stac").unwrap();
        assert_eq!(result, "cats");
    }

    #[test]
    fn returns_nothing() {
        let reverse_decoder = Decoder::<ReverseDecoder>::new();
        let result = reverse_decoder.crack("");
        assert!(result.is_none());
    }
}
