use self::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
};

use crate::config::Config;

/// The default checker we use which simply calls all other checkers in order.
pub mod athena;
/// The checkerResult struct is used to store the results of a checker.
pub mod checker_result;
/// This is the base checker that all other checkers inherit from.
pub mod checker_type;
/// The default checker we use which simply calls all other checkers in order.
pub mod default_checker;
/// The English Checker is a checker that checks if the input is English
pub mod english;
/// The Human Checker asks humans if the expected plaintext is real plaintext
pub mod human_checker;
/// The LemmeKnow Checker checks if the text matches a known Regex pattern.
pub mod lemmeknow_checker;

/// CheckerTypes is a wrapper enum for Checker
pub enum CheckerTypes<'a> {
    /// Wrapper for LemmeKnow Checker
    CheckLemmeKnow(Checker<LemmeKnow>, Config),
    /// Wrapper for English Checker
    CheckEnglish(Checker<EnglishChecker>, Config),
    /// Wrapper for Athena Checker
    CheckAthena(Checker<Athena>, &'a Config),
}

impl CheckerTypes<'_> {
    /// This functions calls appropriate check function of Checker
    pub fn check(&self, text: &str) -> CheckResult {
        match self {
            CheckerTypes::CheckLemmeKnow(lemmeknow_checker, config) => lemmeknow_checker.check(text, config),
            CheckerTypes::CheckEnglish(english_checker, config) => english_checker.check(text, config),
            CheckerTypes::CheckAthena(athena_checker, config) => athena_checker.check(text, config),
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::checkers::{
        athena::Athena,
        checker_type::{Check, Checker},
        CheckerTypes,
    };
    use crate::config::Config;

    #[test]
    fn test_check_ip_address() {
        // new config 
        let config = Config::default();
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new(), &config);
        assert!(athena.check("192.168.0.1").is_identified);
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        let config = Config::default();
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new(), &config);
        assert!(athena.check("and").is_identified);
    }
}
