///! This module contains CrackSuccess and CrackFailure
///

/// Every cracker returns this object which
/// Either indicates success or failure among other things.
struct CrackResult {
    /// If our checkers return success, we change this bool to True
    success: bool,
    /// Encrypted text is the text _before_ we decrypt it.
    encrypted_text: &'static str,
    /// Unencrypted text is what it looks like after.
    unencrypted_text: &'static str,
    /// Deocder is the function we used to decode the text
    decoder: &'static str,
    /// Key is optional as decoders do not use keys.
    key: Option<&'static str>,
    /// Description is a short description of the decoder
    description: &'static str,
    /// Link is a link to more info about the decoder
    link: &'static str,
}

impl CrackResult {
    /// This function returns a new CrackResult
    fn new() {
        CrackResult {
            success: false,
            encrypted_text: "",
            unencrypted_text: "",
            decoder: "",
            key: None,
            description: "",
            link: "",
        }
    }
}
