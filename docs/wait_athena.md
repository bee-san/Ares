# Implementation Plan for `wait_athena.rs` Checker

## Overview

The `wait_athena.rs` checker will be an exact clone of `athena.rs` with the following key differences:
1. It will store every plaintext it finds in a list instead of returning immediately
2. When the program timer expires, it will present all the plaintext it has found
3. We will need to modify the timer module to call a function that prints the list of plaintext when the countdown ends

## Important Clarifications

Based on discussions with the project owner:

1. **Searcher Integration**: We do NOT need to modify the searcher logic in `src/searchers/mod.rs`. The searcher will continue to work as before, but we'll add a CLI argument (`--top_results`) that will determine whether to use the standard Athena checker or the WaitAthena checker.

2. **Human Checker Interaction**: The human checker should NOT be involved in the WaitAthena process. WaitAthena should collect results automatically without prompting the user for each potential plaintext.

3. **Duplicate Handling**: We don't need to implement deduplication logic as duplicate plaintexts are very unlikely in practice.

4. **Dependency Requirements**: The `lazy_static` crate is already a dependency in the project, so no additional dependencies need to be added.

5. **Performance Considerations**: No special throttling or prioritization is needed for WaitAthena.

6. **Integration with Existing Decoders**: All existing decoders will work correctly with WaitAthena as long as they implement the `checker_type` trait and follow the standard checker pattern.

7. **Error Handling**: For mutex poisoning or other storage errors, we should simply panic as these are unexpected conditions.

8. **Sorting/Ranking Results**: For the initial implementation, we'll display results in the order they were found without any sorting. Future improvements may include sorting capabilities.

## Implementation Steps

### 1. Create a Global Storage for Plaintext Results

We need a thread-safe way to store plaintext results that can be accessed from both the checker and the timer. We'll use a lazy static approach with a mutex to ensure thread safety.

```rust
// In src/storage/wait_athena_storage.rs
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PlaintextResult {
    pub text: String,
    pub description: String,
    pub checker_name: String,
}

lazy_static! {
    static ref PLAINTEXT_RESULTS: Mutex<Vec<PlaintextResult>> = Mutex::new(Vec::new());
}

pub fn add_plaintext_result(text: String, description: String, checker_name: String) {
    let result = PlaintextResult {
        text,
        description,
        checker_name,
    };
    
    let mut results = PLAINTEXT_RESULTS.lock().unwrap();
    results.push(result);
}

pub fn get_plaintext_results() -> Vec<PlaintextResult> {
    let results = PLAINTEXT_RESULTS.lock().unwrap();
    results.clone()
}

pub fn clear_plaintext_results() {
    let mut results = PLAINTEXT_RESULTS.lock().unwrap();
    results.clear();
}
```

### 2. Update the `mod.rs` in the storage directory

```rust
// In src/storage/mod.rs
pub mod wait_athena_storage;
```

### 3. Create the `wait_athena.rs` Checker

Create a new file `src/checkers/wait_athena.rs` that is a clone of `athena.rs` but modified to store results instead of returning immediately:

```rust
// In src/checkers/wait_athena.rs
use crate::{checkers::checker_result::CheckResult, cli_pretty_printing, config::get_config};
use gibberish_or_not::Sensitivity;
use lemmeknow::Identifier;
use log::trace;

use crate::storage::wait_athena_storage;

use super::{
    checker_type::{Check, Checker},
    english::EnglishChecker,
    human_checker,
    lemmeknow_checker::LemmeKnow,
    password::PasswordChecker,
    regex_checker::RegexChecker,
};

/// WaitAthena checker runs all other checkers and stores results for later display
/// This is identical to Athena but instead of returning immediately, it stores results
/// and continues checking until the timer expires
pub struct WaitAthena;

impl Check for Checker<WaitAthena> {
    fn new() -> Self {
        Checker {
            name: "WaitAthena Checker",
            description: "Runs all available checkers and stores results until timer expires",
            link: "",
            tags: vec!["wait_athena", "all"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            sensitivity: Sensitivity::Medium, // Default to Medium sensitivity
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        let config = get_config();
        if config.regex.is_some() {
            trace!("running regex");
            let regex_checker = Checker::<RegexChecker>::new().with_sensitivity(self.sensitivity);
            let regex_result = regex_checker.check(text);
            if regex_result.is_identified {
                let mut check_res = CheckResult::new(&regex_checker);
                check_res.is_identified = true; // No human checker involvement
                check_res.text = regex_result.text;
                check_res.description = regex_result.description;
                
                // Store the result instead of returning immediately
                wait_athena_storage::add_plaintext_result(
                    check_res.text.clone(),
                    check_res.description.clone(),
                    regex_checker.name.to_string(),
                );
                
                // Return the result but continue checking
                return check_res;
            }
        } else {
            // In Ciphey if the user uses the regex checker all the other checkers turn off
            // This is because they are looking for one specific bit of information so will not want the other checkers
            let lemmeknow = Checker::<LemmeKnow>::new().with_sensitivity(self.sensitivity);
            let lemmeknow_result = lemmeknow.check(text);
            if lemmeknow_result.is_identified {
                let mut check_res = CheckResult::new(&lemmeknow);
                check_res.is_identified = true; // No human checker involvement
                check_res.text = lemmeknow_result.text;
                check_res.description = lemmeknow_result.description;
                
                // Store the result instead of returning immediately
                wait_athena_storage::add_plaintext_result(
                    check_res.text.clone(),
                    check_res.description.clone(),
                    lemmeknow.name.to_string(),
                );
                
                // Return the result but continue checking
                return check_res;
            }

            let password = Checker::<PasswordChecker>::new().with_sensitivity(self.sensitivity);
            let password_result = password.check(text);
            if password_result.is_identified {
                let mut check_res = CheckResult::new(&password);
                check_res.is_identified = true; // No human checker involvement
                check_res.text = password_result.text;
                check_res.description = password_result.description;
                
                // Store the result instead of returning immediately
                wait_athena_storage::add_plaintext_result(
                    check_res.text.clone(),
                    check_res.description.clone(),
                    password.name.to_string(),
                );
                
                // Return the result but continue checking
                return check_res;
            }

            let english = Checker::<EnglishChecker>::new().with_sensitivity(self.sensitivity);
            let english_result = english.check(text);
            if english_result.is_identified {
                let mut check_res = CheckResult::new(&english);
                check_res.is_identified = true; // No human checker involvement
                check_res.text = english_result.text;
                check_res.description = english_result.description;
                
                // Store the result instead of returning immediately
                wait_athena_storage::add_plaintext_result(
                    check_res.text.clone(),
                    check_res.description.clone(),
                    english.name.to_string(),
                );
                
                // Return the result but continue checking
                return check_res;
            }
        }

        CheckResult::new(self)
    }

    fn with_sensitivity(mut self, sensitivity: Sensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    fn get_sensitivity(&self) -> Sensitivity {
        self.sensitivity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gibberish_or_not::Sensitivity;

    #[test]
    fn test_check_english_sentence() {
        let checker = Checker::<WaitAthena>::new();
        assert!(checker.check("test valid english sentence").is_identified);
    }

    #[test]
    fn test_check_dictionary_word() {
        let checker = Checker::<WaitAthena>::new();
        assert!(checker.check("and").is_identified);
    }

    #[test]
    fn test_default_sensitivity_is_medium() {
        let checker = Checker::<WaitAthena>::new();
        assert!(matches!(checker.get_sensitivity(), Sensitivity::Medium));
    }

    #[test]
    fn test_with_sensitivity_changes_sensitivity() {
        let checker = Checker::<WaitAthena>::new().with_sensitivity(Sensitivity::Low);
        assert!(matches!(checker.get_sensitivity(), Sensitivity::Low));

        let checker = Checker::<WaitAthena>::new().with_sensitivity(Sensitivity::High);
        assert!(matches!(checker.get_sensitivity(), Sensitivity::High));
    }
}
```

### 4. Add Comments to Both Athena.rs and WaitAthena.rs

Add a comment at the top of both files to indicate they are similar but with different behaviors:

```rust
// In src/checkers/athena.rs (add at the top)
/// Athena checker runs all other checkers and returns immediately when a plaintext is found.
/// This is the standard checker that exits early when a plaintext is found.
/// For a version that continues checking and collects all plaintexts, see WaitAthena.

// In src/checkers/wait_athena.rs (add at the top)
/// WaitAthena checker is a variant of Athena that collects all plaintexts found during the search.
/// While Athena exits immediately when a plaintext is found, WaitAthena continues checking and
/// stores all plaintexts it finds until the timer expires.
```

### 5. Update the Checkers Module

Update the `src/checkers/mod.rs` file to include the new WaitAthena checker:

```rust
// In src/checkers/mod.rs
use self::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
    password::PasswordChecker,
    regex_checker::RegexChecker,
    wait_athena::WaitAthena,  // Add this line
    wordlist::WordlistChecker,
};

// Add this line
/// The WaitAthena Checker is a variant of Athena that collects all plaintexts found during the search
pub mod wait_athena;

// Update the CheckerTypes enum
pub enum CheckerTypes {
    /// Wrapper for LemmeKnow Checker
    CheckLemmeKnow(Checker<LemmeKnow>),
    /// Wrapper for English Checker
    CheckEnglish(Checker<EnglishChecker>),
    /// Wrapper for Athena Checker
    CheckAthena(Checker<Athena>),
    /// Wrapper for WaitAthena Checker
    CheckWaitAthena(Checker<WaitAthena>),  // Add this line
    /// Wrapper for Regex
    CheckRegex(Checker<RegexChecker>),
    /// Wrapper for Password Checker
    CheckPassword(Checker<PasswordChecker>),
    /// Wrapper for Wordlist Checker
    CheckWordlist(Checker<WordlistChecker>),
}

// Update the check method in the CheckerTypes impl
impl CheckerTypes {
    /// This functions calls appropriate check function of Checker
    pub fn check(&self, text: &str) -> CheckResult {
        match self {
            CheckerTypes::CheckLemmeKnow(lemmeknow_checker) => lemmeknow_checker.check(text),
            CheckerTypes::CheckEnglish(english_checker) => english_checker.check(text),
            CheckerTypes::CheckAthena(athena_checker) => athena_checker.check(text),
            CheckerTypes::CheckWaitAthena(wait_athena_checker) => wait_athena_checker.check(text),  // Add this line
            CheckerTypes::CheckRegex(regex_checker) => regex_checker.check(text),
            CheckerTypes::CheckPassword(password_checker) => password_checker.check(text),
            CheckerTypes::CheckWordlist(wordlist_checker) => wordlist_checker.check(text),
        }
    }

    // Update the with_sensitivity method
    pub fn with_sensitivity(&self, sensitivity: Sensitivity) -> Self {
        match self {
            // ... existing cases ...
            CheckerTypes::CheckWaitAthena(_checker) => {  // Add this block
                let mut new_checker = Checker::<WaitAthena>::new();
                new_checker.sensitivity = sensitivity;
                CheckerTypes::CheckWaitAthena(new_checker)
            },
            // ... rest of the cases ...
        }
    }

    // Update the get_sensitivity method
    pub fn get_sensitivity(&self) -> Sensitivity {
        match self {
            // ... existing cases ...
            CheckerTypes::CheckWaitAthena(checker) => checker.get_sensitivity(),  // Add this line
            // ... rest of the cases ...
        }
    }
}
```

### 6. Modify the Timer Module

Update the timer module to display the collected plaintext results when the timer expires:

```rust
// In src/timer/mod.rs
use crossbeam::channel::{bounded, Receiver};
use std::sync::atomic::Ordering::Relaxed;
use std::{
    sync::atomic::AtomicBool,
    thread::{self, sleep},
    time::Duration,
};

use crate::cli_pretty_printing::{countdown_until_program_ends, success};
use crate::storage::wait_athena_storage;
use crate::config::get_config;

/// Indicate whether timer is paused
static PAUSED: AtomicBool = AtomicBool::new(false);

/// Start the timer with duration in seconds
pub fn start(duration: u32) -> Receiver<()> {
    let (sender, recv) = bounded(1);
    thread::spawn(move || {
        let mut time_spent = 0;

        while time_spent < duration {
            if !PAUSED.load(Relaxed) {
                sleep(Duration::from_secs(1));
                time_spent += 1;
                // Some pretty printing support
                countdown_until_program_ends(time_spent, duration);
            }
        }
        
        // When the timer expires, display all collected plaintext results
        // Only if we're in wait_athena mode
        if get_config().top_results {
            display_wait_athena_results();
        }
        
        sender.send(()).expect("Timer should send succesfully");
    });

    recv
}

/// Display all plaintext results collected by WaitAthena
fn display_wait_athena_results() {
    let results = wait_athena_storage::get_plaintext_results();
    
    if results.is_empty() {
        return;
    }
    
    success("\n=== Top Results ===");
    success(&format!("Found {} potential plaintext results:", results.len()));
    
    for (i, result) in results.iter().enumerate() {
        success(&format!(
            "Result #{}: [{}] {}",
            i + 1,
            result.checker_name,
            result.text
        ));
        success(&format!("Description: {}", result.description));
        success("---");
    }
    
    success("=== End of Top Results ===\n");
}

/// Pause timer
pub fn pause() {
    PAUSED.store(true, Relaxed);
}

/// Resume timer
pub fn resume() {
    PAUSED.store(false, Relaxed);
}
```

### 7. Update the Config to Support WaitAthena Mode

Add a configuration option to enable WaitAthena mode:

```rust
// In src/config/mod.rs
#[derive(Debug, Clone)]
pub struct Config {
    // ... existing fields ...
    
    /// Whether to use top results mode (collect all plaintexts instead of exiting early)
    pub top_results: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // ... existing defaults ...
            
            top_results: false,
        }
    }
}
```

### 8. Update the Library Interface

Modify the `perform_cracking` function in `lib.rs` to use WaitAthena when the config option is enabled:

```rust
// In src/lib.rs
pub fn perform_cracking(text: &str, config: Config) -> Option<DecoderResult> {
    config::set_global_config(config);
    let text = text.to_string();
    
    // Clear any previous results when starting a new cracking session
    if get_config().top_results {
        storage::wait_athena_storage::clear_plaintext_results();
    }
    
    let initial_check_for_plaintext = check_if_input_text_is_plaintext(&text);
    if initial_check_for_plaintext.is_identified {
        // ... existing code ...
    }
    
    // ... rest of the function ...
}

/// Checks if the given input is plaintext or not
/// Used at the start of the program to not waste CPU cycles
fn check_if_input_text_is_plaintext(text: &str) -> CheckResult {
    let config = get_config();
    
    if config.top_results {
        let wait_athena_checker = Checker::<WaitAthena>::new();
        wait_athena_checker.check(text)
    } else {
        let athena_checker = Checker::<Athena>::new();
        athena_checker.check(text)
    }
}
```

### 9. Update the CLI Interface

Update the CLI interface to add a flag for enabling WaitAthena mode:

```rust
// In src/cli/mod.rs
// Add a new flag for WaitAthena mode
let matches = App::new("ciphey")
    // ... existing arguments ...
    .arg(
        Arg::with_name("top-results")
            .long("top-results")
            .help("Show all potential plaintexts found instead of exiting after the first one")
            .takes_value(false),
    )
    // ... rest of the arguments ...
    .get_matches();

// In the parse_cli_args function
pub fn parse_cli_args() -> (String, Config) {
    // ... existing code ...
    
    // Set top_results mode if the flag is present
    if matches.is_present("top-results") {
        config.top_results = true;
    }
    
    // ... rest of the function ...
}
```

## Testing Plan

1. Unit tests for the WaitAthena checker
2. Integration tests to verify that WaitAthena collects multiple plaintexts
3. Manual testing with different ciphertexts to ensure all plaintexts are collected

## Implementation Notes

1. The WaitAthena storage uses a thread-safe Mutex to store results, ensuring that multiple threads can safely add results
2. The timer module has been modified to display the collected results when the timer expires
3. Both Athena and WaitAthena checkers have comments indicating their relationship and differences
4. The config has been updated to support enabling WaitAthena mode via a command-line flag `--top-results`
5. No modifications to the searcher logic are needed as the checker will handle the collection of results
6. Human checker is not involved in the WaitAthena process to avoid interrupting the search
7. No deduplication logic is implemented as duplicates are unlikely

## Potential Challenges

1. Thread safety: Ensure that the storage mechanism is thread-safe
2. Error handling: The implementation will panic on mutex poisoning or other unexpected errors
3. User experience: Make sure the output is clear and helpful to users

## Future Improvements

1. Add filtering options for WaitAthena results
2. Implement sorting of results by confidence level
3. Add an option to save results to a file 

## Retrospective Implementation Insights

If implementing this feature again, I would make the following changes to improve the design and functionality:

1. **Deeper Integration with A* Search**: Rather than having the checker store results, I would modify the A* search algorithm to continue searching even after finding a valid plaintext when in `top_results` mode. This would eliminate the need for a separate global storage mechanism and make the feature more integrated with the core search algorithm.

2. **Result Scoring and Ranking**: I would implement a scoring system for results based on confidence levels, allowing for better sorting of results when displayed to the user. This could use metrics like:
   - Entropy of the plaintext
   - Checker confidence score
   - Number of decoders used to reach the plaintext
   - Presence of dictionary words or valid syntax

3. **Deduplication Strategy**: While the current implementation assumes duplicates are unlikely, I would add a simple deduplication mechanism that compciphey normalized versions of the plaintexts (case-insensitive, whitespace-normalized) to ensure truly unique results.

4. **Configurable Result Limit**: Instead of collecting all results until the timer expires, I would add a configurable limit to the number of results collected to prevent memory issues with very large result sets.

5. **Progress Updates During Search**: I would add periodic updates during the search process to show how many potential plaintexts have been found so far, giving users feedback before the timer expires.

6. **Result Categorization**: Group results by the type of checker that identified them (e.g., English text, passwords, specific formats) to make the output more organized and useful.

7. **Parallel Checker Execution**: Since we're no longer exiting on the first result, we could potentially run multiple checkers in parallel to speed up the search process.

8. **Memory Optimization**: The current implementation stores complete copies of results. I would optimize memory usage by storing references where possible and only copying data when necessary.

9. **Confidence Thresholds**: Add configurable confidence thresholds to filter out low-confidence results, reducing noise in the output.

10. **Export Functionality**: Add the ability to export all found plaintexts to a file for further analysis, especially useful for ambiguous or complex encodings.

These improvements would make the `wait_athena` feature more robust, efficient, and user-friendly while maintaining compatibility with the existing codebase. 