// import storage
use crate::checkers::checker_result::CheckResult;
use crate::storage;
use log::{debug, info, trace};

use crate::checkers::checker_type::CheckerType;

pub struct EnglishChecker {
    pub checker_type: CheckerType,
}

impl EnglishChecker {
    pub fn new() -> EnglishChecker {
        EnglishChecker {
            name: "English Checker",
            description: "This checker checks if the text is english looping over a dictionary",
            link: "https://en.wikipedia.org/wiki/English_language",
            tags: vec!["english", "dictionary"],
            /// Expected runtime is higher as this is a O(n) checker
            expected_runtime: 0.3,
            /// Popularity is max because English is the most popular
            /// Plaintext language in the world.
            popularity: 1.0,
            ..Default::default()
        }
    }
}

// given an input, check every item in the array and return true if any of them match
pub fn check_english(input: &str) -> Option<CheckResult> {
    if let Some(result) = storage::DICTIONARIES
        .iter()
        .find(|(_, words)| words.contains(input))
    {
        // result.0 is filename
        return Some(CheckResult {
            is_identified: true,
            text: input,
            checker: "Dictionary",
            description: result.0.to_string(),
            link: "https://en.wikipedia.org/wiki/List_of_English_words",
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::checkers::english::check_english;

    #[test]
    fn test_check_basic() {
        assert!(check_english("preinterview").is_some());
    }

    #[test]
    fn test_check_basic2() {
        assert!(check_english("and").is_some());
    }

    #[test]
    fn test_check_multiple_words() {
        assert!(check_english("and woody").is_none());
    }

    #[test]
    fn test_check_non_dictionary_word() {
        assert!(
            check_english("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaBabyShark").is_none()
        );
    }
}
