/// import general checker
use crate::checkers::checker_type::CheckerType;
use lemmeknow::Identify; 


/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
pub struct LibraryInput {
    /// The input to be decoded.
    /// Given to us by the user.
    encoded_text: String,
    /// A level of verbosity to determine.
    /// How much we print in logs.
    verbose: i32,
    /// The checker to use
    checker: CheckerType,
    /// The lemmeknow config to use
    lemmeknow_config: Identify,
}

const lemme_know_default_config: Identify = Identify {
    min_rarity: None,
    max_rarity: None,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

impl Default for LibraryInput {
    fn default() -> LibraryInput {
        LibraryInput {
            encoded_text: String::new(),
            checker: CheckerType::default(),
            verbose: 0,
            lemmeknow_config: lemme_know_default_config,
        }
    }
}