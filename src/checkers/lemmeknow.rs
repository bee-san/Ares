use lemmeknow::Identify;
use crate::checkers::checkerObject::{CheckObject};

pub fn check_lemmeknow(input: &str) -> Option<CheckObject> {
    // Uses lemmeknow to check if any regexes match
    let identifier = Identify::default();
    let lemmeKnowResult = identifier.identify(input);
    if !lemmeKnowResult.is_empty(){
        return Some(CheckObject{
            is_identified: true,
            text: input,
            checker: "LemmeKnow",
            // Returns a vector of matches
            description: lemmeKnowResult[0].data,
            link: "https://swanandx.github.io/lemmeknow-frontend/",
        });
    }
    None
}