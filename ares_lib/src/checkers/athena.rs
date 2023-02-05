use crate::{checkers::checker_result::CheckResult, config::get_config};
use lemmeknow::Identifier;
use log::trace;

use super::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    human_checker,
    lemmeknow_checker::LemmeKnow,
    regex_checker::RegexChecker,
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
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        let config = get_config();
        // Only run regex if its in the config
        if config.regex.is_some() {
            trace!("running regex");
            let regex_checker = Checker::<RegexChecker>::new();
            let regex_result = regex_checker.check(text);
            if regex_result.is_identified {
                let mut check_res = CheckResult::new(&regex_checker);
                check_res.is_identified = human_checker::human_checker(&regex_result);
                return check_res;
            }
        } else {
            // In Ciphey if the user uses the regex checker all the other checkers turn off
            // This is because they are looking for one specific bit of information so will not want the other checkers
            // TODO: wrap all checkers in oncecell so we only create them once!
            let lemmeknow = Checker::<LemmeKnow>::new();
            let lemmeknow_result = lemmeknow.check(text);
            if lemmeknow_result.is_identified {
                let mut check_res = CheckResult::new(&lemmeknow);
                check_res.is_identified = human_checker::human_checker(&lemmeknow_result);
                return check_res;
            }

            let english = Checker::<EnglishChecker>::new();
            let english_result = english.check(text);
            if english_result.is_identified {
                let mut check_res = CheckResult::new(&english);
                check_res.is_identified = human_checker::human_checker(&english_result);
                return check_res;
            }
        }

        CheckResult::new(self)
    }
}
