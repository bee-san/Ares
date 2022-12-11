use crate::checkers::checker_result::CheckResult;

use super::interface::Decoder;

///! This module contains CrackSuccess and CrackFailure
///

/// Every cracker returns this object which
/// Either indicates success or failure among other things.
#[derive(Debug, Clone)]
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
}
