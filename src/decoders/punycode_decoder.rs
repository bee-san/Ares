//! Decode Punycode
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use log::{debug, info, trace};

pub struct PunycodeDecoder;

impl Crack for Decoder<PunycodeDecoder> {
    fn new() -> Decoder<PunycodeDecoder> {
        Decoder {
            name: "Punycode",
            description: "Punycode is a representation of Unicode with the limited ASCII character subset used for Internet hostnames.",
            link: "https://en.wikipedia.org/wiki/Punycode",
            tags: vec!["punycode", "idna", "decoder", "dns"],
            popularity: 0.4,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Punycode with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        let decoded_text = decode_punycode_no_error_handling(text);

        if decoded_text.is_none() {
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
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

fn decode_punycode_no_error_handling(text: &str) -> Option<String> {
    if text.is_empty() { return None; }

    // Punycode often starts with xn-- but the decoder might expect raw punycode
    // punycode crate decode function expects the encoded string WITHOUT xn-- prefix usually
    // or checks for it?
    // Let's check typical usage. punycode::decode takes a str.

    let input = if text.starts_with("xn--") {
        &text[4..]
    } else {
        text
    };

    match punycode::decode(input) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::PunycodeDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn punycode_mnchen() {
        // München -> Mnchen-3ya
        let decoder = Decoder::<PunycodeDecoder>::new();
        let result = decoder.crack("Mnchen-3ya", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "München");
    }

    #[test]
    fn punycode_with_prefix() {
        let decoder = Decoder::<PunycodeDecoder>::new();
        let result = decoder.crack("xn--Mnchen-3ya", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "München");
    }
}
