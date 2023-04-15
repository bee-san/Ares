use crate::checkers::checker_result::CheckResult;
use crate::storage;
use lemmeknow::Identifier;
use log::{debug, trace};

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, German};

use crate::checkers::checker_type::{Check, Checker};

/// Checks English plaintext.
pub struct QuadgramsChecker;

/// given an input, check every item in the array and return true if any of them match
impl Check for Checker<QuadgramsChecker> {
    fn new() -> Self {
        Checker {
            name: "Quadgrams Checker",
            description: "Checks for english words using quadgrams",
            link: "http://practicalcryptography.com/cryptanalysis/text-characterisation/quadgrams/",
            tags: vec!["english", "quadgrams"],
            expected_runtime: 0.1,
            /// English is the most popular language
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, input: &str) -> CheckResult {
        let mut result = CheckResult {
            is_identified: false,
            text: "idek".to_owned(),
            checker_name: self.name,
            checker_description: self.description,
            description: "quadgrams lol".to_owned(),
            link: self.link,
        };

        let languages = vec![English, German];
        let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

        if let Some(_detected_language) = detector.detect_language_of(input) {
            println!("English detected");
            result.is_identified = true;
            return result;
        } else {
            println!("Language detection failed");
            return result;
        }
    }
}