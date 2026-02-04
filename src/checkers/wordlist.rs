use crate::checkers::checker_result::CheckResult;
use crate::checkers::checker_type::{Check, Checker};
use crate::config::get_config;
use crate::storage::bloom::load_bloom_filter;
use crate::storage::database::word_exists;
use gibberish_or_not::Sensitivity;
use lemmeknow::Identifier;
use log::trace;
#[cfg(test)]
use std::collections::HashSet;

/// WordlistChecker checks if the input text exactly matches any word in a user-provided wordlist
///
/// This checker uses a two-tier lookup system for optimal performance:
/// 1. **Bloom filter** (fast): First checks a bloom filter for quick rejection of non-matches
/// 2. **Database lookup** (accurate): If bloom filter says "maybe", confirms with SQLite query
///
/// Falls back to the config-based wordlist if bloom filter/database is not available.
pub struct WordlistChecker;

impl Check for Checker<WordlistChecker> {
    fn new() -> Self {
        Checker {
            name: "Wordlist Checker",
            description:
                "Checks if the input text exactly matches any word in a user-provided wordlist",
            link: "",
            tags: vec!["wordlist", "exact-match"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            sensitivity: Sensitivity::Medium, // Dummy value - not used by this checker
            enhanced_detector: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        // Try bloom filter + DB first (new approach)
        match load_bloom_filter() {
            Ok(Some(bloom)) => {
                trace!("Using bloom filter for wordlist check");

                // Fast path: bloom filter says definitely not present
                if !bloom.check(&text.to_string()) {
                    trace!("Bloom filter: word '{}' definitely not in wordlist", text);
                    return CheckResult::new(self);
                }

                // Bloom filter says "maybe" - verify with database
                trace!("Bloom filter: word '{}' may exist, checking database", text);
                match word_exists(text) {
                    Ok(true) => {
                        trace!("Database confirmed word '{}' exists in wordlist", text);
                        let mut result = CheckResult::new(self);
                        result.is_identified = true;
                        result.text = text.to_string();
                        result.description =
                            "text which matches an entry in the wordlist database".to_string();
                        return result;
                    }
                    Ok(false) => {
                        trace!(
                            "Database: word '{}' not found (bloom filter false positive)",
                            text
                        );
                        return CheckResult::new(self);
                    }
                    Err(e) => {
                        trace!(
                            "Database error while checking word '{}': {}, falling back to config wordlist",
                            text,
                            e
                        );
                        // Fall through to config wordlist
                    }
                }
            }
            Ok(None) => {
                trace!("No bloom filter found, using config wordlist");
            }
            Err(e) => {
                trace!(
                    "Error loading bloom filter: {}, falling back to config wordlist",
                    e
                );
            }
        }

        // Fallback: use config wordlist (backward compatibility)
        let config = get_config();

        if let Some(wordlist) = &config.wordlist {
            trace!(
                "Running wordlist checker with {} config entries",
                wordlist.len()
            );

            // Perform exact matching against the wordlist
            let is_match = wordlist.contains(text);

            if is_match {
                trace!("Found exact match in config wordlist for: {}", text);
                let mut result = CheckResult::new(self);
                result.is_identified = true;
                result.text = text.to_string();
                result.description =
                    "text which matches an entry in the provided wordlist".to_string();
                return result;
            }

            trace!("No match found in config wordlist for: {}", text);
        } else {
            trace!("Wordlist checker skipped - no wordlist provided and no bloom filter");
        }

        // No match found or no wordlist provided
        CheckResult::new(self)
    }

    fn with_sensitivity(mut self, sensitivity: Sensitivity) -> Self {
        // Wordlist checker doesn't use sensitivity, but we need to implement this method
        // to satisfy the Check trait. The sensitivity value is stored but not used.
        self.sensitivity = sensitivity;
        self
    }

    fn get_sensitivity(&self) -> Sensitivity {
        // Return the stored sensitivity value, though it's not used for checking
        self.sensitivity
    }
}

// Extension methods for testing
#[cfg(test)]
impl Checker<WordlistChecker> {
    /// Check with a directly provided wordlist (for testing)
    fn check_with_wordlist(&self, text: &str, wordlist: &HashSet<String>) -> CheckResult {
        trace!("Running wordlist checker with {} entries", wordlist.len());

        // Perform exact matching against the wordlist
        let is_match = wordlist.contains(text);

        if is_match {
            trace!("Found exact match in wordlist for: {}", text);
            let mut result = CheckResult::new(self);
            result.is_identified = true;
            result.text = text.to_string();
            result.description = "Text matches an entry in the provided wordlist".to_string();
            return result;
        }

        trace!("No match found in wordlist for: {}", text);

        // No match found
        CheckResult::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_wordlist_match() {
        // Create a test wordlist
        let mut wordlist = HashSet::new();
        wordlist.insert("password123".to_string());
        wordlist.insert("hello".to_string());
        wordlist.insert("test".to_string());

        // Create checker and test
        let checker = Checker::<WordlistChecker>::new();

        // Print debug info to help diagnose the issue
        println!("Testing with wordlist containing: password123, hello, test");

        // Should match
        let result = checker.check_with_wordlist("hello", &wordlist);
        println!("Result for 'hello': is_identified={}", result.is_identified);
        assert!(result.is_identified);

        // Should not match
        let result = checker.check_with_wordlist("goodbye", &wordlist);
        println!(
            "Result for 'goodbye': is_identified={}",
            result.is_identified
        );
        assert!(!result.is_identified);
    }

    #[test]
    fn test_no_wordlist() {
        // Create an empty wordlist
        let wordlist = HashSet::new();

        // Create checker and test
        let checker = Checker::<WordlistChecker>::new();

        // Print debug info
        println!("Testing with empty wordlist");

        // Should not match anything when no wordlist is provided
        let result = checker.check_with_wordlist("hello", &wordlist);
        println!(
            "Result for 'hello' with empty wordlist: is_identified={}",
            result.is_identified
        );
        assert!(!result.is_identified);
    }
}
