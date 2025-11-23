//! Decode a Decimal string
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use log::{debug, info, trace};

pub struct DecimalDecoder;

impl Crack for Decoder<DecimalDecoder> {
    fn new() -> Decoder<DecimalDecoder> {
        Decoder {
            name: "Decimal",
            description: "Decimal is a base-10 number system. This decoder converts space-separated decimal values to ASCII text.",
            link: "https://en.wikipedia.org/wiki/Decimal",
            tags: vec!["decimal", "base10", "decoder", "numeric"],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Decimal with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let decoded_text = decode_decimal_no_error_handling(text);

        if decoded_text.is_none() {
            debug!("Failed to decode decimal");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!("Failed check_string_success for decimal");
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);
        results.update_checker(&checker_result);

        results
    }

    fn get_tags(&self) -> &Vec<&str> { &self.tags }
    fn get_name(&self) -> &str { self.name }
    fn get_popularity(&self) -> f32 { self.popularity }
    fn get_description(&self) -> &str { self.description }
    fn get_link(&self) -> &str { self.link }
}

fn decode_decimal_no_error_handling(text: &str) -> Option<String> {
    if text.is_empty() { return None; }

    // Decimal is strictly space separated usually.
    if !text.contains(' ') && text.len() > 3 {
         // If just one number, might be a single char, but usually we expect a string.
         // Single char decoding is valid but trivial.
         // We might allow it.
         match text.parse::<u8>() {
             Ok(b) => return String::from_utf8(vec![b]).ok(),
             Err(_) => return None,
         }
    }

    let mut bytes = Vec::new();
    for part in text.split_whitespace() {
         match part.parse::<u8>() {
             Ok(b) => bytes.push(b),
             Err(_) => return None,
         }
    }

    if bytes.is_empty() { return None; }
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::DecimalDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn decimal_hello() {
        // "Hello" -> 72 101 108 108 111
        let decoder = Decoder::<DecimalDecoder>::new();
        let result = decoder.crack("72 101 108 108 111", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello");
    }

    #[test]
    fn decimal_invalid() {
        let decoder = Decoder::<DecimalDecoder>::new();
        let result = decoder.crack("256 100", &get_checker()); // 256 overflow u8
        assert!(result.unencrypted_text.is_none());
    }

    #[test]
    fn decimal_single() {
        let decoder = Decoder::<DecimalDecoder>::new();
        let result = decoder.crack("65", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A");
    }
}
