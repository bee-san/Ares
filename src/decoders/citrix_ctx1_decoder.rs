use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

///! Citrix CTX1 Decoder
pub struct CitrixCTX1Decoder;

impl Crack for Decoder<CitrixCTX1Decoder> {
    fn new() -> Decoder<CitrixCTX1Decoder> {
        Decoder {
            name: "citrix_ctx1",
            description: "Citrix CTX1 is a very old encoding that was used for encoding Citrix passwords.",
            link: "https://www.remkoweijnen.nl/blog/2012/05/13/encoding-and-decoding-citrix-passwords/",
            tags: vec!["citrix_ctx1", "citrix", "passwords", "decoder"],
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 0.1,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying citrix_ctx1 with text {:?}", text);
        let decoded_text: Option<String> = decode_citrix_ctx1(text);

        trace!("Decoded text for citrix_ctx1: {:?}", decoded_text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode citrix_ctx1 because the length is not a multiple of 4");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode citrix_ctx1 because check_string_success returned false on string {}",
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

/// Decodes Citrix CTX1
fn decode_citrix_ctx1(text: &str) -> Option<String> {
    if text.len() % 4 != 0 {
        return None;
    }

    if text.to_uppercase() != text || !text.chars().all(|c| c.is_ascii()) {
        return None;
    }

    let mut rev = text.as_bytes().to_vec();
    rev.reverse();
    let mut result = Vec::new();
    let mut temp;

    for i in (0..rev.len()).step_by(2) {
        if i + 2 >= rev.len() {
            temp = 0;
        } else {
            temp = ((rev[i + 2] - 0x41) & 0xF) ^ (((rev[i + 3] - 0x41) << 4) & 0xF0);
        }
        temp ^= (((rev[i] - 0x41) & 0xF) ^ (((rev[i + 1] - 0x41) << 4) & 0xF0)) ^ 0xA5;
        result.push(temp);
    }

    result.retain(|&x| x != 0);
    result.reverse();

    String::from_utf8(result).ok()
}

#[cfg(test)]
mod tests {
    use super::CitrixCTX1Decoder;
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
    fn test_citrix_ctx1() {
        let decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = decoder.crack(
            "MNGIKIANMEGBKIANMHGCOHECJADFPPFKINCIOBEEIFCA",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap(), "hello world");
    }
    #[test]
    fn citrix_ctx1_decode_empty_string() {
        // Citrix_ctx1 returns an empty string, this is a valid citrix_ctx1 string
        // but returns False on check_string_success
        let citrix_ctx1_decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = citrix_ctx1_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn citrix_ctx1_decode_handles_panics() {
        let citrix_ctx1_decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = citrix_ctx1_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        if result.is_some() {
            panic!("Decode_citrix_ctx1 did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the citrix_ctx1_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }

    #[test]
    fn citrix_ctx1_handle_panic_if_empty_string() {
        let citrix_ctx1_decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = citrix_ctx1_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn citrix_ctx1_work_if_string_not_citrix_ctx1() {
        // You can citrix_ctx1 decode a string that is not citrix_ctx1
        // This string decodes to:
        // ```.ée¢
        // (uÖ²```
        // https://gchq.github.io/CyberChef/#recipe=From_Base58('A-Za-z0-9%2B/%3D',true)&input=aGVsbG8gZ29vZCBkYXkh
        let citrix_ctx1_decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = citrix_ctx1_decoder
            .crack("hello good day!", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn citrix_ctx1_handle_panic_if_emoji() {
        let citrix_ctx1_decoder = Decoder::<CitrixCTX1Decoder>::new();
        let result = citrix_ctx1_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }
}
