/// import general checker
use lemmeknow::Identifier;
use once_cell::sync::OnceCell;

/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
/// The Config object is a default configuration object
/// For the entire program
/// It's access using a variable like configuration
/// ```rust
/// use ares::config::{CONFIG, Config, set_global_config};
/// set_global_config(Config::default());
/// // Assert that the CONFIG has an offline mode
/// assert!(!CONFIG.wait().offline_mode);

pub struct Config {
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

/// Global config
pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[allow(unused_must_use)]
/// To initialize global config
pub fn set_global_config(config: Config) {
    CONFIG.set(config);
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
            verbose: 0,
            lemmeknow_config: LEMMEKNOW_DEFAULT_CONFIG,
            human_checker_on: false,
            offline_mode: false,
        }
    }
}
