///! Reverses the input string
///! Performs error handling and returns a string
///! Call reverse_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use super::interface::Crack;
use super::interface::Decoder;

use log::trace;

/// .decoder is never used, so Rust considers this dead code
/// Really it's just a co-reference to the Decoder in `interface.rs`
#[allow(dead_code)]
pub struct ReverseDecoder {
    decoder: Decoder,
}

impl ReverseDecoder {
    pub fn new() -> Self {
        Self {
            decoder: Decoder {
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
            },
        }
    }
}

impl Crack for ReverseDecoder {
    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str) -> Option<String> {
        trace!("Running reverse string");
        if text.is_empty() {
            return None;
        }
        Some(text.chars().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoders::interface::Crack;

    #[test]
    fn returns_success() {
        let reverse_decoder = ReverseDecoder::new();
        let result = reverse_decoder.crack("stac").unwrap();
        assert_eq!(result, "cats");
    }

    #[test]
    fn returns_nothing() {
        let reverse_decoder = ReverseDecoder::new();
        let result = reverse_decoder.crack("");
        assert!(result.is_none());
    }
}
