use self::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
};

pub mod athena;
pub mod checker_result;
pub mod checker_type;
pub mod default_checker;
pub mod english;
pub mod human_checker;
pub mod lemmeknow_checker;

pub enum CheckerTypes {
    CheckLemmeKnow(Checker<LemmeKnow>),
    CheckEnglish(Checker<EnglishChecker>),
    CheckAthena(Checker<Athena>),
}

impl CheckerTypes {
    pub fn check(&self, text: &str) -> CheckResult {
        match self {
            CheckerTypes::CheckLemmeKnow(lemmeknow_checker) => lemmeknow_checker.check(text),
            CheckerTypes::CheckEnglish(english_checker) => english_checker.check(text),
            CheckerTypes::CheckAthena(athena_checker) => athena_checker.check(text),
        }
    }
}

// pub fn check(input: &str) -> bool {
//     // Uses lemmeknow to check if any regexes match
//     // import and call lemmeknow.rs

//     let lemmeknow_result = Checker::<LemmeKnow>::new().check(input);
//     if lemmeknow_result.is_identified {
//         return human_checker::human_checker(&lemmeknow_result);
//     }

//     let english_result = Checker::<EnglishChecker>::new().check(input);
//     if english_result.is_identified {
//         return human_checker::human_checker(&english_result);
//     }

//     false
// }

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
        assert!(athena.check("192.168.0.1").is_identified);
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new());
        assert!(athena.check("and").is_identified);
    }
}
