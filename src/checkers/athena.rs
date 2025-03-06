use crate::{checkers::checker_result::CheckResult, cli_pretty_printing, config::get_config};
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

impl Check for Checker<Athena> {
    fn new() -> Self {
        Checker {
            // TODO: Update fields with proper values
            name: "Athena Checker",
            description: "Runs all available checkers",
            link: "",
            tags: vec!["athena", "all"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            sensitivity: Sensitivity::Medium, // Default to Medium sensitivity
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        let config = get_config();

        // If regex is specified, only run the regex checker
        if config.regex.is_some() {
            trace!("running regex");
            let regex_checker = Checker::<RegexChecker>::new().with_sensitivity(self.sensitivity);
            let regex_result = regex_checker.check(text);
            if regex_result.is_identified {
                let mut check_res = CheckResult::new(&regex_checker);
                let human_result = human_checker::human_checker(&regex_result);
                check_res.is_identified = human_result;
                check_res.text = regex_result.text;
                check_res.description = regex_result.description;
                cli_pretty_printing::success(&format!(
                    "DEBUG: Athena regex checker - human_result: {}, check_res.is_identified: {}",
                    human_result, check_res.is_identified
                ));
                return check_res;
            }
        } else {
            // Run wordlist checker first if a wordlist is provided
            if config.wordlist.is_some() {
                trace!("running wordlist checker");
                let wordlist_checker =
                    Checker::<WordlistChecker>::new().with_sensitivity(self.sensitivity);
                let wordlist_result = wordlist_checker.check(text);
                if wordlist_result.is_identified {
                    let mut check_res = CheckResult::new(&wordlist_checker);
                    let human_result = human_checker::human_checker(&wordlist_result);
                    check_res.is_identified = human_result;
                    check_res.text = wordlist_result.text;
                    check_res.description = wordlist_result.description;
                    cli_pretty_printing::success(&format!(
                        "DEBUG: Athena wordlist checker - human_result: {}, check_res.is_identified: {}",
                        human_result, check_res.is_identified
                    ));
                    return check_res;
                }
            }

            // In Ciphey if the user uses the regex checker all the other checkers turn off
            // This is because they are looking for one specific bit of information so will not want the other checkers
            // TODO: wrap all checkers in oncecell so we only create them once!
            let lemmeknow = Checker::<LemmeKnow>::new().with_sensitivity(self.sensitivity);
            let lemmeknow_result = lemmeknow.check(text);
            //println!("Text is {}", text);
            if lemmeknow_result.is_identified {
                println!("lemmeknow_result: {:?}", lemmeknow_result.is_identified);
                let mut check_res = CheckResult::new(&lemmeknow);
                let human_result = human_checker::human_checker(&lemmeknow_result);
                check_res.is_identified = human_result;
                check_res.text = lemmeknow_result.text;
                check_res.description = lemmeknow_result.description;
                cli_pretty_printing::success(&format!("DEBUG: Athena lemmeknow checker - human_result: {}, check_res.is_identified: {}", human_result, check_res.is_identified));
                return check_res;
            }

            let password = Checker::<PasswordChecker>::new().with_sensitivity(self.sensitivity);
            let password_result = password.check(text);
            if password_result.is_identified {
                let mut check_res = CheckResult::new(&password);
                let human_result = human_checker::human_checker(&password_result);
                check_res.is_identified = human_result;
                check_res.text = password_result.text;
                check_res.description = password_result.description;
                cli_pretty_printing::success(&format!("DEBUG: Athena password checker - human_result: {}, check_res.is_identified: {}", human_result, check_res.is_identified));
                return check_res;
            }

            let english = Checker::<EnglishChecker>::new().with_sensitivity(self.sensitivity);
            let english_result = english.check(text);
            if english_result.is_identified {
                let mut check_res = CheckResult::new(&english);
                let human_result = human_checker::human_checker(&english_result);
                check_res.is_identified = human_result;
                check_res.text = english_result.text;
                check_res.description = english_result.description;
                cli_pretty_printing::success(&format!(
                    "DEBUG: Athena english checker - human_result: {}, check_res.is_identified: {}",
                    human_result, check_res.is_identified
                ));
                return check_res;
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
