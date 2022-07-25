//! Ares is an automatic decoding and cracking tool.
//! https://github.com/bee-san/ares

// Warns in case we forget to include documentation
#![warn(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

mod api_library_input_struct;
mod checkers;
mod cli_input_parser;
pub mod decoders;
mod filtration_system;
mod searchers;
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
        let result = perform_cracking("Q0FOQVJZOiBoZWxsbw==");
        assert!(result.is_some());
        assert!(result.unwrap() == "CANARY: hello")
    }
    #[test]
    fn test_perform_cracking_returns_failure() {
        let result = perform_cracking("");
        assert!(result.is_none());
    }

    #[test]
    fn test_perform_cracking_returns_successful_base64_reverse() {
        let result = perform_cracking("b2xsZWg=");
        assert!(result.is_some());
        assert!(result.unwrap() == "hello")
    }
}
