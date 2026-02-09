/// import general checker
use lemmeknow::Identifier;
use memmap2::Mmap;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};
use std::path::Path;

/// Library input is the default API input
/// The CLI turns its arguments into a LibraryInput struct
/// The Config object is a default configuration object
/// For the entire program
/// It's access using a variable like configuration
/// ```rust
/// use ciphey::config::get_config;
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
    /// The timeout threshold before ciphey quits
    /// This is in seconds
    pub timeout: u32,
    /// Whether to collect all plaintexts until timeout expires
    /// instead of exiting after finding the first valid plaintext
    pub top_results: bool,
    /// Is the program being run in API mode?
    /// This is used to determine if we should print to stdout
    /// Or return the values
    pub api_mode: bool,
    /// Regex enables the user to search for a specific regex or crib
    pub regex: Option<String>,
    /// Path to the wordlist file. Will be overridden by CLI argument if provided.
    pub wordlist_path: Option<String>,
    /// Wordlist data structure (loaded from file). CLI takes precedence if both config and CLI specify a wordlist.
    #[serde(skip)]
    pub wordlist: Option<HashSet<String>>,
    /// Colourscheme hashmap
    pub colourscheme: HashMap<String, String>,
    /// Enables enhanced plaintext detection using a BERT model.
    pub enhanced_detection: bool,
    /// Path to the enhanced detection model. If None, will use the default path.
    pub model_path: Option<String>,
    /// Depth penalty for A* search - adds cost per depth level to ensure
    /// shallow unexplored decoders eventually become competitive.
    /// Default: 0.15 (at depth ~13, exploring deeper costs more than trying Caesar at depth 0)
    pub depth_penalty: f32,
    /// Maximum number of decoders to try per node expansion in A* search.
    /// Lower values = faster but may miss correct path.
    /// Higher values = more thorough but slower.
    /// Default: 5 (covers Base64, Base32, Hex, Binary, URL - the most common encodings)
    pub decoder_batch_size: usize,
    /// List of decoders to run. Empty list means no decoders run.
    /// Use `get_all_decoder_names()` from `crate::decoders` to get the full list.
    /// Note: If not present in config file (None after deserialization), all decoders run.
    /// This allows backwards compatibility with existing configs.
    #[serde(default)]
    pub decoders_to_run: Vec<String>,
    /// List of checkers to run. Empty list means no checkers run.
    /// Use `get_all_checker_names()` from `crate::checkers` to get the full list.
    /// Note: If not present in config file (None after deserialization), all checkers run.
    /// This allows backwards compatibility with existing configs.
    #[serde(default)]
    pub checkers_to_run: Vec<String>,
    /// How long status messages display before auto-clearing (in seconds).
    /// Default: 10 seconds. Set to 0 to never auto-clear.
    #[serde(default = "default_status_message_timeout")]
    pub status_message_timeout: u64,
    /// Whether AI features are enabled.
    #[serde(default)]
    pub ai_enabled: bool,
    /// OpenAI-compatible API endpoint URL (e.g., "https://api.openai.com/v1").
    #[serde(default)]
    pub ai_api_url: Option<String>,
    /// API key for the AI endpoint.
    #[serde(default)]
    pub ai_api_key: Option<String>,
    /// Model name to use for AI features (e.g., "gpt-4o-mini").
    #[serde(default)]
    pub ai_model: Option<String>,
    /// Quick search URL templates for the "Open" shortcut in the TUI Results screen.
    /// Each entry is in the format "Name=https://example.com/search?q={}" where `{}` is
    /// replaced with URL-encoded output text, or `{base64}` is replaced with
    /// base64-encoded output text (useful for tools like CyberChef).
    #[serde(default = "default_quick_searches")]
    pub quick_searches: Vec<String>,
}

/// Default status message timeout in seconds.
fn default_status_message_timeout() -> u64 {
    10
}

/// Default quick search URL templates.
pub fn default_quick_searches() -> Vec<String> {
    vec![
        "Google=https://www.google.com/search?q={}".to_string(),
        "ChatGPT=https://chatgpt.com/?q={}".to_string(),
        "DuckDuckGo=https://duckduckgo.com/?q={}".to_string(),
        "CyberChef=https://gchq.github.io/CyberChef/#input={base64}".to_string(),
    ]
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

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            verbose: self.verbose,
            lemmeknow_config: make_identifier_from_config(self),
            lemmeknow_min_rarity: self.lemmeknow_min_rarity,
            lemmeknow_max_rarity: self.lemmeknow_max_rarity,
            lemmeknow_tags: self.lemmeknow_tags.clone(),
            lemmeknow_exclude_tags: self.lemmeknow_exclude_tags.clone(),
            lemmeknow_boundaryless: self.lemmeknow_boundaryless,
            human_checker_on: self.human_checker_on,
            timeout: self.timeout,
            top_results: self.top_results,
            api_mode: self.api_mode,
            regex: self.regex.clone(),
            wordlist_path: self.wordlist_path.clone(),
            wordlist: self.wordlist.clone(),
            colourscheme: self.colourscheme.clone(),
            enhanced_detection: self.enhanced_detection,
            model_path: self.model_path.clone(),
            depth_penalty: self.depth_penalty,
            decoder_batch_size: self.decoder_batch_size,
            decoders_to_run: self.decoders_to_run.clone(),
            checkers_to_run: self.checkers_to_run.clone(),
            status_message_timeout: self.status_message_timeout,
            ai_enabled: self.ai_enabled,
            ai_api_url: self.ai_api_url.clone(),
            ai_api_key: self.ai_api_key.clone(),
            ai_model: self.ai_model.clone(),
            quick_searches: self.quick_searches.clone(),
        }
    }
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
            top_results: false,
            api_mode: false,
            regex: None,
            wordlist_path: None,
            wordlist: None,
            enhanced_detection: false,
            model_path: None,
            colourscheme: HashMap::new(),
            depth_penalty: 0.5,
            decoder_batch_size: 5,
            // Default to all decoders and checkers enabled (empty means all)
            decoders_to_run: vec![],
            checkers_to_run: vec![],
            status_message_timeout: default_status_message_timeout(),
            ai_enabled: false,
            ai_api_url: None,
            ai_api_key: None,
            ai_model: None,
            quick_searches: default_quick_searches(),
        };

        // Set default colors
        config
            .colourscheme
            .insert(String::from("informational"), String::from("255,215,0")); // Gold yellow
        config
            .colourscheme
            .insert(String::from("warning"), String::from("255,0,0")); // Red
        config
            .colourscheme
            .insert(String::from("success"), String::from("0,255,0")); // Green
        config
            .colourscheme
            .insert(String::from("error"), String::from("255,0,0")); // Red

        config
            .colourscheme
            .insert(String::from("question"), String::from("255,215,0")); // Gold yellow (same as informational)
        config
    }
}

/// Get the path to the ciphey config directory (~/.ciphey/)
///
/// # Returns
///
/// The path to the ciphey config directory, or None if the home directory cannot be found.
pub fn get_config_dir() -> Option<std::path::PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(".ciphey");
    Some(path)
}

/// Get the path to the ciphey config file
///
/// # Panics
///
/// This function will panic if:
/// - The home directory cannot be found
/// - The ciphey directory cannot be created
pub fn get_config_file_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ciphey");
    fs::create_dir_all(&path).expect("Could not create ciphey directory");
    path.push("config.toml");
    path
}

/// Delete the entire ciphey configuration directory (~/.ciphey/).
///
/// This removes:
/// - config.toml (user configuration)
/// - database.sqlite (cache and analytics)
/// - wordlist_bloom.dat (bloom filter cache)
/// - Any other files in the directory
///
/// # Returns
///
/// - `Ok(true)` if the directory was deleted successfully
/// - `Ok(false)` if the directory did not exist
/// - `Err(String)` if deletion failed
pub fn delete_config_directory() -> Result<bool, String> {
    let config_dir = match get_config_dir() {
        Some(path) => path,
        None => return Err("Could not find home directory".to_string()),
    };

    if !config_dir.exists() {
        return Ok(false);
    }

    fs::remove_dir_all(&config_dir)
        .map_err(|e| format!("Failed to delete config directory: {}", e))?;
    Ok(true)
}

/// Checks if the config file exists (without loading it).
///
/// This is useful for determining if first-run setup should be shown.
pub fn config_exists() -> bool {
    let mut path = match dirs::home_dir() {
        Some(p) => p,
        None => return false,
    };
    path.push(".ciphey");
    path.push("config.toml");
    path.exists()
}

/// Create a default config file at the specified path
///
/// # Panics
///
/// This function will panic if:
/// - The config cannot be serialized to TOML
/// - The config file path cannot be determined (see `get_config_file_path`)
pub fn create_default_config_file() -> std::io::Result<()> {
    let config = Config::default();
    let toml_string = toml::to_string_pretty(&config).expect("Could not serialize config");
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
    let parsed_value: toml::Value = toml::from_str(contents).expect("Could not parse config file");

    // Check for unknown keys at the root level
    if let toml::Value::Table(table) = &parsed_value {
        let known_keys = [
            "verbose",
            "lemmeknow_min_rarity",
            "enhanced_detection",
            "model_path",
            "lemmeknow_max_rarity",
            "lemmeknow_tags",
            "lemmeknow_exclude_tags",
            "lemmeknow_boundaryless",
            "human_checker_on",
            "timeout",
            "top_results",
            "api_mode",
            "regex",
            "wordlist_path",
            "question",
            "colourscheme",
            "depth_penalty",
            "decoder_batch_size",
            "decoders_to_run",
            "checkers_to_run",
            "status_message_timeout",
            "ai_enabled",
            "ai_api_url",
            "ai_api_key",
            "ai_model",
            "quick_searches",
        ];
        for key in table.keys() {
            if !known_keys.contains(&key.as_str()) {
                crate::cli_pretty_printing::warning_unknown_config_key(key);
            }
        }
    }

    // Parse into Config struct
    let mut config: Config = toml::from_str(contents).expect("Could not parse config file");
    update_identifier_in_config(&mut config);
    config
}

/// Loads a wordlist from a file into a HashSet for efficient lookups
/// Uses memory mapping for large files to improve performance and memory usage
///
/// # Arguments
/// * `path` - Path to the wordlist file
///
/// # Returns
/// * `Ok(HashSet<String>)` - The loaded wordlist as a HashSet for O(1) lookups
/// * `Err(io::Error)` - If the file cannot be opened or read
///
/// # Errors
/// This function will return an error if:
/// * The file does not exist
/// * The file cannot be opened due to permissions
/// * The file cannot be memory-mapped
/// * The file contains invalid UTF-8 characters
///
/// # Safety
/// This implementation uses unsafe code in two places:
/// 1. Memory mapping (unsafe { Mmap::map(&file) }):
///    - This is unsafe because the memory map could become invalid if the underlying file is modified
///    - We accept this risk since the wordlist is only loaded once at startup and not expected to change
///
/// 2. UTF-8 conversion (unsafe { std::str::from_utf8_unchecked(&mmap) }):
///    - This is unsafe because it assumes the file contains valid UTF-8
///    - We attempt to convert to UTF-8 first and panic if invalid, making this assumption safe
///    - The unchecked version is used for performance since we verify UTF-8 validity first
pub fn load_wordlist<P: AsRef<Path>>(path: P) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let file_size = file.metadata()?.len();

    // For small files (under 10MB), use regular file reading
    // This threshold was chosen because:
    // 1. Most wordlists under 10MB can be loaded quickly with minimal memory overhead
    // 2. Memory mapping has overhead that may not be worth it for small files
    // 3. 10MB allows for roughly 1 million words (assuming average word length of 10 chars)
    if file_size < 10_000_000 {
        // 10MB threshold
        let reader = BufReader::new(file);
        let mut wordlist = HashSet::new();

        for line in reader.lines() {
            if let Ok(word) = line {
                let trimmed = word.trim().to_string();
                if !trimmed.is_empty() {
                    wordlist.insert(trimmed);
                }
            }
        }

        Ok(wordlist)
    } else {
        // For large files, use memory mapping
        // First create the memory map
        let mmap = unsafe { Mmap::map(&file)? };

        // Verify the file contains valid UTF-8 before proceeding
        if std::str::from_utf8(&mmap).is_err() {
            panic!("Wordlist file contains invalid UTF-8");
        }

        // Now we can safely use from_utf8_unchecked since we verified it's valid UTF-8
        let mut wordlist = HashSet::new();
        let content = unsafe { std::str::from_utf8_unchecked(&mmap) };
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                wordlist.insert(trimmed.to_string());
            }
        }

        Ok(wordlist)
    }
}

/// Get configuration from file or create default if it doesn't exist
///
/// Note: This function no longer runs the first-time setup wizard automatically.
/// The setup wizard is handled at a higher level in main.rs to support TUI mode.
/// If no config file exists, this will return the default configuration.
pub fn get_config_file_into_struct() -> Config {
    let path = get_config_file_path();

    if !path.exists() {
        // No config file - return default
        // First-run setup is handled separately in main.rs
        Config::default()
    } else {
        // Existing config - read and parse it
        match read_config_file() {
            Ok(contents) => {
                let mut config = parse_toml_with_unknown_keys(&contents);

                // If wordlist is specified in config file, set it in the config struct
                if let Some(wordlist_path) = &config.wordlist_path {
                    // Load the wordlist here in the config layer
                    match load_wordlist(wordlist_path) {
                        Ok(wordlist) => {
                            config.wordlist = Some(wordlist);
                        }
                        Err(_e) => {
                            // Critical error - exit if config specifies wordlist but can't load it
                            eprintln!("Can't load wordlist at '{}'. Either fix or remove wordlist from config file at '{}'", 
                                wordlist_path, path.display());
                            std::process::exit(1);
                        }
                    }
                }

                config
            }
            Err(e) => {
                eprintln!("Error reading config file: {}. Using defaults.", e);
                Config::default()
            }
        }
    }
}

/// Creates a Config from a first-run setup HashMap and saves it to disk.
///
/// This is called after the TUI or CLI first-run wizard completes.
///
/// # Arguments
///
/// * `setup_config` - HashMap of configuration values from the setup wizard
///
/// # Returns
///
/// The built Config struct
pub fn create_config_from_setup(setup_config: std::collections::HashMap<String, String>) -> Config {
    let path = get_config_file_path();
    let mut config = Config::default();

    // Extract color scheme values
    config.colourscheme = setup_config
        .iter()
        .filter(|(k, _)| {
            !k.starts_with("wordlist")
                && *k != "timeout"
                && *k != "top_results"
                && *k != "enhanced_detection"
                && *k != "model_path"
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // Set timeout if present
    if let Some(timeout) = setup_config.get("timeout") {
        config.timeout = timeout.parse().unwrap_or(5);
    }

    // Set top_results if present
    if let Some(top_results) = setup_config.get("top_results") {
        config.top_results = top_results.parse().unwrap_or(false);
    }

    // Set enhanced detection if present
    if let Some(enhanced) = setup_config.get("enhanced_detection") {
        config.enhanced_detection = enhanced.parse().unwrap_or(false);
    }

    // Set model path if present
    if let Some(model_path) = setup_config.get("model_path") {
        config.model_path = Some(model_path.clone());
    }

    // Extract wordlist path if present
    if let Some(wordlist_path) = setup_config.get("wordlist_path") {
        config.wordlist_path = Some(wordlist_path.clone());

        // Load the wordlist
        match load_wordlist(wordlist_path) {
            Ok(wordlist) => {
                config.wordlist = Some(wordlist);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Could not load wordlist at '{}': {}",
                    wordlist_path, e
                );
                // Don't exit - just continue without the wordlist
            }
        }
    }

    // Set AI config if present
    if let Some(ai_enabled) = setup_config.get("ai_enabled") {
        config.ai_enabled = ai_enabled.parse().unwrap_or(false);
    }
    if let Some(ai_api_url) = setup_config.get("ai_api_url") {
        if !ai_api_url.is_empty() {
            config.ai_api_url = Some(ai_api_url.clone());
        }
    }
    if let Some(ai_api_key) = setup_config.get("ai_api_key") {
        if !ai_api_key.is_empty() {
            config.ai_api_key = Some(ai_api_key.clone());
        }
    }
    if let Some(ai_model) = setup_config.get("ai_model") {
        if !ai_model.is_empty() {
            config.ai_model = Some(ai_model.clone());
        }
    }

    // Set quick searches if present (pipe-separated)
    if let Some(quick_searches_str) = setup_config.get("quick_searches") {
        if !quick_searches_str.is_empty() {
            config.quick_searches = quick_searches_str
                .split('|')
                .map(|s| s.to_string())
                .collect();
        }
    }

    // Save the config to file
    save_config_to_file(&config, &path);
    config
}

/// Save a Config struct to a file
pub fn save_config_to_file(config: &Config, path: &std::path::Path) {
    let toml_string = toml::to_string_pretty(config).expect("Could not serialize config");
    let mut file = File::create(path).expect("Could not create config file");
    file.write_all(toml_string.as_bytes())
        .expect("Could not write to config file");
}

/// Saves the given config to the standard config file location.
///
/// # Arguments
///
/// * `config` - The configuration to save
///
/// # Errors
///
/// Returns an error if the config file cannot be written.
pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let path = get_config_file_path();
    save_config_to_file(config, &path);
    Ok(())
}
