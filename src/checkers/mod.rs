mod english;
mod lemmeknow;
mod checker_object;
pub mod human_checker;


trait GeneralChecker {
    fn check(&self, input: &str) -> bool;
}

pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    // import and call lemmeknow.rs
    let lemme_know_result = lemmeknow::check_lemmeknow(input);
    if lemme_know_result.is_some() {
        return true;
        // TODO get human checker to work
        /*if humanChecker::human_checker(lemmeKnowResult.unwrap()) {
            return true;
        */}

    let english_result = english::check_english(input);
    if  english_result.is_some() {
        return true;
        // TODO get human checker to work
        /*if humanChecker(englishResult.unwrap()) {
            return true;
        }*/
    }

    false
}

// test
#[cfg(test)]
mod tests {
    use crate::checkers::check;

    #[test]
    fn test_check_ip_address() {
        assert_eq!(true, check("192.168.0.1"));
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        assert_eq!(true, check("and"));
    }
}
