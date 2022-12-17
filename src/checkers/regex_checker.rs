use lemmeknow::Identifier;

use super::checker_type::{Check, Checker};
use crate::{checkers::checker_result::CheckResult, config::{get_config}};
use regex::Regex;

/// The Regex Checker checks if the text matches a known Regex pattern.
/// This is the struct for it.
pub struct RegexChecker;

impl Check for Checker<RegexChecker> {
    fn new() -> Self {
        Checker {
            name: "Regex Checker",
            description: "Uses Regex to check for regex matches, useful for finding cribs.",
            link: "https://github.com/rust-lang/regex",
            tags: vec!["crib", "regex"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        // TODO put this into a lazy static so we don't generate it everytime
        let config = get_config();
        let regex_to_parse = config.regex.clone();
        let re = Regex::new(&regex_to_parse.unwrap()).unwrap();
        
        let regess_check_result = re.is_match(text);
        
        
        let mut result = CheckResult { is_identified: false, text: text.to_owned(), description: "".to_string(), checker_name: self.name, checker_description: self.description, link: self.link };

        result.checker_name = self.name;
        result.checker_description = self.description;
        result.link = self.link;
        result.text = text.to_owned();

        if regess_check_result {
            result.is_identified = true;
        }
        result
    }
}