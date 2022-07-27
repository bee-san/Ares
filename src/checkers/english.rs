// import storage
use crate::checkers::checker_result::CheckResult;
use crate::storage;
use lemmeknow::Identify;
use log::{debug, trace};

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
            _phatom: std::marker::PhantomData,
        }
    }

    fn check(&self, input: &str) -> CheckResult {
        trace!("Checking English for sentence {}", input);
        // If 50% of the words are in the english list, then we consider it english.
        // This is the threshold at which we consider it english.
        // TODO: Do we want to put this into a config somewhere?
        const PLAINTEXT_DETECTION_PERCENTAGE: f64 = 0.0;
        let mut words_found: f64 = 0.0;

        let mut plaintext_found = false;
        // TODO: Change this when the below bugs are fixed.
        let filename = "English.txt";

        // loop through all the words in the input
        for word in input.split_whitespace() {
            // if the word is in the english list, then we consider it english
            // TODO: I think the below function iterates through each dictionary in turn.
            // Which means it'll try English.txt, then rockyou.txt etc
            // This is inefficient and makes it harder to compute what dictionary the word came from.
            // We should probably just use a single dictionary and assign the filenames to the values in the dictionary.
            // Like {"hello": "English.txt"} etc.
            // If we're using muiltiple dictionaries we may also have duplicated words which is inefficient.
            if storage::DICTIONARIES.iter().find(|(_, words)| words.contains(word)).is_some() {
                trace!("Found word {} in English", word);
                words_found += 1.0;
            }

            trace!("Checking word {} with words_found {} and input length: {}", word, words_found, input.len());
            // TODO: There is a bug here. We are comparing how many words found to how many characters there are in bytes.
            // This means the numbers don't exactly match up. There may be some cases where this will break.
            // We are also typecasting to f64 instead of usize, which costs CPU cycles.
            if words_found / input.len() as f64 > PLAINTEXT_DETECTION_PERCENTAGE {
                debug!("Found {} words in {}", words_found, input);
                debug!("Returning from English chekcer successfully with {}", input);
                plaintext_found = true;
                break;
            }
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
        assert_eq!(checker.check("and woody").is_identified, true);
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

    #[test]
    fn test_check_multiple_words2() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(checker.check("preinterview hello dog").is_identified);
    }
}
