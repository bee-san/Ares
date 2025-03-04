/// import general checker
use lemmeknow::Identifier;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

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
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// A level of verbosity to determine.
    /// How much we print in logs.
    pub verbose: u8,
    /// The lemmeknow config to use
    #[serde(skip)]
    pub lemmeknow_config: Identifier,
    /// lemmeknow_config serialization fields
    #[serde(default)]
    pub lemmeknow_min_rarity: f32,
    /// Maximum rarity threshold for lemmeknow detection
    #[serde(default)]
    pub lemmeknow_max_rarity: f32,
    /// List of lemmeknow tags to include in detection
    #[serde(default)]
    pub lemmeknow_tags: Vec<String>,
    /// List of lemmeknow tags to exclude from detection
    #[serde(default)]
    pub lemmeknow_exclude_tags: Vec<String>,
    /// Whether to use boundaryless mode in lemmeknow detection
    #[serde(default)]
    pub lemmeknow_boundaryless: bool,
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
    /// Colourscheme hashmap
    pub colourscheme: HashMap<String, String>,
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
    min_rarity: 0.0_f32,
    max_rarity: 0.0_f32,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

/// Convert Config fields into an Identifier
fn make_identifier_from_config(config: &Config) -> Identifier {
    Identifier {
        min_rarity: config.lemmeknow_min_rarity,
        max_rarity: config.lemmeknow_max_rarity,
        tags: config.lemmeknow_tags.clone(),
        exclude_tags: config.lemmeknow_exclude_tags.clone(),
        file_support: false, // Always false as per LEMMEKNOW_DEFAULT_CONFIG
        boundaryless: config.lemmeknow_boundaryless,
    }
}

/// Update Config's Identifier field from its serialization fields
fn update_identifier_in_config(config: &mut Config) {
    config.lemmeknow_config = make_identifier_from_config(config);
}

impl Default for Config {
    fn default() -> Self {
        let mut config = Config {
            verbose: 0,
            lemmeknow_config: LEMMEKNOW_DEFAULT_CONFIG,
            lemmeknow_min_rarity: 0.0_f32,
            lemmeknow_max_rarity: 0.0_f32,
            lemmeknow_tags: vec![],
            lemmeknow_exclude_tags: vec![],
            lemmeknow_boundaryless: false,
            human_checker_on: false,
            timeout: 5,
            api_mode: true,
            regex: None,
            colourscheme: HashMap::new(),
        };

        // Set default colors
        config.colourscheme.insert(String::from("informational"), String::from("255,215,0")); // Gold yellow
        config.colourscheme.insert(String::from("warning"), String::from("255,0,0")); // Red
        config.colourscheme.insert(String::from("success"), String::from("0,255,0")); // Green
        config.colourscheme.insert(String::from("error"), String::from("255,0,0")); // Red

        config
    }
}

/// Get the path to the Ares config file
pub fn get_config_file_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push("Ares");
    fs::create_dir_all(&path).expect("Could not create Ares directory");
    path.push("config.toml");
    path
}

/// Create a default config file at the specified path
pub fn create_default_config_file() -> std::io::Result<()> {
    let config = Config::default();
    let toml_string = toml::to_string_pretty(&config)
        .expect("Could not serialize config");
    let path = get_config_file_path();
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

/// Read and parse the config file
fn read_config_file() -> std::io::Result<String> {
    let path = get_config_file_path();
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parse a TOML string into a Config struct, handling unknown keys
fn parse_toml_with_unknown_keys(contents: &str) -> Config {
    // First parse into a generic Value to check for unknown keys
    let parsed_value: toml::Value = toml::from_str(contents)
        .expect("Could not parse config file");

    // Check for unknown keys at the root level
    if let toml::Value::Table(table) = &parsed_value {
        let known_keys = vec![
            "verbose",
            "lemmeknow_min_rarity",
            "lemmeknow_max_rarity",
            "lemmeknow_tags",
            "lemmeknow_exclude_tags",
            "lemmeknow_boundaryless",
            "human_checker_on",
            "timeout",
            "api_mode",
            "regex",
            "colourscheme",
        ];
        for key in table.keys() {
            if !known_keys.contains(&key.as_str()) {
                crate::cli_pretty_printing::warning_unknown_config_key(key);
            }
        }
    }

    // Parse into Config struct
    let mut config: Config = toml::from_str(contents)
        .expect("Could not parse config file");
    update_identifier_in_config(&mut config);
    config
}

/// Get configuration from file or create default if it doesn't exist
pub fn get_config_file_into_struct() -> Config {
    let path = get_config_file_path();
    if !path.exists() {
        create_default_config_file().expect("Could not create default config file");
        return Config::default();
    }

    match read_config_file() {
        Ok(contents) => parse_toml_with_unknown_keys(&contents),
        Err(e) => {
            eprintln!("Error reading config file: {}. Using defaults.", e);
            Config::default()
        }
    }
}
