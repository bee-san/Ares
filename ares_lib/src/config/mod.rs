/// import general checker
use lemmeknow::Identifier;
use once_cell::sync::OnceCell;

/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
/// The Config object is a default configuration object
/// For the entire program
/// It's access using a variable like configuration
/// ```rust
/// use ares::config::get_config;
/// let config = get_config();
/// assert_eq!(config.verbose, 0);
/// ```

pub struct Config {
    /// A level of verbosity to determine.
    /// How much we print in logs.
    pub verbose: u8,
    /// The lemmeknow config to use
    pub lemmeknow_config: Identifier,
    /// Should the human checker be on?
    /// This asks yes/no for plaintext. Turn off for API
    pub human_checker_on: bool,
    /// The timeout threshold before Ares quites
    /// This is in seconds
    pub timeout: u32,
    /// Is the program being run in API mode?
    /// This is used to determine if we should print to stdout
    /// Or return the values
    pub api_mode: bool,
    /// Regex enables the user to search for a specific regex or crib
    pub regex: Option<String>,
}

/// Cell for storing global Config
static CONFIG: OnceCell<Config> = OnceCell::new();

/// To initialize global config with custom values
pub fn set_global_config(config: Config) {
    CONFIG.set(config).ok(); // ok() used to make compiler happy about using Result
}

/// Get the global config.
/// This will return default config if the config wasn't already initialized
pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
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
            timeout: 5,
            api_mode: true,
            regex: None,
        }
    }
}
