//! Ares is an automatic decoding and cracking tool.

mod decoders;
mod filtration_system;
mod searchers;
use crate::searchers::search::Node;

/// The main function to call which performs the cracking.
/// ```rust
/// use ares::perform_cracking;
/// perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu");
/// assert!(true, true)
/// ```
pub fn perform_cracking(text: &str) -> Option<String> {
    // Build a new search tree
    // This starts us with a node with no parents
    let search_tree = Tree::new(text.to_string());
    // Perform the search algorithm
    // It will either return a failure or success.
    search_tree.bfs()
 }

#[cfg(test)]
mod tests {
    use super::perform_cracking;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_perform_cracking_returns() {
        perform_cracking("SGVscCBJIG5lZWQgc29tZWJvZHkh");
    }
}
