/// import general checker
use crate::checkers::{checker_type::{Checker, Check}, default_checker::DefaultChecker};
use lemmeknow::Identify;

/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
pub struct LibraryInput<Type> {
    /// The input to be decoded.
    /// Given to us by the user.
    pub encoded_text: String,
    /// A level of verbosity to determine.
    /// How much we print in logs.
    pub verbose: i32,
    /// The checker to use
    pub checker: Checker<Type>,
    /// The lemmeknow config to use
    pub lemmeknow_config: Identify,
}

const LEMMEKNOW_DEFAULT_CONFIG: Identify = Identify {
    min_rarity: None,
    max_rarity: None,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

impl Default for LibraryInput<DefaultChecker> {
    fn default() -> Self {
        LibraryInput {
            encoded_text: String::new(),
            // this will be of type Checker<DefaultChecker>
            checker: Checker::new(),
            verbose: 0,
            lemmeknow_config: LEMMEKNOW_DEFAULT_CONFIG,
        }
    }
}
