use crate::checkers::checker_result::CheckResult;
use gibberish_or_not::is_gibberish;
use lemmeknow::Identifier;

use crate::checkers::checker_type::{Check, Checker};

/// Checks English plaintext.
pub struct EnglishChecker;

/// given an input, check every item in the array and return true if any of them match
impl Check for Checker<EnglishChecker> {
    fn new() -> Self {
        Checker {
            name: "English Checker",
            description: "Uses gibberish detection to check if text is meaningful English",
            link: "https://crates.io/crates/gibberish-or-not",
            tags: vec!["english", "nlp"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        // Normalize before checking
        let text = normalise_string(text);

        let mut result = CheckResult {
            is_identified: !is_gibberish(&text),
            text: text.to_string(),
            checker_name: self.name,
            checker_description: self.description,
            description: "Gibberish detection".to_string(),
            link: self.link,
        };

        // Handle edge case of very short strings after normalization
        if text.len() < 2 {
            // Reduced from 3 since normalization may remove punctuation
            result.is_identified = false;
        }

        result
    }
}

/// Strings look funny, they might have commas, be uppercase etc
/// This normalises the string so English checker can work on it
/// In particular it:
/// Removes punctuation from the string
/// Lowercases the string
fn normalise_string(input: &str) -> String {
    // The replace function supports patterns https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html#impl-Pattern%3C%27a%3E-3
    // TODO add more punctuation
    input
        .to_ascii_lowercase()
        .chars()
        .filter(|x| !x.is_ascii_punctuation())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::checkers::english::normalise_string;
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
        assert!(
            checker
                .check("this is a valid english sentence")
                .is_identified
        );
    }

    #[test]
    fn test_check_non_dictionary_word() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(
            !checker
                .check("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaBabyShark")
                .is_identified
        );
    }

    #[test]
    fn test_check_multiple_words2() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(checker.check("preinterview hello dog").is_identified);
    }
    #[test]
    fn test_check_normalise_string_works_with_lowercasing() {
        let x = normalise_string("Hello Dear");
        assert_eq!(x, "hello dear")
    }
    #[test]
    fn test_check_normalise_string_works_with_puncuation() {
        let x = normalise_string("Hello, Dear");
        assert_eq!(x, "hello dear")
    }
    #[test]
    fn test_check_normalise_string_works_with_messy_puncuation() {
        let x = normalise_string(".He/ll?O, Dea!r");
        assert_eq!(x, "hello dear")
    }

    #[test]
    fn test_checker_works_with_puncuation_and_lowercase() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(checker.check("Prei?nterview He!llo Dog?").is_identified);
    }

    #[test]
    fn test_checker_fails_doesnt_hit_40_percent() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(
            checker
                .check("Hello Dog nnnnnnnnnnn llllllll ppppppppp gggggggg")
                .is_identified
        );
    }

    #[test]
    fn test_check_fail_single_puncuation_char() {
        let checker = Checker::<EnglishChecker>::new();
        assert!(!checker.check("#").is_identified);
    }
}
