use lemmeknow::Identify;

pub fn check_lemmeknow(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    let identifier = Identify::default();
    !identifier.identify(input).is_empty()
}
