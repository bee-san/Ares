//! The search algorithm decides what encryptions to do next
//! And also runs the decryption modules
//! Click here to find out more:
//! https://broadleaf-angora-7db.notion.site/Search-Nodes-Edges-What-should-they-look-like-b74c43ca7ac341a1a5cfdbeb84a7eef0

use crate::filtration_system::filter_and_get_decoders;
use lemmeknow::{identify_text, to_json};
mod bfs;

/*pub struct Tree <'a> {
    // Wrap in a box because
    // https://doc.rust-lang.org/error-index.html#E0072
    parent: &'a Box<Option<Tree<'a>>>,
    value: String
}*/

/// Performs the search algorithm.
///
/// When we perform the decryptions, we will get a vector of Some<String>
/// We need to loop through these and determine:
/// 1. Did we reach our exit condition?
/// 2. If not, create new nodes out of them and add them to the queue.
/// We can return an Option? An Enum? And then match on that
/// So if we return CrackSuccess we return
/// Else if we return an array, we add it to the children and go again.
pub fn search_for_plaintext(input: &str) -> Option<String> {
    // Change this to select which search algorithm we want to use.
    bfs::bfs(input)
}

// Performs the decodings by getting all of the decoders
// and calling `.run` which in turn loops through them and calls
// `.crack()`.
fn perform_decoding(text: &str) -> Vec<Option<String>> {
    let decoders = filter_and_get_decoders();
    decoders.run(text)
}

// https://github.com/bee-san/Ares/pull/14/files#diff-b8829c7e292562666c7fa5934de7b478c4a5de46d92e42c46215ac4d9ff89db2R37
fn exit_condition(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    to_json(&identify_text(input)) != "[]"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_condition_succeeds() {
        let result = exit_condition("https://www.google.com");
        assert!(result);
    }
    #[test]
    fn exit_condition_fails() {
        let result = exit_condition("vjkrerkdnxhrfjekfdjexk");
        assert!(!result);
    }

    #[test]
    fn perform_decoding_succeeds() {
        let result = perform_decoding("aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbQ==");
        assert!(!result.is_empty());
        assert!(result.get(0).is_some());
        //TODO assert that the plaintext is correct by looping over the vector
    }
    #[test]
    fn perform_decoding_succeeds_empty_string() {
        // Some decoders like base64 return even when the string is empty.
        let result = perform_decoding("");
        assert!(!result.is_empty());
    }
}
