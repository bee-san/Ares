use self::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
};

/// The checkerResult struct is used to store the results of a checker.
mod checker_result;
/// This is the base checker that all other checkers inherit from.
pub mod checker_type;
/// The default checker we use which simply calls all other checkers in order.
pub mod default_checker;
/// The English Checker is a checker that checks if the input is English
mod english;
/// The Human Checker asks humans if the expected plaintext is real plaintext
pub mod human_checker;
/// The LemmeKnow Checker checks if the text matches a known Regex pattern.
mod lemmeknow_checker;

/// The default checker we use which simply calls all other checkers in order.
trait GeneralChecker {
    /// Checks the given text to see if its plaintext
    fn check(&self, input: &str) -> bool;
}

/// The main function to call which performs the checking.
/// This function calls all other checkers in order to check if the input is plaintext.
/// ```rust
/// use ares::checkers::check;
/// let plaintext = check("hello");
/// let plaintext_ip = check("192.168.0.1");
/// assert!(plaintext);
/// assert!(plaintext_ip);
/// ```
pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    // import and call lemmeknow.rs

    let lemmeknow_result = Checker::<LemmeKnow>::new().check(input);
    if lemmeknow_result.is_identified {
        return human_checker::human_checker(&lemmeknow_result);
    }

    let english_result = Checker::<EnglishChecker>::new().check(input);
    if english_result.is_identified {
        return human_checker::human_checker(&english_result);
    }

    false
}

// test
#[cfg(test)]
mod tests {
    use crate::checkers::check;

    #[test]
    fn test_check_ip_address() {
        assert!(check("192.168.0.1"));
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        assert!(check("and"));
    }
}
