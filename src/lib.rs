//! ciphey is an automatic decoding and cracking tool. https://github.com/bee-san/ciphey
// Warns in case we forget to include documentation
#![warn(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

/// The main crate for the ciphey project.
/// This provides the library API interface for ciphey.
mod api_library_input_struct;
/// Checkers is a module that contains the functions that check if the input is plaintext
pub mod checkers;
/// CLI Arg Parsing library
pub mod cli;
/// CLI Input Parser parses the input from the CLI and returns a struct.
mod cli_input_parser;
/// CLI Pretty Printing module for consistent output formatting
///
/// # Examples
/// ```
/// use ciphey::cli_pretty_printing::{success, warning};
///
/// // Print a success message
/// let success_msg = success("Operation completed successfully");
/// assert!(!success_msg.is_empty());
///
/// // Print a warning message
/// let warning_msg = warning("Please check your input");
/// assert!(!warning_msg.is_empty());
/// ```
pub mod cli_pretty_printing;
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
/// Storage module for dictionaries and invisible characters
pub mod storage;
/// Timer for internal use
mod timer;

use checkers::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    wait_athena::WaitAthena,
};
use log::debug;

use crate::{
    config::{get_config, Config},
    decoders::interface::Decoder,
};

use self::decoders::crack_results::CrackResult;

/// The main function to call which performs the cracking.
/// ```rust
/// use ciphey::perform_cracking;
/// use ciphey::config::Config;
/// let mut config = Config::default();
/// // You can set the config to your liking using the Config struct
/// // Just edit the data like below if you want:
/// config.timeout = 5;
/// config.human_checker_on = false;
/// config.verbose = 0;
/// let result = perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu", config);
/// assert!(true);
/// // The result is an Option<DecoderResult> so we need to unwrap it
/// // The DecoderResult contains the text and the path
/// // The path is a vector of CrackResults which contains the decoder used and the keys used
/// // The text is a vector of strings because some decoders return more than 1 text (Caesar)
/// // Becuase the program has returned True, the first result is the plaintext (and it will only have 1 result).
/// // This is some tech debt we need to clean up https://github.com/bee-san/ciphey/issues/130
/// assert!(result.unwrap().text[0] == "The main function to call which performs the cracking.");
/// ```
/// The human checker defaults to off in the config, but it returns the first thing it finds currently.
/// We have an issue for that here https://github.com/bee-san/ciphey/issues/129
/// ```rust
/// use ciphey::perform_cracking;
/// use ciphey::config::Config;
/// let mut config = Config::default();
/// // You can set the config to your liking using the Config struct
/// // Just edit the data like below if you want:
/// config.timeout = 0;
/// let result = perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu", config);
/// assert!(true);
/// // If the program times out, or it cannot decode the text it will return None.
/// assert!(result.is_none());
/// ```
pub fn perform_cracking(text: &str, config: Config) -> Option<DecoderResult> {
    // If top_results is enabled, ensure human_checker_on is disabled
    let mut modified_config = config;
    if modified_config.top_results {
        modified_config.human_checker_on = false;
        // Clear any previous results when starting a new cracking session
        storage::wait_athena_storage::clear_plaintext_results();
    }

    config::set_global_config(modified_config);
    let text = text.to_string();

    let initial_check_for_plaintext = check_if_input_text_is_plaintext(&text);
    if initial_check_for_plaintext.is_identified {
        debug!(
            "The input text provided to the program {} is the plaintext. Returning early.",
            text
        );
        cli_pretty_printing::return_early_because_input_text_is_plaintext();

        let mut crack_result = CrackResult::new(&Decoder::default(), text.to_string());
        crack_result.checker_name = initial_check_for_plaintext.checker_name;

        let output = DecoderResult {
            text: vec![text],
            path: vec![crack_result],
        };

        return Some(output);
    }

    // Build a new search tree
    // This starts us with a node with no parents
    // let search_tree = searchers::Tree::new(text.to_string());
    cli_pretty_printing::success(&format!(
        "DEBUG: lib.rs - Calling search_for_plaintext with text: {}",
        text
    ));
    // Perform the search algorithm
    // It will either return a failure or success.
    let result = searchers::search_for_plaintext(text);
    cli_pretty_printing::success(&format!(
        "DEBUG: lib.rs - Result from search_for_plaintext: {:?}",
        result.is_some()
    ));
    if let Some(ref res) = result {
        cli_pretty_printing::success(&format!(
            "DEBUG: lib.rs - Result has {} decoders in path",
            res.path.len()
        ));
    }
    result
}

/// Checks if the given input is plaintext or not
/// Used at the start of the program to not waste CPU cycles
fn check_if_input_text_is_plaintext(text: &str) -> CheckResult {
    let config = get_config();

    if config.top_results {
        let wait_athena_checker = Checker::<WaitAthena>::new();
        wait_athena_checker.check(text)
    } else {
        let athena_checker = Checker::<Athena>::new();
        athena_checker.check(text)
    }
}

/// DecoderResult is the result of decoders
#[derive(Debug, Clone)]
pub struct DecoderResult {
    /// The text we have from the decoder, as a vector
    /// because the decoder might return more than 1 text (caesar)
    pub text: Vec<String>,
    /// The list of decoders we have so far
    /// The CrackResult contains more than just each decoder, such as the keys used
    /// or the checkers used.
    pub path: Vec<CrackResult>,
}

/// Creates a default DecoderResult with Default as the text / path
impl Default for DecoderResult {
    fn default() -> Self {
        DecoderResult {
            text: vec!["Default".to_string()],
            path: vec![CrackResult::new(&Decoder::default(), "Default".to_string())],
        }
    }
}

/// Lets us create a new decoderResult with given text
impl DecoderResult {
    /// It's only used in tests so it thinks its dead code
    fn _new(text: &str) -> Self {
        DecoderResult {
            text: vec![text.to_string()],
            path: vec![CrackResult::new(&Decoder::default(), "Default".to_string())],
        }
    }
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
        // Since we are exiting early the path should be of length 1, which is 1 check (the Athena check)
        assert!(result.unwrap().path.len() == 1);
    }

    #[ignore]
    #[test]
    // Previously this would decode to `Fchohs as 13 dzoqsg!` because the English checker wasn't that good
    // This test makes sure we can decode it okay
    // TODO: Skipping this test because the English checker still isn't good.
    fn test_successfully_decode_caesar() {
        let config = Config::default();
        let result = perform_cracking("Ebgngr zr 13 cynprf!", config);
        // We return None since the input is the plaintext
        assert!(result.unwrap().text[0] == "Rotate me 13 places!");
    }

    #[test]
    fn test_successfully_inputted_plaintext() {
        let config = Config::default();
        let result = perform_cracking("Hello, World!", config);
        // We return None since the input is the plaintext
        let res_unwrapped = result.unwrap();
        assert!(&res_unwrapped.text[0] == "Hello, World!");
        // Since our input is the plaintext we did not decode it
        // Therefore we return with the default decoder
        assert!(res_unwrapped.path[0].decoder == "Default decoder");
    }
}
