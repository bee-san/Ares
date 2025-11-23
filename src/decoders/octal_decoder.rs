//! Decode an Octal string
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use log::{debug, info, trace};

pub struct OctalDecoder;

impl Crack for Decoder<OctalDecoder> {
    fn new() -> Decoder<OctalDecoder> {
        Decoder {
            name: "Octal",
            description: "Octal is a base-8 number system. It uses the digits 0 to 7. This decoder converts space-separated octal values to ASCII text.",
            link: "https://en.wikipedia.org/wiki/Octal",
            tags: vec!["octal", "base8", "decoder", "numeric"],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Octal with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let decoded_text = decode_octal_no_error_handling(text);

        if decoded_text.is_none() {
            debug!("Failed to decode octal");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!("Failed check_string_success for octal");
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

fn decode_octal_no_error_handling(text: &str) -> Option<String> {
    if text.is_empty() { return None; }

    let mut bytes = Vec::new();

    // Check if space separated
    if text.contains(' ') {
        for part in text.split_whitespace() {
            // Check if valid octal
            if part.chars().any(|c| !('0'..='7').contains(&c)) {
                return None;
            }
            match u8::from_str_radix(part, 8) {
                Ok(b) => bytes.push(b),
                Err(_) => return None,
            }
        }
    } else {
        // Try chunks of 3 if length is divisible by 3 and looks like octal
        let input = text.replace('\\', ""); // basic cleanup
        if input.len() > 0 && input.len() % 3 == 0 {
             let chars: Vec<char> = input.chars().collect();
             for chunk in chars.chunks(3) {
                 let s: String = chunk.iter().collect();
                 if s.chars().any(|c| !('0'..='7').contains(&c)) {
                     return None;
                 }
                 match u8::from_str_radix(&s, 8) {
                     Ok(b) => bytes.push(b),
                     Err(_) => return None,
                 }
             }
        } else {
            return None;
        }
    }

    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::OctalDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn octal_hello() {
        let decoder = Decoder::<OctalDecoder>::new();
        let result = decoder.crack("110 145 154 154 157", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello");
    }

    #[test]
    fn octal_hello_chunked() {
        let decoder = Decoder::<OctalDecoder>::new();
        let result = decoder.crack("110145154154157", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello");
    }

    #[test]
    fn octal_invalid_digit() {
        let decoder = Decoder::<OctalDecoder>::new();
        let result = decoder.crack("110 148", &get_checker()); // 8 is invalid
        assert!(result.unencrypted_text.is_none());
    }

    #[test]
    fn octal_empty() {
        let decoder = Decoder::<OctalDecoder>::new();
        assert!(decoder.crack("", &get_checker()).unencrypted_text.is_none());
    }

    #[test]
    fn octal_utf8() {
        // 303 266 -> \xC3\xB6 -> ö
        let decoder = Decoder::<OctalDecoder>::new();
        let result = decoder.crack("303 266", &get_checker());
        if let Some(texts) = result.unencrypted_text {
            assert_eq!(texts[0], "ö");
        } else {
            panic!("Failed to decode 303 266");
        }
    }
}
