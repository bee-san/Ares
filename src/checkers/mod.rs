mod checker_result;
mod english;
mod lemmeknow_checker;
pub mod human_checker;
pub mod default_checker;
pub mod checker_type;

trait GeneralChecker {
    fn check(&self, input: &str) -> bool;
}

pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    // import and call lemmeknow.rs
    if let Some(lemmeknow_result) = lemmeknow_checker::check_lemmeknow(input) {
        return human_checker::human_checker(&lemmeknow_result);
    };

    if let Some(english_result) = english::check_english(input) {
        return human_checker::human_checker(&english_result);
    };

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
