// import storage
use crate::checkers::checker_result::CheckResult;
use crate::storage;
use lemmeknow::Identify;
// use log::{debug, info, trace}; unused imports

use crate::checkers::checker_type::{Check, Checker};

pub struct EnglishChecker;

/// given an input, check every item in the array and return true if any of them match
impl Check for Checker<EnglishChecker> {
    fn new() -> Self {
        Checker {
            name: "English Checker",
            description: "Checks for english words",
            link: "https://en.wikipedia.org/wiki/List_of_English_words",
            tags: vec!["english"],
            expected_runtime: 0.1,
            /// English is the most popular language
            popularity: 1.0,
            lemmeknow_config: Identify::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, input: &str) -> CheckResult {
        let mut plaintext_found = false;
        let mut filename = "";
        if let Some(result) = storage::DICTIONARIES
            .iter()
            .find(|(_, words)| words.contains(input))
        {
            plaintext_found = true;
            filename = result.0; // result.0 is the filename
        }

        CheckResult {
            is_identified: plaintext_found,
            text: input.to_string(),
            checker_name: self.name,
            checker_description: self.description,
            description: filename.to_string(),
            link: self.link,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::checkers::{
        checker_type::{Check, Checker},
        english::EnglishChecker,
    };

    #[test]
    fn test_check_basic() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(checker.check("preinterview").is_identified);
    }

    #[test]
    fn test_check_basic2() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(checker.check("and").is_identified);
    }

    #[test]
    fn test_check_multiple_words() {
        let checker = Checker::<EnglishChecker>::new();
        assert_eq!(checker.check("and woody").is_identified, false);
    }

    #[test]
    fn test_check_non_dictionary_word() {
        let checker = Checker::<EnglishChecker>::new();
        assert_eq!(
            checker
                .check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaBabyShark")
                .is_identified,
            false
        );
    }
}
