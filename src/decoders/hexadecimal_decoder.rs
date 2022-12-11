use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::debug;
use log::trace;

///! Hexadecimal Decoder
pub struct HexadecimalDecoder;

impl Crack for Decoder<HexadecimalDecoder> {
    fn new() -> Decoder<HexadecimalDecoder> {
        Decoder {
            name: "hexadecimal",
            description: "Data is broken into 4-bit sequences, and each value (between 0 and 15 inclusively) is encoded using one of 16 symbols from the ASCII character set. Although any 16 symbols from the ASCII character set can be used, in practice the ASCII digits '0'–'9' and the letters 'A'–'F' (or the lowercase 'a'–'f') are always chosen in order to align with standard written notation for hexadecimal numbers.",
            link: "https://en.wikipedia.org/wiki/Hexadecimal#Base16_(transfer_encoding)",
            tags: vec!["hexadecimal", "hex", "base", "decoder"],
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
        trace!("Trying hexadecimal with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        let decoded_text = match hexadecimal_to_string(text) {
            x if x.is_some() => x.unwrap(),
            _ => return results,
        };

        trace!("Decoded text for hexadecimal: {:?}", decoded_text);

        if !check_string_success(&decoded_text, text) {
            debug!(
                "Failed to decode hexadecimal because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
}

/// Decodes hexadecimal to string
fn hexadecimal_to_string(hex: &str) -> Option<String> {
    // Remove all non-hexadecimal characters from the string
    let hex = hex.replace(|c: char| !c.is_ascii_hexdigit(), "");

    // Convert the hexadecimal string to a vector of bytes
    let bytes = hex.as_bytes();

    // Ensure that the vector of bytes has an even length, so it can be processed in pairs
    if bytes.len() % 2 == 1 {
        return None;
    }

    // Iterate over the vector of bytes in pairs
    let mut result = String::new();
    for pair in bytes.chunks(2) {
        // Parse the pair of bytes as a hexadecimal number and push the corresponding
        // ASCII character to the result string
        result.push(u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap() as char);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::HexadecimalDecoder;
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
    fn hexadecimal_with_no_spaces_decodes_successfully() {
        // This tests if Hexadecimal can decode Hexadecimal with no spaces successfully
        let decoder = Decoder::<HexadecimalDecoder>::new();
        let result = decoder.crack(
            "537068696e78206f6620626c61636b2071756172747a2c206a75646765206d7920766f772e",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "Sphinx of black quartz, judge my vow."
        );
    }

    #[test]
    fn hexadecimal_with_spaces_decodes_successfully() {
        // This tests if Hexadecimal can decode Hexadecimal with spaces successfully
        let decoder = Decoder::<HexadecimalDecoder>::new();
        let result = decoder.crack(
            "54 68 69 73 20 69 73 20 61 6e 20 65 78 61 6d 70 6c 65 20 6f 66 20 68 65 78 61 64 65 63 69 6d 61 6c 20 77 69 74 68 20 73 70 61 63 65 73",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "This is an example of hexadecimal with spaces"
        );
    }

    #[test]
    fn hexadecimal_tryhackme_ctf_flag() {
        // This tests if we can hex decode the hex from the tryhackme CTF challenge
        let decoder = Decoder::<HexadecimalDecoder>::new();
        let result = decoder.crack(
            "68 65 78 61 64 65 63 69 6d 61 6c 20 6f 72 20 62 61 73 65 31 36 3f",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "hexadecimal or base16?"
        );
    }

    #[test]
    fn hexadecimal_with_delimiters_decodes_successfully() {
        // This tests if Hexadecimal can decode Hexadecimal with delimiters successfully
        let decoder = Decoder::<HexadecimalDecoder>::new();
        let result = decoder.crack(
            "68;74;74;70;73;3a;2f;2f;77;77;77;2e;67;6f;6f;67;6c;65;2e;63;6f;6d",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "https://www.google.com"
        );
    }

    #[test]
    fn uppercase_hexadecimal_decodes_successfully() {
        // This tests if Hexadecimal can decode uppercase Hexadecimal successfully
        let decoder = Decoder::<HexadecimalDecoder>::new();
        let result = decoder.crack(
            "5570706572636173652068657861646563696D616C",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap()[0], "Uppercase hexadecimal");
    }

    #[test]
    fn hexadecimal_handles_panics() {
        // This tests if Hexadecimal can handle panics
        // It should return Some
        // This is because Hexadecimal can technically decode it, but it will be gibberish
        let hexadecimal_decoder = Decoder::<HexadecimalDecoder>::new();
        let result = hexadecimal_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_some());
    }

    #[test]
    fn hexadecimal_handles_panic_if_empty_string() {
        // This tests if Hexadecimal can handle an empty string
        // It should return None
        let citrix_ctx1_decoder = Decoder::<HexadecimalDecoder>::new();
        let result = citrix_ctx1_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn hexadecimal_handles_panic_if_emoji() {
        // This tests if Hexadecimal can handle an emoji
        // It should return None
        let base64_url_decoder = Decoder::<HexadecimalDecoder>::new();
        let result = base64_url_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
