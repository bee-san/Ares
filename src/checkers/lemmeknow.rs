use lemmeknow::{identify_text, to_json};

pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    to_json(&identify_text(input)) != "[]"
}