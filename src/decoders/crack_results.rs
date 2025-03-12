//! This module contains CrackSuccess and CrackFailure
use crate::checkers::checker_result::CheckResult;

use super::interface::Decoder;
use serde::{Deserialize, Serialize};

/// Every cracker returns this object which
/// Either indicates success or failure among other things.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrackResult {
    /// If our checkers return success, we change this bool to True
    pub success: bool,
    /// Encrypted text is the text _before_ we decrypt it.
    pub encrypted_text: String,
    /// Unencrypted text is what it looks like after.
    /// if decoder failed, this will be None
    pub unencrypted_text: Option<Vec<String>>,
    /// Decoder is the function we used to decode the text
    pub decoder: &'static str,
    /// Checker which identified the text
    pub checker_name: &'static str,
    /// Description is a short description of the checker
    pub checker_description: &'static str,
    /// Key is optional as decoders do not use keys.
    pub key: Option<&'static str>,
    /// Description is a short description of the decoder
    pub description: &'static str,
    /// Link is a link to more info about the decoder
    pub link: &'static str,
}

impl CrackResult {
    /// This function returns a new CrackResult
    pub fn new<T>(decoder_used: &Decoder<T>, text: String) -> Self {
        CrackResult {
            success: false,
            encrypted_text: text,
            unencrypted_text: None,
            decoder: decoder_used.name,
            checker_name: "",
            checker_description: "",
            key: None,
            description: decoder_used.description,
            link: decoder_used.link,
        }
    }

    /// Updates the checker information
    pub fn update_checker(&mut self, checker_result: &CheckResult) {
        self.checker_name = checker_result.checker_name;
        self.checker_description = checker_result.checker_description;
        self.success = checker_result.is_identified;
    }

    /// Converts CrackResult into JSON
    pub fn get_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::checkers::CheckerTypes;
    use super::super::super::decoders::interface::Crack;
    use super::*;

    struct MockDecoder;
    impl Crack for Decoder<MockDecoder> {
        fn new() -> Decoder<MockDecoder> {
            Decoder {
                name: "MockEncoding",
                description: "A mocked decoder for testing",
                link: "https://en.wikipedia.org/wiki/Mock_object",
                tags: vec!["mock", "decoder", "base"],
                popularity: 1.0,
                phantom: std::marker::PhantomData,
            }
        }

        /// Mocked cracking function
        fn crack(&self, text: &str, _checker: &CheckerTypes) -> CrackResult {
            CrackResult::new(self, text.to_string())
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

    #[test]
    fn get_json_success() {
        let mock_decoder = Decoder::<MockDecoder>::new();
        let crack_result = CrackResult::new(&mock_decoder, String::from("encrypted text"));
        let expected_str = String::from("{\"success\":false,\"encrypted_text\":\"encrypted text\",\"unencrypted_text\":null,\"decoder\":\"MockEncoding\",\"checker_name\":\"\",\"checker_description\":\"\",\"key\":null,\"description\":\"A mocked decoder for testing\",\"link\":\"https://en.wikipedia.org/wiki/Mock_object\"}");
        let crack_json_result = crack_result.get_json();
        assert!(crack_json_result.is_ok());
        assert_eq!(crack_json_result.unwrap(), expected_str);
    }
}
