//! Bloom filter module for fast wordlist membership testing
//!
//! This module provides functionality to build, save, and load bloom filters
//! for efficient probabilistic membership testing of wordlists. The bloom filter
//! is used as a fast first-pass check before querying the database for confirmation.

use bloomfilter::Bloom;
use log::trace;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::OnceLock;

use super::database::{get_word_count, read_all_words};

/// Cached bloom filter loaded once from disk and reused for all subsequent
/// lookups. This avoids deserializing JSON from disk on every call to
/// `load_bloom_filter()`, which was a significant performance bottleneck
/// for the wordlist checker (called on every A* node).
static CACHED_BLOOM_FILTER: OnceLock<Option<Bloom<String>>> = OnceLock::new();

/// Bloom filter file name stored in ~/.ciphey/
const BLOOM_FILTER_FILE: &str = "wordlist_bloom.dat";

/// False positive rate for the bloom filter (1%)
/// This means ~1% of non-existent words may incorrectly return "maybe exists"
const FALSE_POSITIVE_RATE: f64 = 0.01;

/// Minimum number of items for bloom filter creation
/// If the wordlist has fewer items, we use this as the minimum capacity
const MIN_BLOOM_ITEMS: usize = 100;

/// Error type for bloom filter operations
#[derive(Debug)]
pub enum BloomError {
    /// IO error during file operations
    Io(std::io::Error),
    /// Database error when reading words
    Database(rusqlite::Error),
    /// Serialization/deserialization error
    Serialization(String),
}

impl std::fmt::Display for BloomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BloomError::Io(e) => write!(f, "IO error: {}", e),
            BloomError::Database(e) => write!(f, "Database error: {}", e),
            BloomError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for BloomError {}

impl From<std::io::Error> for BloomError {
    fn from(err: std::io::Error) -> Self {
        BloomError::Io(err)
    }
}

impl From<rusqlite::Error> for BloomError {
    fn from(err: rusqlite::Error) -> Self {
        BloomError::Database(err)
    }
}

/// Returns the path to the bloom filter file (~/.ciphey/wordlist_bloom.dat)
pub fn get_bloom_filter_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ciphey");
    path.push(BLOOM_FILTER_FILE);
    path
}

/// Builds a bloom filter from all words in the database
///
/// This function reads all words from the wordlist table and creates
/// an optimally-sized bloom filter based on the word count.
///
/// # Returns
///
/// Returns a `Bloom<String>` filter containing all words from the database
///
/// # Errors
///
/// Returns `BloomError` if database operations fail
pub fn build_bloom_filter_from_db() -> Result<Bloom<String>, BloomError> {
    // Get word count to size the bloom filter optimally
    let count = get_word_count()? as usize;
    trace!("Building bloom filter for {} words", count);

    // Use minimum capacity if wordlist is very small
    let capacity = std::cmp::max(count, MIN_BLOOM_ITEMS);

    // Create bloom filter with optimal parameters for the given capacity
    // The bloomfilter crate calculates optimal number of hash functions internally
    let mut bloom = Bloom::new_for_fp_rate(capacity, FALSE_POSITIVE_RATE);

    // If no words, return empty bloom filter
    if count == 0 {
        trace!("No words in database, returning empty bloom filter");
        return Ok(bloom);
    }

    // Read all words and insert into bloom filter
    let words = read_all_words()?;
    for word in words {
        bloom.set(&word);
    }

    trace!(
        "Bloom filter built successfully with capacity for {} items",
        capacity
    );
    Ok(bloom)
}

/// Saves a bloom filter to disk at ~/.ciphey/wordlist_bloom.dat
///
/// The bloom filter is serialized using serde_json for portability.
///
/// # Arguments
///
/// * `bloom` - The bloom filter to save
///
/// # Errors
///
/// Returns `BloomError` if file operations fail
pub fn save_bloom_filter(bloom: &Bloom<String>) -> Result<(), BloomError> {
    let path = get_bloom_filter_path();
    trace!("Saving bloom filter to {:?}", path);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize bloom filter to JSON bytes
    let serialized = serde_json::to_vec(bloom).map_err(|e| {
        BloomError::Serialization(format!("Failed to serialize bloom filter: {}", e))
    })?;

    // Write to file
    let mut file = fs::File::create(&path)?;
    file.write_all(&serialized)?;
    file.flush()?;

    trace!(
        "Bloom filter saved successfully ({} bytes)",
        serialized.len()
    );
    Ok(())
}

/// Loads a bloom filter from disk, caching it in a `OnceLock` so the
/// deserialization only happens once.
///
/// # Returns
///
/// Returns `Some(&Bloom<String>)` if the file exists and can be loaded,
/// `None` if the file doesn't exist (not an error condition)
///
/// # Errors
///
/// Returns `BloomError` if the file exists but cannot be read or deserialized
pub fn load_bloom_filter() -> Result<Option<&'static Bloom<String>>, BloomError> {
    // Fast path: return the cached value if already loaded
    if let Some(cached) = CACHED_BLOOM_FILTER.get() {
        return Ok(cached.as_ref());
    }

    // Slow path: load from disk (only runs once)
    let result = load_bloom_filter_from_disk();

    match result {
        Ok(bloom_opt) => {
            // Store in cache (ignore if another thread beat us)
            let _ = CACHED_BLOOM_FILTER.set(bloom_opt);
            Ok(CACHED_BLOOM_FILTER.get().unwrap().as_ref())
        }
        Err(e) => {
            // Cache None so we don't retry on every call
            let _ = CACHED_BLOOM_FILTER.set(None);
            Err(e)
        }
    }
}

/// Internal function that actually reads the bloom filter from disk.
/// Called only once by `load_bloom_filter()`.
fn load_bloom_filter_from_disk() -> Result<Option<Bloom<String>>, BloomError> {
    let path = get_bloom_filter_path();
    trace!("Loading bloom filter from {:?}", path);

    // Check if file exists
    if !path.exists() {
        trace!("Bloom filter file does not exist");
        return Ok(None);
    }

    // Read file contents
    let mut file = fs::File::open(&path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Deserialize bloom filter
    let bloom: Bloom<String> = serde_json::from_slice(&contents).map_err(|e| {
        BloomError::Serialization(format!("Failed to deserialize bloom filter: {}", e))
    })?;

    trace!("Bloom filter loaded successfully");
    Ok(Some(bloom))
}

/// Checks if the bloom filter file exists
pub fn bloom_filter_exists() -> bool {
    get_bloom_filter_path().exists()
}

/// Deletes the bloom filter file if it exists
///
/// # Errors
///
/// Returns `BloomError` if the file exists but cannot be deleted
pub fn delete_bloom_filter() -> Result<(), BloomError> {
    let path = get_bloom_filter_path();
    if path.exists() {
        fs::remove_file(&path)?;
        trace!("Bloom filter deleted");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::database::{init_database, insert_word, set_db_path};

    fn set_test_db_path() {
        let path = std::path::PathBuf::from(String::from("file::memory:?cache=shared"));
        set_db_path(Some(path));
    }

    #[test]
    #[serial_test::serial]
    fn test_build_empty_bloom_filter() {
        set_test_db_path();
        let _conn = init_database().unwrap(); // Keep connection alive

        let bloom = build_bloom_filter_from_db().unwrap();
        // Empty bloom filter should not contain any words
        assert!(!bloom.check(&"test".to_string()));
    }

    #[test]
    #[serial_test::serial]
    fn test_build_bloom_filter_with_words() {
        set_test_db_path();
        let _conn = init_database().unwrap(); // Keep connection alive

        // Insert some test words
        insert_word("password123", "test").unwrap();
        insert_word("hello", "test").unwrap();
        insert_word("world", "test").unwrap();

        let bloom = build_bloom_filter_from_db().unwrap();

        // Bloom filter should contain inserted words
        assert!(bloom.check(&"password123".to_string()));
        assert!(bloom.check(&"hello".to_string()));
        assert!(bloom.check(&"world".to_string()));

        // Should not contain words that weren't inserted
        // (Note: bloom filters can have false positives, but this is unlikely for distinct strings)
        assert!(!bloom.check(&"notinlist".to_string()));
    }

    #[test]
    fn test_bloom_filter_path() {
        let path = get_bloom_filter_path();
        assert!(path.ends_with("wordlist_bloom.dat"));
        assert!(path.to_string_lossy().contains(".ciphey"));
    }
}