use lemmeknow::Identify;

use super::{checker_type::{Checker, Check}, checker_result::CheckResult};

/// The default checker is used to check if the text is plaintext
/// Based on what the Ares team has found to be the best checker.

pub struct DefaultChecker;

impl Check for Checker<DefaultChecker> {
    fn new() -> Self {
        Checker {
            name: "Template checker",
            description: "This is a default template checker. If you're seeing this, it's an error. Please contact us on Discord http://discord.skerritt.blog",
            link: "http://discord.skerritt.blog",
            tags: vec![],
            expected_runtime: 0.0,
            popularity: 0.0,
            lemmeknow_config: Identify::default(),
            _phatom: std::marker::PhantomData,
        }
    }

    fn check(&self, _text: &str) -> CheckResult {
        CheckResult::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::checkers::{
        checker_type::{Checker, Check}, default_checker::DefaultChecker, checker_result::CheckResult,
    };

    #[test]
    fn default_checker_works() {
        let checker = Checker::<DefaultChecker>::new();
        let checker_result = CheckResult::new(&checker);
        assert_eq!(checker_result.is_identified, false);
    }
}