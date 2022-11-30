//! Ares is an automatic decoding and cracking tool. https://github.com/bee-san/ares
// Warns in case we forget to include documentation
#![warn(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

/// The main crate for the Ares project.
/// This provides the library API interface for Ares.
mod api_library_input_struct;
/// Checkers is a module that contains the functions that check if the input is plaintext
pub mod checkers;
/// CLI Arg Parsing library
pub mod cli;
/// CLI Input Parser parses the input from the CLI and returns a struct.
mod cli_input_parser;
/// The Config module enables a configuration module
/// Like a global API to access config details
pub mod config;
/// Decoders are the functions that actually perform the decodings.
pub mod decoders;
/// The filtration system builds what decoders to use at runtime
/// By default it will use them all.
mod filtration_system;
/// The searcher is the thing which searches for the plaintext
/// It is the core of the program.
mod searchers;
/// The storage module contains all the dictionaries and provides
/// storage of data to our decoderrs and checkers.
mod storage;
/// Timer for internal use
mod timer;

use checkers::{checker_type::{Checker, Check}, athena::Athena, CheckerTypes};
use log::debug;

use crate::config::Config;

use self::decoders::crack_results::CrackResult;
/// The main function to call which performs the cracking.
/// ```rust
/// use ares::perform_cracking;
/// use ares::config::Config;
///
/// perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu", Config::default());
/// assert!(true)
/// ```
pub fn perform_cracking(text: &str, config: Config) -> Option<Text> {
    if check_if_input_text_is_plaintext(text){
        debug!("The input text provided to the program {} is the plaintext. Returning early.", text);
        return_early_becauseinput_text_is_plaintext();
        return None
    }
pub fn perform_cracking(text: &str, config: Config) -> Option<DecoderResult> {
    // Build a new search tree
    // This starts us with a node with no parents
    // let search_tree = searchers::Tree::new(text.to_string());
    // Perform the search algorithm
    // It will either return a failure or success.
    config::set_global_config(config);
    searchers::search_for_plaintext(text)
}

fn check_if_input_text_is_plaintext(text: &str) -> bool {
    let athena_checker = Checker::<Athena>::new();
    athena_checker.check(text).is_identified
}

fn return_early_becauseinput_text_is_plaintext(){
    println!("Your input text is the plaintext 🥳")
}

/// DecoderResult is the result of decoders
#[derive(Debug)]
pub struct DecoderResult {
    /// The text we have from the decoder, as a vector
    /// because the decoder might return more than 1 text (caesar)
    pub text: Vec<String>,
    /// The list of decoders we have so far
    /// The CrackResult contains more than just each decoder, such as the keys used
    /// or the checkers used.
    pub path: Vec<CrackResult>,
}

#[cfg(test)]
mod tests {
    use super::perform_cracking;
    use crate::config::Config;

    #[test]
    fn test_perform_cracking_returns() {
        let config = Config::default();
        perform_cracking("SGVscCBJIG5lZWQgc29tZWJvZHkh", config);
    }

    #[test]
    fn test_perform_cracking_returns_successful() {
        // this will work after english checker can identify "CANARY: hello"
        // let result = perform_cracking("Q0FOQVJZOiBoZWxsbw==");
        // assert!(result.is_some());
        // assert!(result.unwrap() == "CANARY: hello")
        let config = Config::default();
        let result = perform_cracking("b2xsZWg=", config);
        assert!(result.is_some());
        assert!(result.unwrap().text[0] == "hello");
    }
    #[test]
    fn test_perform_cracking_returns_failure() {
        let config = Config::default();
        let result = perform_cracking("", config);
        assert!(result.is_none());
    }

    #[test]
    fn test_perform_cracking_returns_successful_base64_reverse() {
        let config = Config::default();
        let result = perform_cracking("aGVsbG8gdGhlcmUgZ2VuZXJhbA==", config);
        assert!(result.is_some());
        assert!(result.unwrap().text[0] == "hello there general")
    }

    #[test]
    fn test_early_exit_if_input_is_plaintext() {
        let config = Config::default();
        let result = perform_cracking("192.168.0.1", config);
        // We return None since the input is the plaintext
        assert!(result.is_none());
    }
}
