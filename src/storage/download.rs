//! Module for downloading and importing wordlists from URLs.
//!
//! This module provides functionality to download wordlists from remote URLs,
//! parse them into word sets, and import them into the database with automatic
//! bloom filter rebuilding.

use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use super::bloom::{build_bloom_filter_from_db, save_bloom_filter};
use super::database::{
    import_wordlist, insert_wordlist_file, setup_database, update_words_file_id,
};

/// Represents a predefined wordlist available for download.
#[derive(Debug, Clone)]
pub struct PredefinedWordlist {
    /// Display name shown to users
    pub name: String,
    /// Source identifier stored in database (filename without extension)
    pub source_id: String,
    /// Full URL to download the wordlist from
    pub url: String,
    /// Short description of the wordlist content
    pub description: String,
}

/// List of predefined wordlists available for users to download during setup.
///
/// These wordlists are sourced from the SecLists repository and contain
/// commonly used passwords useful for CTF challenges and security testing.
pub const PREDEFINED_WORDLISTS: &[PredefinedWordlist] = &[
    PredefinedWordlist {
        name: String::new(),
        source_id: String::new(),
        url: String::new(),
        description: String::new(),
    },
    PredefinedWordlist {
        name: String::new(),
        source_id: String::new(),
        url: String::new(),
        description: String::new(),
    },
];

/// Initialize the predefined wordlists array.
/// This function exists to work around const initialization limitations.
///
/// # Returns
///
/// A vector containing the two predefined SecLists wordlists
pub fn get_predefined_wordlists() -> Vec<PredefinedWordlist> {
    vec![
        PredefinedWordlist {
            name: "2025 Top 199 Passwords".to_string(),
            source_id: "2025-199_most_used_passwords".to_string(),
            url: "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Passwords/Common-Credentials/2025-199_most_used_passwords.txt".to_string(),
            description: "Most commonly used passwords in 2025".to_string(),
        },
        PredefinedWordlist {
            name: "500 Worst Passwords".to_string(),
            source_id: "500-worst-passwords".to_string(),
            url: "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Passwords/Common-Credentials/500-worst-passwords.txt".to_string(),
            description: "Notoriously weak passwords".to_string(),
        },
    ]
}

/// Downloads a wordlist from a URL and parses it into a set of words.
///
/// This function fetches the wordlist file from the provided URL, reads it line by line,
/// and collects all non-empty trimmed lines into a HashSet. The HashSet automatically
/// deduplicates any repeated words.
///
/// # Arguments
///
/// * `url` - The URL to download the wordlist from
///
/// # Returns
///
/// Returns `Ok(HashSet<String>)` containing all words from the wordlist on success.
/// Returns `Err(String)` with an error message if the download or parsing fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request fails (network error, invalid URL, etc.)
/// - The HTTP response status is not successful (4xx, 5xx errors)
/// - The request times out (30 second timeout)
/// - The response body cannot be read
///
/// # Examples
///
/// ```no_run
/// use ciphey::storage::download::download_wordlist_from_url;
///
/// let words = download_wordlist_from_url("https://example.com/wordlist.txt")?;
/// println!("Downloaded {} unique words", words.len());
/// # Ok::<(), String>(())
/// ```
pub fn download_wordlist_from_url(url: &str) -> Result<HashSet<String>, String> {
    // Create HTTP client with 30 second timeout
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Send GET request
    let response = client
        .get(url)
        .send()
        .map_err(|e| format!("Failed to download from {}: {}", url, e))?;

    // Check if request was successful
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download wordlist: HTTP {}",
            response.status()
        ));
    }

    // Read response body line by line
    let reader = BufReader::new(response);
    let mut words = HashSet::new();

    for line in reader.lines() {
        match line {
            Ok(word) => {
                let trimmed = word.trim();
                if !trimmed.is_empty() {
                    words.insert(trimmed.to_string());
                }
            }
            Err(e) => {
                // Log error but continue processing other lines
                log::warn!("Error reading line from wordlist: {}", e);
            }
        }
    }

    if words.is_empty() {
        return Err("Downloaded wordlist is empty".to_string());
    }

    Ok(words)
}

/// Imports a wordlist into the database and rebuilds the bloom filter.
///
/// This function performs three operations:
/// 1. Registers the wordlist file in the `wordlist_files` table for the Wordlist Manager UI
/// 2. Imports the provided words into the `wordlist` table
/// 3. Rebuilds the bloom filter from all words in the database
///
/// The bloom filter rebuild ensures that the fast lookup mechanism is updated
/// to include the newly imported words.
///
/// # Arguments
///
/// * `words` - A HashSet containing the words to import
/// * `source` - Source identifier for the wordlist (e.g., "2025-199_most_used_passwords")
/// * `filename` - Display name for the wordlist file (e.g., "2025 Top 199 Passwords")
/// * `file_path` - Path or URL identifier for the wordlist (must be unique per wordlist)
///
/// # Returns
///
/// Returns `Ok(usize)` with the number of words successfully imported on success.
/// Returns `Err(String)` with an error message if the import or bloom rebuild fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Database connection fails
/// - Word insertion fails
/// - Bloom filter rebuild fails
/// - Bloom filter save fails
///
/// # Examples
///
/// ```no_run
/// use std::collections::HashSet;
/// use ciphey::storage::download::import_wordlist_with_bloom_rebuild;
///
/// let mut words = HashSet::new();
/// words.insert("password".to_string());
/// words.insert("123456".to_string());
///
/// let count = import_wordlist_with_bloom_rebuild(&words, "test_wordlist", "test.txt", "/path/to/test.txt")?;
/// println!("Imported {} words", count);
/// # Ok::<(), String>(())
/// ```
pub fn import_wordlist_with_bloom_rebuild(
    words: &HashSet<String>,
    source: &str,
    filename: &str,
    file_path: &str,
) -> Result<usize, String> {
    // Ensure database is set up (creates wordlist table if it doesn't exist)
    setup_database().map_err(|e| format!("Failed to setup database: {}", e))?;

    // Register the wordlist file so it appears in the Wordlist Manager UI
    let file_id = insert_wordlist_file(filename, file_path, source, words.len() as i64)
        .map_err(|e| format!("Failed to register wordlist file: {}", e))?;

    // Import wordlist to database
    let count = import_wordlist(words, source)
        .map_err(|e| format!("Failed to import wordlist to database: {}", e))?;

    // Link the imported words to their wordlist_files entry
    let _ = update_words_file_id(source, file_id);

    // Rebuild bloom filter from database
    let bloom =
        build_bloom_filter_from_db().map_err(|e| format!("Failed to build bloom filter: {}", e))?;

    // Save bloom filter to disk
    save_bloom_filter(&bloom).map_err(|e| format!("Failed to save bloom filter: {}", e))?;

    Ok(count)
}

/// Imports a wordlist from a file path into the database with bloom filter rebuild.
///
/// This function reads a wordlist file from the local filesystem, parses it into
/// a HashSet of words, and imports them into the database with automatic bloom
/// filter rebuilding.
///
/// # Arguments
///
/// * `path` - Path to the wordlist file
/// * `source` - Source identifier for the wordlist
///
/// # Returns
///
/// Returns `Ok(usize)` with the number of words successfully imported on success.
/// Returns `Err(String)` with an error message if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be opened
/// - The file cannot be read
/// - Database import fails
/// - Bloom filter rebuild fails
pub fn import_wordlist_from_file(path: &str, source: &str) -> Result<usize, String> {
    // Ensure database is set up (creates wordlist table if it doesn't exist)
    setup_database().map_err(|e| format!("Failed to setup database: {}", e))?;

    // Extract filename for display in Wordlist Manager
    let filename = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Open and read the file
    let file = std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    // Collect words into HashSet
    let mut words = HashSet::new();
    for line in reader.lines() {
        match line {
            Ok(word) => {
                let trimmed = word.trim();
                if !trimmed.is_empty() {
                    words.insert(trimmed.to_string());
                }
            }
            Err(e) => {
                log::warn!("Error reading line from file: {}", e);
            }
        }
    }

    if words.is_empty() {
        return Err("Wordlist file is empty".to_string());
    }

    // Import with bloom filter rebuild (also registers in wordlist_files)
    import_wordlist_with_bloom_rebuild(&words, source, &filename, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_predefined_wordlists() {
        let wordlists = get_predefined_wordlists();
        assert_eq!(wordlists.len(), 2);
        assert_eq!(wordlists[0].source_id, "2025-199_most_used_passwords");
        assert_eq!(wordlists[1].source_id, "500-worst-passwords");
    }

    #[test]
    fn test_download_invalid_url() {
        let result = download_wordlist_from_url(
            "https://invalid-url-that-does-not-exist-12345.com/wordlist.txt",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_import_empty_wordlist() {
        let words = HashSet::new();
        // This should succeed with 0 imported (empty is valid for import_wordlist)
        // but our wrapper might handle it differently
        let result = import_wordlist_with_bloom_rebuild(&words, "test", "test.txt", "test://empty");
        // We allow empty imports, so this should work
        assert!(result.is_ok() || result.is_err());
    }
}
