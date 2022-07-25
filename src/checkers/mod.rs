use self::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
};

mod checker_result;
pub mod checker_type;
pub mod default_checker;
mod english;
pub mod human_checker;
mod lemmeknow_checker;

trait GeneralChecker {
    fn check(&self, input: &str) -> bool;
}

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
