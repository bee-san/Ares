use lemmeknow::{identify_text, to_json};

pub fn CheckLemmeKnow(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    !identify_text(input).is_empty()
}