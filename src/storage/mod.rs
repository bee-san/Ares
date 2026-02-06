use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Module for bloom filter management (fast wordlist membership testing)
pub mod bloom;
/// Module housing functions for managing SQLite database
pub mod database;
/// Module for downloading and importing wordlists from URLs
pub mod download;
/// Module for storing WaitAthena results
pub mod wait_athena_storage;

/// Returns the path to the Ciphey data directory (~/.ciphey/).
///
/// This is the central location for all Ciphey data files including:
/// - config.toml
/// - database.sqlite
/// - wordlist_bloom.dat
pub fn get_ciphey_dir() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|p| p.join(".ciphey"))
}

/// Initializes the Ciphey storage directory and database.
///
/// This function should be called early in the application startup.
/// It creates the `~/.ciphey/` directory if it doesn't exist and
/// initializes the SQLite database with the required schema.
///
/// Both TUI and CLI modes should call this before performing any
/// operations that require database access.
///
/// # Errors
///
/// Returns an error string if directory creation or database initialization fails.
pub fn initialize_storage() -> Result<(), String> {
    // Get the ciphey directory path
    let ciphey_dir = get_ciphey_dir().ok_or("Could not find home directory")?;

    // Create directory if it doesn't exist
    if !ciphey_dir.exists() {
        std::fs::create_dir_all(&ciphey_dir)
            .map_err(|e| format!("Failed to create ~/.ciphey directory: {}", e))?;
    }

    // Initialize the database
    database::setup_database().map_err(|e| format!("Failed to setup database: {}", e))?;

    Ok(())
}

/// English letter frequency distribution (A-Z)
/// Used for frequency analysis in various decoders
pub const ENGLISH_FREQS: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, // V-Z
];

/// Loads invisible character list into a HashSet
pub static INVISIBLE_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    let mut entries: HashSet<char> = HashSet::new();

    // Path to the invisible characters file
    let chars_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("storage")
        .join("invisible_chars")
        .join("chars.txt");

    // Read the file content
    if let Ok(content) = fs::read_to_string(&chars_file_path) {
        let content_lines = content.split('\n');
        for line in content_lines {
            if line.is_empty() {
                continue;
            }
            let unicode_line_split: Vec<&str> = line.split_ascii_whitespace().collect();
            if unicode_line_split.is_empty() {
                continue;
            }
            let unicode_literal = unicode_line_split[0].trim_start_matches("U+");
            if let Ok(unicode_value) = u32::from_str_radix(unicode_literal, 16) {
                if let Some(unicode_char) = char::from_u32(unicode_value) {
                    entries.insert(unicode_char);
                }
            }
        }
    }

    entries
});

// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invisible_chars_loaded() {
        // Verify that the INVISIBLE_CHARS HashSet is not empty
        assert!(!INVISIBLE_CHARS.is_empty());
    }

    #[test]
    fn test_invisible_chars_contains_space() {
        // Verify that the space character (U+0020) is in the HashSet
        assert!(INVISIBLE_CHARS.contains(&' '));
    }

    #[test]
    fn test_invisible_chars_contains_zero_width_space() {
        // Verify that the zero width space (U+200B) is in the HashSet
        // This is a common invisible character
        let zero_width_space = char::from_u32(0x200B).unwrap();
        assert!(INVISIBLE_CHARS.contains(&zero_width_space));
    }
}
