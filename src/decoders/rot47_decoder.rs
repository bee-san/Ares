use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{info, trace};

///! ROT47 Decoder
pub struct ROT47Decoder;

impl Crack for Decoder<ROT47Decoder> {
    fn new() -> Decoder<ROT47Decoder> {
        Decoder {
            name: "ROT47",
            description: "ROT47 is a derivative of ROT13 which, in addition to scrambling the basic letters, treats numbers and common symbols. Instead of using the sequence A–Z as the alphabet, ROT47 uses a larger set of characters from the common character encoding known as ASCII. Specifically, the 7-bit printable characters, excluding space, from decimal 33 '!' through 126 '~', 94 in total.",
            link: "https://en.wikipedia.org/wiki/ROT13#Variants",
            tags: vec!["rot47", "substitution", "decoder", "reciprocal"],
            popularity: 1.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying rot47 with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let mut decoded_strings = Vec::new();

        // loops through all possible shifts up to 94
        for shift in 1..94 {
            let decoded_text = rot47_to_alphabet(text, shift);
            decoded_strings.push(decoded_text);
            let borrowed_decoded_text = &decoded_strings[decoded_strings.len() - 1];
            if !check_string_success(borrowed_decoded_text, text) {
                info!(
                    "Failed to decode rot47 because check_string_success returned false on string {}. This means the string is 'funny' as it wasn't modified.",
                    borrowed_decoded_text
                );
                return results;
            }
            let checker_result = checker.check(borrowed_decoded_text);
            // If checkers return true, exit early with the correct result
            if checker_result.is_identified {
                trace!("Found a match with rot47 shift {}", shift);
                results.unencrypted_text = Some(vec![borrowed_decoded_text.to_string()]);
                results.update_checker(&checker_result);
                return results;
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

/// Maps rot47 to the alphabet (up to ROT94 with the ROT47 alphabet)
fn rot47_to_alphabet(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        let mut c = c as u8;
        if c >= 33 && c <= 126 {
            c = ((c - 33 + shift) % 94) + 33;
        }
        result.push(c as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::ROT47Decoder;
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
    fn rot47_decodes_successfully() {
        // This tests if ROT47 can decode ROT47 successfully
        // Shift is 47, but due to shift 15 resulting in plaintext too
        // we check for shift 15's result instead
        let rot47_decoder = Decoder::<ROT47Decoder>::new();
        let result = rot47_decoder.crack(
            "$A9:?I @7 3=24< BF2CEK[ ;F586 >J G@H",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "3PHINX OF BLACK QUARTZj JUDGE MY VOW"
        );
    }

    #[test]
    fn rot47_handles_panic_if_empty_string() {
        // This tests if ROT47 can handle an empty string
        // It should return None
        let rot47_decoder = Decoder::<ROT47Decoder>::new();
        let result = rot47_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
