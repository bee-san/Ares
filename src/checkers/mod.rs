use self::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
    password::PasswordChecker,
    regex_checker::RegexChecker,
    wait_athena::WaitAthena,
    wordlist::WordlistChecker,
};

use gibberish_or_not::Sensitivity;

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
/// The Password checker checks if the text matches a known common password
pub mod password;
/// The Regex checker checks to see if the intended text matches the plaintext
pub mod regex_checker;
/// The WaitAthena Checker is a variant of Athena that collects all plaintexts found during the search
pub mod wait_athena;
/// The Wordlist checker checks if the text exactly matches any word in a user-provided wordlist
pub mod wordlist;

/// CheckerTypes is a wrapper enum for Checker
pub enum CheckerTypes {
    /// Wrapper for LemmeKnow Checker
    CheckLemmeKnow(Checker<LemmeKnow>),
    /// Wrapper for English Checker
    CheckEnglish(Checker<EnglishChecker>),
    /// Wrapper for Athena Checker
    CheckAthena(Checker<Athena>),
    /// Wrapper for WaitAthena Checker
    CheckWaitAthena(Checker<WaitAthena>),
    /// Wrapper for Regex
    CheckRegex(Checker<RegexChecker>),
    /// Wrapper for Password Checker
    CheckPassword(Checker<PasswordChecker>),
    /// Wrapper for Wordlist Checker
    CheckWordlist(Checker<WordlistChecker>),
}

impl Clone for CheckerTypes {
    fn clone(&self) -> Self {
        match self {
            CheckerTypes::CheckLemmeKnow(_) => {
                CheckerTypes::CheckLemmeKnow(Checker::<LemmeKnow>::new())
            }
            CheckerTypes::CheckEnglish(_) => {
                CheckerTypes::CheckEnglish(Checker::<EnglishChecker>::new())
            }
            CheckerTypes::CheckAthena(_) => {
                CheckerTypes::CheckAthena(Checker::<Athena>::new())
            }
            CheckerTypes::CheckWaitAthena(_) => {
                CheckerTypes::CheckWaitAthena(Checker::<WaitAthena>::new())
            }
            CheckerTypes::CheckRegex(_) => {
                CheckerTypes::CheckRegex(Checker::<RegexChecker>::new())
            }
            CheckerTypes::CheckPassword(_) => {
                CheckerTypes::CheckPassword(Checker::<PasswordChecker>::new())
            }
            CheckerTypes::CheckWordlist(_) => {
                CheckerTypes::CheckWordlist(Checker::<WordlistChecker>::new())
            }
        }
    }
}

impl CheckerTypes {
    /// This functions calls appropriate check function of Checker
    pub fn check(&self, text: &str) -> CheckResult {
        match self {
            CheckerTypes::CheckLemmeKnow(lemmeknow_checker) => lemmeknow_checker.check(text),
            CheckerTypes::CheckEnglish(english_checker) => english_checker.check(text),
            CheckerTypes::CheckAthena(athena_checker) => athena_checker.check(text),
            CheckerTypes::CheckWaitAthena(wait_athena_checker) => wait_athena_checker.check(text),
            CheckerTypes::CheckRegex(regex_checker) => regex_checker.check(text),
            CheckerTypes::CheckPassword(password_checker) => password_checker.check(text),
            CheckerTypes::CheckWordlist(wordlist_checker) => wordlist_checker.check(text),
        }
    }

    /// Sets the sensitivity level for gibberish detection
    pub fn with_sensitivity(&self, sensitivity: Sensitivity) -> Self {
        match self {
            CheckerTypes::CheckLemmeKnow(_checker) => {
                let mut new_checker = Checker::<LemmeKnow>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckLemmeKnow(new_checker)
            }
            CheckerTypes::CheckEnglish(_checker) => {
                let mut new_checker = Checker::<EnglishChecker>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckEnglish(new_checker)
            }
            CheckerTypes::CheckAthena(_checker) => {
                let mut new_checker = Checker::<Athena>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckAthena(new_checker)
            }
            CheckerTypes::CheckWaitAthena(_checker) => {
                let mut new_checker = Checker::<WaitAthena>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckWaitAthena(new_checker)
            }
            CheckerTypes::CheckRegex(_checker) => {
                let mut new_checker = Checker::<RegexChecker>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckRegex(new_checker)
            }
            CheckerTypes::CheckPassword(_checker) => {
                let mut new_checker = Checker::<PasswordChecker>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckPassword(new_checker)
            }
            CheckerTypes::CheckWordlist(_checker) => {
                let mut new_checker = Checker::<WordlistChecker>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckWordlist(new_checker)
            }
        }
    }

    /// Gets the current sensitivity level
    pub fn get_sensitivity(&self) -> Sensitivity {
        match self {
            CheckerTypes::CheckLemmeKnow(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckEnglish(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckAthena(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckWaitAthena(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckRegex(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckPassword(checker) => checker.get_sensitivity(),
            CheckerTypes::CheckWordlist(checker) => checker.get_sensitivity(),
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

    #[test]
    fn test_check_ip_address() {
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new());
        assert!(athena.check("test valid english sentence").is_identified);
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new());
        assert!(athena.check("exuberant").is_identified);
    }
}
