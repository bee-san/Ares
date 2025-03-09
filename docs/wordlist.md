# Wordlist Checker Implementation Plan

## Overview

The Wordlist Checker will check if the input text exactly matches any word in a user-provided wordlist. This checker will run if the user provides a `--wordlist` argument pointing to a file containing newline-separated words or specifies a wordlist in the config file (CLI argument takes precedence if both are specified).

## Implementation Steps

### 1. Update Config Structure

1. Modify `src/config/mod.rs` to add a new field for the wordlist:

```rust 
pub struct Config {
    // ... existing fields
    
    /// Path to the wordlist file. Will be overridden by CLI argument if provided.
    pub wordlist_path: Option<String>,
    
    /// Wordlist data structure (loaded from file). CLI takes precedence if both config and CLI specify a wordlist.
    #[serde(skip)]
    pub wordlist: Option<std::collections::HashSet<String>>,
}
```

2. Update the `Default` implementation for `Config` to set these new fields to `None`.

3. Update the config file handling to support a `wordlist` key that points to a wordlist file path:

```rust
// In the function that loads the config file
// (likely in src/config/mod.rs) 
pub fn get_config_file_into_struct() -> Config {
    // ... existing code
    
    // If wordlist is specified in config file, set it in the config struct
    if let Some(wordlist_path) = config_values.get("wordlist") {
        config.wordlist_path = Some(wordlist_path.to_string());
        
        // Load the wordlist here in the config layer
        match load_wordlist(wordlist_path) {
            Ok(wordlist) => {
                config.wordlist = Some(wordlist);
            },
            Err(e) => {
                // Critical error - exit if config specifies wordlist but can't load it
                eprintln!("Can't load wordlist at '{}'. Either fix or remove wordlist from config file at '{}'", 
                    wordlist_path, config_file_path);
                std::process::exit(1);
            }
        }
    }
    
    // ... rest of the function
}
```

### 2. Update CLI Arguments

1. Modify `src/cli/mod.rs` to add the wordlist argument to the `Opts` struct:

```rust
pub struct Opts {
    // ... existing fields
    
    /// Path to a wordlist file containing newline-separated words
    /// The checker will match input against these words exactly
    /// Takes precedence over config file if both specify a wordlist
    #[arg(long)]
    wordlist: Option<String>,
}
```

2. Update the `cli_args_into_config_struct` function to handle the new wordlist argument:

```rust
fn cli_args_into_config_struct(opts: Opts, text: String) -> (String, Config) {
    // ... existing code
    
    if let Some(wordlist_path) = opts.wordlist {
        config.wordlist_path = Some(wordlist_path.clone());
        
        // Load the wordlist here in the CLI layer
        match load_wordlist(&wordlist_path) {
            Ok(wordlist) => {
                config.wordlist = Some(wordlist);
            },
            Err(e) => {
                // Critical error - exit if wordlist is specified but can't be loaded
                eprintln!("Can't load wordlist at '{}'", wordlist_path);
                std::process::exit(1);
            }
        }
    }
    
    // ... rest of the function
}
```

3. Update any help text or documentation to include the new `--wordlist` option:

```rust
// In the help text for the CLI
/// Path to a wordlist file containing newline-separated words
/// The checker will perform exact matching against these words
/// Takes precedence over config file if both specify a wordlist
#[arg(long, help = "Path to a wordlist file with newline-separated words for exact matching")]
wordlist: Option<String>,
```

### 3. Create Wordlist Checker Module

[Previous implementation remains the same, with updated doc comments]

### 4. Update Checkers Module

[Previous implementation remains the same]

### 5. Update Athena Checker

[Previous implementation remains the same]

### 6. Implement Wordlist Loading with mmap2

Add the necessary dependency to Cargo.toml:

```toml
[dependencies]
# ... existing dependencies
memmap2 = "0.9.0"
```

Add a public function to load the wordlist in `src/config/mod.rs`:

```rust
use memmap2::Mmap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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
    if file_size < 10_000_000 { // 10MB threshold
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
        if let Err(_) = std::str::from_utf8(&mmap) {
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
```

### 7. Library API Integration

[Previous implementation remains the same]

### 8. CLI Implementation

[Previous implementation remains the same]

## Performance Considerations

[Previous implementation remains the same]

## Error Handling

1. **Wordlist Loading Failure**: If a wordlist is specified (via CLI or config) but can't be loaded:
   - Print a clear error message indicating the file path
   - For config file failures, indicate the config file location
   - Exit with a non-zero status code in both cases
   - Do not fall back to running without a wordlist

2. **Invalid UTF-8**: If the wordlist file contains invalid UTF-8:
   - Panic with a clear error message about UTF-8 invalidity
   - Do not attempt to proceed with partial wordlist loading

3. **Library API Errors**: When used as a library:
   - Accept only pre-loaded HashSet to avoid file I/O errors
   - Move all file handling to the CLI/config layer

## Matching Behavior

1. **Exact Matching**: The wordlist checker performs exact, case-sensitive matching:
   - "Password" and "password" are different words
   - Leading/trailing whitespace is trimmed from wordlist entries
   - Words with internal whitespace or special characters match exactly

2. **No Partial Matching**: Only complete words are matched, not substrings

## Testing Strategy

[Previous implementation remains the same]

## Implementation Notes

1. CLI argument (`--wordlist`) takes precedence over config file if both specify a wordlist
2. All wordlist loading fails fatally - there is no fallback behavior
3. The checker uses HashSet for O(1) lookups for performance
4. Memory mapping is used for files over 10MB to improve performance and memory usage 
5. Empty lines in wordlist files are ignored
6. Case-sensitive matching only (no case-insensitive option)
7. Only loaded once at startup - file changes not detected during runtime 

## Future Improvements

[Previous implementation remains the same]
        
        Ok(wordlist)
    } else {
        // For large files, use memory mapping
        let mmap = unsafe { Mmap::map(&file)? };
        let mut wordlist = HashSet::new();
        
        // Process the memory-mapped file
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
```

### 7. Library API Integration

The library should accept a pre-loaded HashSet directly rather than loading the wordlist itself:

```rust
// In src/lib.rs or appropriate module

/// LibraryInput struct should be updated to include wordlist
pub struct LibraryInput {
    // ... existing fields
    
    /// Pre-loaded wordlist (allows library users to provide wordlist directly)
    pub wordlist: Option<HashSet<String>>,
}

impl LibraryInput {
    // ... existing methods
    
    /// Set a pre-loaded wordlist
    pub fn with_wordlist(mut self, wordlist: HashSet<String>) -> Self {
        self.wordlist = Some(wordlist);
        self
    }
}

/// When converting LibraryInput to Config, handle wordlist
fn library_input_to_config(input: LibraryInput) -> Config {
    let mut config = Config::default();
    
    // ... existing conversion code
    
    // Handle wordlist - just pass the pre-loaded HashSet
    config.wordlist = input.wordlist;
    
    config
}

/// The main cracking function doesn't need to load the wordlist
pub fn perform_cracking(text: &str, config: Config) -> Option<DecoderResult> {
    // ... existing code
    
    // The wordlist is already loaded by the CLI/config layer
    // Just set the config
    config::set_global_config(config);
    
    // ... rest of the function
}
```

### 8. CLI Implementation

The CLI should handle loading the wordlist and passing it to the library:

```rust
// In src/main.rs or appropriate CLI module

fn main() {
    // ... existing code
    
    let opts: Opts = Opts::parse();
    let mut config = get_config();
    
    // Handle wordlist if provided
    if let Some(wordlist_path) = &opts.wordlist {
        match load_wordlist(wordlist_path) {
            Ok(wordlist) => {
                config.wordlist = Some(wordlist);
            },
            Err(e) => {
                eprintln!("Error loading wordlist '{}': {}", wordlist_path, e);
                std::process::exit(1);
            }
        }
    }
    
    // Pass the config with pre-loaded wordlist to the library
    let result = perform_cracking(&text, config);
    
    // ... rest of the function
}
```

## Performance Considerations

1. **HashSet for O(1) Lookups**: Using a HashSet for the wordlist ensures constant-time lookups, making the checker very fast.

2. **Memory Mapping for Large Files**: Using the `memmap2` crate for large wordlist files (>10MB) to avoid loading the entire file into memory at once, which is crucial for handling wordlists with millions of entries.

3. **Lazy Loading**: The wordlist is only loaded when needed, not at program startup.

4. **Memory Efficiency**: The wordlist is stored as a HashSet of Strings, which is memory-efficient for exact matching.

5. **Early Exit**: The wordlist checker runs before other checkers if a wordlist is provided, allowing for early exit if a match is found.

6. **Separation of Concerns**: The CLI/config layer is responsible for loading the wordlist, while the library just uses the pre-loaded HashSet, maintaining a clean separation of concerns.

## Error Handling

1. **Missing Wordlist File**: If the user provides a `--wordlist` argument but the file doesn't exist or can't be read, the program should:
   - Print a clear error message indicating the problem
   - Exit with a non-zero status code
   - Not attempt to continue without the wordlist

2. **Invalid Wordlist Format**: If the wordlist file contains invalid UTF-8 or other issues:
   - Print a clear error message
   - Exit with a non-zero status code

3. **Library API Errors**: When used as a library, the API should accept a pre-loaded HashSet, avoiding file I/O errors at the library level.

## Matching Behavior

1. **Exact Matching**: The wordlist checker performs exact, case-sensitive matching. This means:
   - "Password" and "password" are considered different words
   - Leading/trailing whitespace is trimmed from words in the wordlist file
   - Words with internal whitespace or special characters are matched exactly as they appear

2. **No Partial Matching**: The checker only matches complete words, not substrings.

## Testing Strategy

1. **Unit Tests**: Test the wordlist checker with various inputs, including matches, non-matches, and when no wordlist is provided.

2. **Integration Tests**: Test the entire cracking process with a wordlist to ensure it works end-to-end.

3. **Error Handling Tests**: Test error cases such as non-existent wordlist files or invalid formats.

## Implementation Notes

1. The wordlist checker is only active when a wordlist is provided via the `--wordlist` argument or in the config file.

2. The checker uses a HashSet for O(1) lookups, making it very efficient.

3. The wordlist is loaded by the CLI/config layer, not by the library, maintaining a clean separation of concerns.

4. The checker performs exact matching, so it's case-sensitive and whitespace-sensitive.

5. Empty lines in the wordlist file are ignored.

6. The wordlist checker runs alongside other checkers, not replacing them, but it runs first for efficiency.

7. The config file can contain a `wordlist` key pointing to a wordlist file, which will be loaded automatically.

## Future Improvements

1. Add support for case-insensitive matching as an option.

2. Add support for multiple wordlist files.

3. Add support for wordlist formats other than newline-separated (e.g., CSV).

4. Add a progress indicator when loading large wordlists.

5. Implement wordlist caching to avoid reloading the same wordlist multiple times.

## Notes

the checker needs to be stand alone called `wordlist.rs`. If we wanted to, we could change the code to use it. Athena is a checker itself, and it just calls other checkers. Do not put much logic for this checker into Athena, Athena should just call it.

The CLI argument should take precedence. If the config is set, ALWAYS use it.

If we can't load from config, also exit. Do not warn. This is on the user to fix. Instead, we can print the config file location and tell them we can't load the wordlist. Something like "Can't load wordlist at (WORDLIST LOCATION). Either fix or remove WORDLIST from config file at (CONFIG FILE LOCATION)

Non UTF-8 - We must assume the wordlist could be in any format. We can try converting to utf-8, and if it doesn't work we can panic

Athena has a regex checker. If the user uses the regex checker, all other checkers should be disabled. Similarly, if the user uses the wordlist checker, all other checkers should be disabled.