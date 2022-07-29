//! Ares is an automatic decoding and cracking tool.
//! https://github.com/bee-san/ares

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
/// CLI Input Parser parses the input from the CLI and returns a struct.
mod cli_input_parser;
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

/// The main function to call which performs the cracking.
/// ```rust
/// use ares::perform_cracking;
/// perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu");
/// assert!(true)
/// ```
pub fn perform_cracking(text: &str) -> Option<String> {
    // Build a new search tree
    // This starts us with a node with no parents
    // let search_tree = searchers::Tree::new(text.to_string());
    // Perform the search algorithm
    // It will either return a failure or success.
    searchers::search_for_plaintext(text)
}

#[cfg(test)]
mod tests {
    use super::perform_cracking;

    #[test]
    fn test_perform_cracking_returns() {
        perform_cracking("SGVscCBJIG5lZWQgc29tZWJvZHkh");
    }

    #[test]
    fn test_perform_cracking_returns_successful() {
        // this will work after english checker can identify "CANARY: hello"
        // let result = perform_cracking("Q0FOQVJZOiBoZWxsbw==");
        // assert!(result.is_some());
        // assert!(result.unwrap() == "CANARY: hello")

        let result = perform_cracking("b2xsZWg=");
        assert!(result.is_some());
        assert!(result.unwrap() == "hello");
    }
    #[test]
    fn test_perform_cracking_returns_failure() {
        let result = perform_cracking("");
        assert!(result.is_none());
    }

    #[test]
    fn test_perform_cracking_returns_successful_base64_reverse() {
        let result = perform_cracking("aGVsbG8gdGhlcmUgZ2VuZXJhbA==");
        assert!(result.is_some());
        assert!(result.unwrap() == "hello there general")
    }
}
