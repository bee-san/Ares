/// import general checker
use lemmeknow::Identifier;

/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
/// The Config object is a default configuration object
/// For the entire program 
/// It's access using a variable like configuration 
/// ```rust
/// use ares::config::CONFIG;
/// // Assert that the CONFIG has an offline modetest_it_works
/// assert!(!CONFIG.offline_mode);

pub struct Config {
    /// The input to be decoded.
    /// Given to us by the user.
    pub encoded_text: String,
    /// A level of verbosity to determine.
    /// How much we print in logs.
    pub verbose: i32,
    /// The lemmeknow config to use
    pub lemmeknow_config: Identifier,
    /// Should the human checker be on?
    /// This asks yes/no for plaintext. Turn off for API
    pub human_checker_on: bool,
    /// Is the program being run in offline mode?
    pub offline_mode: bool,

}

/// Creates a default lemmeknow config
const LEMMEKNOW_DEFAULT_CONFIG: Identifier = Identifier {
    min_rarity: 0.0,
    max_rarity: 0.0,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

impl Default for Config {
    fn default() -> Self {
        Config {
            encoded_text: String::new(),
            // this will be of type Checker<DefaultChecker>
            verbose: 0,
            lemmeknow_config: LEMMEKNOW_DEFAULT_CONFIG,
            human_checker_on: false,
            offline_mode: false,
        }
    }
}
