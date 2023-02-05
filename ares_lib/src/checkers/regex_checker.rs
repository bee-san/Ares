use lemmeknow::Identifier;

use super::checker_type::{Check, Checker};
use crate::{checkers::checker_result::CheckResult, config::get_config};
use log::trace;
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
        trace!("Checking {} with regex", text);
        // TODO put this into a lazy static so we don't generate it everytime
        let config = get_config();
        let regex_to_parse = config.regex.clone();
        let re = Regex::new(&regex_to_parse.unwrap()).unwrap();

        let regex_check_result = re.is_match(text);
        let mut plaintext_found = false;
        let printed_name = format!("Regex matched: {}", re);
        if regex_check_result {
            plaintext_found = true;
        }

        CheckResult {
            is_identified: plaintext_found,
            text: text.to_string(),
            checker_name: self.name,
            checker_description: self.description,
            description: printed_name,
            link: self.link,
        }
    }
}
