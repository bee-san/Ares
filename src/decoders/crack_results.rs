//! This module contains CrackSuccess and CrackFailure
use crate::checkers::{checker_result::CheckResult, CheckerTypes, CHECKER_MAP};
use crate::decoders::{DecoderType, DECODER_MAP};

use super::interface::Decoder;
use serde::{Deserialize, Serialize};

/// Every cracker returns this object which
/// Either indicates success or failure among other things.
#[derive(Debug, Clone, Serialize)]
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

impl<'de> Deserialize<'de> for CrackResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(unused)]
        #[derive(Deserialize)]
        struct TempCrackResult {
            pub success: bool,
            pub encrypted_text: String,
            pub unencrypted_text: Option<Vec<String>>,
            pub decoder: String,
            pub checker_name: String,
            pub checker_description: String,
            pub key: Option<String>,
            pub description: String,
            pub link: String,
        }
        let temp_cr: TempCrackResult =
            TempCrackResult::deserialize(deserializer).expect("Error deserializing CrackResult");
        let decoder = DECODER_MAP
            .get(temp_cr.decoder.as_str())
            .unwrap_or_else(|| panic!("Error during deserialization of CrackResult: could not find matching decoder for {}", temp_cr.decoder.as_str()))
            .get::<DecoderType>();
        if temp_cr.checker_name.is_empty() {
            return Ok(CrackResult {
                success: temp_cr.success,
                encrypted_text: temp_cr.encrypted_text,
                unencrypted_text: temp_cr.unencrypted_text,
                decoder: decoder.get_name(),
                checker_name: "",
                checker_description: "",
                key: None,
                description: decoder.get_description(),
                link: decoder.get_link(),
            });
        }
        let checker = CHECKER_MAP
            .get(temp_cr.checker_name.as_str())
            .unwrap_or_else(|| panic!("Error during deserialization of CrackResult: could not find matching checker for {}", temp_cr.checker_name.as_str()))
            .get::<CheckerTypes>();
        Ok(CrackResult {
            success: temp_cr.success,
            encrypted_text: temp_cr.encrypted_text,
            unencrypted_text: temp_cr.unencrypted_text,
            decoder: decoder.get_name(),
            checker_name: checker.get_name(),
            checker_description: checker.get_description(),
            key: None,
            description: decoder.get_description(),
            link: decoder.get_link(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::checkers::checker_type::{Check, Checker};
    use super::super::super::checkers::english::EnglishChecker;
    use super::super::super::checkers::CheckerTypes;
    use super::super::super::decoders::interface::Crack;
    use super::super::super::decoders::{
        base64_decoder::Base64Decoder, caesar_decoder::CaesarDecoder,
    };
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
        /// Gets the description for the current decoder
        fn get_description(&self) -> &str {
            self.description
        }
        /// Gets the link for the current decoder
        fn get_link(&self) -> &str {
            self.link
        }
    }

    #[test]
    fn get_json_success() {
        let mock_decoder = Decoder::<MockDecoder>::new();
        let crack_result = CrackResult::new(&mock_decoder, String::from("text that is encrypted"));
        let expected_str = String::from("{\"success\":false,\"encrypted_text\":\"text that is encrypted\",\"unencrypted_text\":null,\"decoder\":\"MockEncoding\",\"checker_name\":\"\",\"checker_description\":\"\",\"key\":null,\"description\":\"A mocked decoder for testing\",\"link\":\"https://en.wikipedia.org/wiki/Mock_object\"}");
        let crack_json_result = crack_result.get_json();
        assert!(crack_json_result.is_ok());
        assert_eq!(crack_json_result.unwrap(), expected_str);
    }

    #[test]
    fn deserialize_crack_result_base64() {
        let json = String::from("{\"success\":true,\"encrypted_text\":\"aGVsbG8gd29ybGQK\",\"unencrypted_text\":[\"hello world\\n\"],\"decoder\":\"Base64\",\"checker_name\":\"English Checker\",\"checker_description\":\"Uses gibberish detection to check if text is meaningful English\",\"key\":null,\"description\":\"Base64 is a group of binary-to-text encoding schemes that represent binary data in ASCII string format. Supports both standard Base64 (with +/) and URL-safe Base64 (with -_) variants.\",\"link\":\"https://en.wikipedia.org/wiki/Base64\"}");

        let checker = Checker::<EnglishChecker>::new();
        let check_result = CheckResult {
            is_identified: false,
            text: "".to_string(),
            checker_name: checker.name,
            checker_description: checker.description,
            description: "".to_string(),
            link: checker.link,
        };

        let decoder = Decoder::<Base64Decoder>::new();
        let mut expected_crack_result =
            CrackResult::new(&decoder, String::from("aGVsbG8gd29ybGQK"));
        expected_crack_result.update_checker(&check_result);
        expected_crack_result.success = true;
        expected_crack_result.unencrypted_text = Some(vec![String::from("hello world\n")]);

        let result = serde_json::from_str(json.as_str());
        assert!(result.is_ok());
        let crack_result: CrackResult = result.unwrap();
        assert_eq!(crack_result.success, expected_crack_result.success);
        assert_eq!(
            crack_result.encrypted_text,
            expected_crack_result.encrypted_text
        );
        assert!(crack_result.unencrypted_text.is_some());
        assert_eq!(
            crack_result.unencrypted_text.unwrap(),
            expected_crack_result.unencrypted_text.unwrap()
        );
        assert_eq!(crack_result.decoder, expected_crack_result.decoder);
        assert_eq!(
            crack_result.checker_name,
            expected_crack_result.checker_name
        );
        assert_eq!(
            crack_result.checker_description,
            expected_crack_result.checker_description
        );
        assert!(crack_result.key.is_none());
        assert_eq!(crack_result.description, expected_crack_result.description);
        assert_eq!(crack_result.link, expected_crack_result.link);
    }

    #[test]
    fn deserialize_crack_result_caesar() {
        let json = String::from("{\"success\":true,\"encrypted_text\":\"ifmmp uijt jt mpoh ufyu\",\"unencrypted_text\":[\"hello this is long text\"],\"decoder\":\"caesar\",\"checker_name\":\"English Checker\",\"checker_description\":\"Uses gibberish detection to check if text is meaningful English\",\"key\":null,\"description\":\"Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code or Caesar shift, is one of the simplest and most widely known encryption techniques. It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet. Uses Low sensitivity for gibberish detection.\",\"link\":\"https://en.wikipedia.org/wiki/Caesar_cipher\"}");

        let checker = Checker::<EnglishChecker>::new();
        let check_result = CheckResult {
            is_identified: false,
            text: "".to_string(),
            checker_name: checker.name,
            checker_description: checker.description,
            description: "".to_string(),
            link: checker.link,
        };

        let decoder = Decoder::<CaesarDecoder>::new();
        let mut expected_crack_result =
            CrackResult::new(&decoder, String::from("ifmmp uijt jt mpoh ufyu"));
        expected_crack_result.update_checker(&check_result);
        expected_crack_result.success = true;
        expected_crack_result.unencrypted_text =
            Some(vec![String::from("hello this is long text")]);

        let result = serde_json::from_str(json.as_str());
        assert!(result.is_ok());
        let crack_result: CrackResult = result.unwrap();
        assert_eq!(crack_result.success, expected_crack_result.success);
        assert_eq!(
            crack_result.encrypted_text,
            expected_crack_result.encrypted_text
        );
        assert!(crack_result.unencrypted_text.is_some());
        assert_eq!(
            crack_result.unencrypted_text.unwrap(),
            expected_crack_result.unencrypted_text.unwrap()
        );
        assert_eq!(crack_result.decoder, expected_crack_result.decoder);
        assert_eq!(
            crack_result.checker_name,
            expected_crack_result.checker_name
        );
        assert_eq!(
            crack_result.checker_description,
            expected_crack_result.checker_description
        );
        assert!(crack_result.key.is_none());
        assert_eq!(crack_result.description, expected_crack_result.description);
        assert_eq!(crack_result.link, expected_crack_result.link);
    }
}
