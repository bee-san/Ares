//! Decode Base32Hex
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use data_encoding::BASE32HEX_NOPAD;
use log::{debug, info, trace};

pub struct Base32HexDecoder;

impl Crack for Decoder<Base32HexDecoder> {
    fn new() -> Decoder<Base32HexDecoder> {
        Decoder {
            name: "Base32Hex",
            description: "Base32Hex is a variant of Base32 that uses the characters 0-9 and A-V. It is also known as Base32 Extended Hex.",
            link: "https://en.wikipedia.org/wiki/Base32#base32hex",
            tags: vec!["base32hex", "base32", "decoder", "base"],
            popularity: 0.7,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Base32Hex with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        let text_no_pad = text.replace('=', "");
        if let Ok(decoded_bytes) = BASE32HEX_NOPAD.decode(text_no_pad.as_bytes()) {
             if let Ok(decoded) = String::from_utf8(decoded_bytes) {
                  if check_string_success(&decoded, text) {
                      let checker_result = checker.check(&decoded);
                      results.unencrypted_text = Some(vec![decoded]);
                      results.update_checker(&checker_result);
                  }
             }
        }

        results
    }

    fn get_tags(&self) -> &Vec<&str> { &self.tags }
    fn get_name(&self) -> &str { self.name }
    fn get_popularity(&self) -> f32 { self.popularity }
    fn get_description(&self) -> &str { self.description }
    fn get_link(&self) -> &str { self.link }
}

#[cfg(test)]
mod tests {
    use super::Base32HexDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn base32hex_hello() {
        // "Hello" -> 91IMOR3F41......
        // Normal Base32 "Hello": JBSWY3DP
        // Base32: A-Z 2-7. 0->A.
        // Base32Hex: 0-9 A-V.
        // J (9) -> 9. B (1) -> 1. S (18) -> I. W (22) -> M. Y (24) -> O. 3 (27) -> R. D (3) -> 3. P (15) -> F.
        // So "Hello" in Base32Hex is "91IMOR3F"
        let decoder = Decoder::<Base32HexDecoder>::new();
        let result = decoder.crack("91IMOR3F", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello");
    }

    #[test]
    fn base32hex_padding() {
        let decoder = Decoder::<Base32HexDecoder>::new();
        let result = decoder.crack("91IMOR3F======", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello");
    }
}
