// import storage
use crate::checkers::checker_result::CheckResult;
use crate::storage;
use log::{debug, info, trace};

use crate::checkers::checker_type::{CheckerType, Check};

pub struct EnglishChecker {
    english_checker: CheckerType,
}

impl EnglishChecker {
    pub fn new() -> Self {
        Self {
            english_checker: CheckerType {
                name: "English Checker",
                description: "This checker checks if the text is english looping over a dictionary",
                link: "https://en.wikipedia.org/wiki/English_language",
                tags: vec!["english", "dictionary"],
                /// Expected runtime is higher as this is a O(n) checker
                expected_runtime: 0.01,
                /// Popularity is max because English is the most popular
                /// Plaintext language in the world.
                popularity: 1.0,
                ..Default::default()
            }
        }
    }
}

/// given an input, check every item in the array and return true if any of them match
impl Check for EnglishChecker {
    fn check(&self, input: &'static str, checker: CheckerType) -> CheckResult {
        let mut plaintext_found = false;
        if let Some(result) = storage::DICTIONARIES
        .iter()
        .find(|(_, words)| words.contains(input))
        {
            plaintext_found = true;
        }

        CheckResult{
            is_identified: plaintext_found,
            text: input,
            checker,
        }
    }
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
