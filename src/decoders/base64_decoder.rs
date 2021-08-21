///! Decode a base64 string
///! Performs error handling and returns a string
///! Call base64_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.

use log::{trace};
use crate::decoders::interface::Crack;

pub struct Base64Decoder {
    name: String,
    description: String,
    link: String,
    tags: Vec<String>,
    /// We get expectedRuntime this by bench marking the code
    expected_runtime: f32,
    /// We get popularity by eye-balling it or using the API's data
    popularity: f32,
    /// Expected success is calculated during cracking
    /// Generally this can be ignored and set to 1.0
    expected_success: f32,
    /// failure_runtime is the absolute worst case
    /// Expected is how long we expect, if it fails completely
    /// This is how long it'll take to fail.
    failure_runtime: f32,
    // normalised_entropy is the range of entropy for this
    // So base64's normalised entropy might be between 2.5 and 3
    // This allows us to decide whether it's worth decoding
    // If current text has entropy 9, it's unlikey to be base64
    normalised_entropy: Vec<f32>,
}

impl Base64Decoder {
    pub fn new() -> Self {
        Self {
            name: "base64".to_string(), // TODO clean this up
            description: "Base64 decoding schema".to_string(),
            link: "https://en.wikipedia.org/wiki/Base64".to_string(),
            tags: vec!["base64".to_string(), "decoder".to_string(), "baser".to_string()], // TODO clean this up
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 1.1,
        }
    }

    fn decode_base64_no_error_handling(text: &str) -> Result<String, base64::DecodeError> {
        // Runs the code to decode base64
        // Doesn't perform error handling, call from_base64
        let bytes = base64::decode(text)?;
        let ascii_string = String::from_utf8(bytes).unwrap();
        Ok(ascii_string)
    }

    pub fn Crack(&self, text: &str) -> Option<String> {
        trace!("Trying Base64 with text {:?}", text);
        let result = Base64Decoder::decode_base64_no_error_handling(text);
        match result {
            Ok(x) => Some(x),
            Err(_) => {
                trace!("Failed to decode base64.");
                None
                }
            }
        }
}

#[cfg(test)]
mod tests {
    use crate::decoders::base64_decoder::{Base64Decoder};

    #[test]
    fn it_works() {
        let base64_decoder = Base64Decoder::new();
        let _result = base64_decoder.Crack("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn successful_decoding(){
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.Crack("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn base64_decode_empty_string(){
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.Crack("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn base64_decode_handles_panics() {
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.Crack("hello my name is panicky mc panic face!");
        if result.is_some() {
            panic!("Decode_base64 did not return an option with Some<t>.")
            
        }
        else {
            // If we get here, the test passed
            // Because the base64_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }
}