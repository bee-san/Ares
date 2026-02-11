/// Athena checker runs all other checkers and returns immediately when a plaintext is found.
/// This is the standard checker that exits early when a plaintext is found.
/// For a version that continues checking and collects all plaintexts, see WaitAthena.
use crate::{checkers::checker_result::CheckResult, config::get_config};
use gibberish_or_not::Sensitivity;
use lemmeknow::Identifier;
use log::trace;

use super::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    human_checker,
    lemmeknow_checker::LemmeKnow,
    password::PasswordChecker,
    regex_checker::RegexChecker,
    wordlist::WordlistChecker,
};

/// Athena checker runs all other checkers
pub struct Athena;

/// Helper function to check if a checker should run based on config.checkers_to_run
///
/// Returns true if:
/// - checkers_to_run is empty (all checkers enabled)
/// - checkers_to_run contains the checker name
fn should_run_checker(checker_name: &str) -> bool {
    let config = get_config();
    config.checkers_to_run.is_empty() || config.checkers_to_run.contains(&checker_name.to_string())
}

/// Run a single checker and, if it identifies plaintext, pass it through
/// the human checker. Returns `Some(CheckResult)` if the checker identified
/// the text (human may have accepted or rejected), `None` otherwise.
///
/// This eliminates the repeated 8-line pattern that was duplicated for every
/// checker in the old Athena implementation.
fn run_checker_with_human<T>(
    checker: &Checker<T>,
    checker_result: &CheckResult,
) -> Option<CheckResult> {
    if !checker_result.is_identified {
        return None;
    }

    let human_result = human_checker::human_checker(checker_result);
    trace!(
        "Human checker called from {} with result: {}",
        checker.name,
        human_result
    );

    let mut check_res = CheckResult::new(checker);
    check_res.is_identified = human_result;
    check_res.text = checker_result.text.clone();
    check_res.description = checker_result.description.clone();
    Some(check_res)
}

impl Check for Checker<Athena> {
    fn new() -> Self {
        Checker {
            name: "Athena Checker",
            description: "Runs all available checkers",
            link: "",
            tags: vec!["athena", "all"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            sensitivity: Sensitivity::Medium, // Default to Medium sensitivity
            enhanced_detector: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        trace!("Athena checker running on text: {}", text);
        let config = get_config();

        // If regex is specified, only run the regex checker (if enabled)
        if config.regex.is_some() && should_run_checker("Regex Checker") {
            trace!("running regex");
            let regex_checker = Checker::<RegexChecker>::new().with_sensitivity(self.sensitivity);
            let regex_result = regex_checker.check(text);
            if let Some(res) = run_checker_with_human(&regex_checker, &regex_result) {
                return res;
            }
        } else if config.regex.is_none() {
            // Run wordlist checker first if a wordlist is provided (and enabled)
            if config.wordlist.is_some() && should_run_checker("Wordlist Checker") {
                trace!("running wordlist checker");
                let wordlist_checker =
                    Checker::<WordlistChecker>::new().with_sensitivity(self.sensitivity);
                let wordlist_result = wordlist_checker.check(text);
                if let Some(res) = run_checker_with_human(&wordlist_checker, &wordlist_result) {
                    return res;
                }
            }

            // In Ciphey if the user uses the regex checker all the other checkers turn off
            // This is because they are looking for one specific bit of information so will not want the other checkers
            if should_run_checker("LemmeKnow Checker") {
                let lemmeknow = Checker::<LemmeKnow>::new().with_sensitivity(self.sensitivity);
                let lemmeknow_result = lemmeknow.check(text);
                if let Some(res) = run_checker_with_human(&lemmeknow, &lemmeknow_result) {
                    return res;
                }
            }

            // Password checker - note: hidden from UI but still runs if in checkers_to_run
            if should_run_checker("Password Checker") {
                let password = Checker::<PasswordChecker>::new().with_sensitivity(self.sensitivity);
                let password_result = password.check(text);
                if let Some(res) = run_checker_with_human(&password, &password_result) {
                    return res;
                }
            }

            if should_run_checker("English Checker") {
                let english = Checker::<EnglishChecker>::new().with_sensitivity(self.sensitivity);
                let english_result = english.check(text);
                if let Some(res) = run_checker_with_human(&english, &english_result) {
                    return res;
                }
            }
        }

        CheckResult::new(self)
    }

    fn with_sensitivity(mut self, sensitivity: Sensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    fn get_sensitivity(&self) -> Sensitivity {
        self.sensitivity
    }
}