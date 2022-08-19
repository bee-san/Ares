use crate::checkers::checker_result::CheckResult;
use lemmeknow::Identifier;

use super::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    human_checker,
    lemmeknow_checker::LemmeKnow,
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
        // TODO: wrap all checkers in oncecell so we only create them once!
        // let config = CONFIG.wait(); // wait instead of get so it waits for config being set
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

        CheckResult::new(self)
    }
}
