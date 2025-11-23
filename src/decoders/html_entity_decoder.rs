//! Decode HTML Entities
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;
use html_escape::decode_html_entities;

use log::{debug, info, trace};

pub struct HtmlEntityDecoder;

impl Crack for Decoder<HtmlEntityDecoder> {
    fn new() -> Decoder<HtmlEntityDecoder> {
        Decoder {
            name: "HTML Entity",
            description: "HTML entities are used to display reserved characters in HTML. This decoder converts them back to characters.",
            link: "https://en.wikipedia.org/wiki/HTML_entity",
            tags: vec!["html", "entity", "decoder", "web"],
            popularity: 0.8,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying HTML Entity with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // decode_html_entities returns Cow<str>
        let decoded_cow = decode_html_entities(text);
        let decoded_text = decoded_cow.to_string();

        if !check_string_success(&decoded_text, text) {
             // If nothing changed, it wasn't HTML encoded or didn't contain entities
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

#[cfg(test)]
mod tests {
    use super::HtmlEntityDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn html_basic() {
        let decoder = Decoder::<HtmlEntityDecoder>::new();
        let result = decoder.crack("&lt;Hello&gt;", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "<Hello>");
    }

    #[test]
    fn html_hex() {
        let decoder = Decoder::<HtmlEntityDecoder>::new();
        let result = decoder.crack("&#x41;", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A");
    }

    #[test]
    fn html_decimal() {
        let decoder = Decoder::<HtmlEntityDecoder>::new();
        let result = decoder.crack("&#65;", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A");
    }

    #[test]
    fn html_no_change() {
        let decoder = Decoder::<HtmlEntityDecoder>::new();
        let result = decoder.crack("Hello", &get_checker());
        assert!(result.unencrypted_text.is_none());
    }
}
