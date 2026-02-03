# Implementing a Decoder in Ciphey

This guide walks you through creating a new decoder for Ciphey. Decoders are the core components that transform encoded/encrypted text back into plaintext.

## Overview

A decoder in Ciphey:
- Implements the `Crack` trait
- Uses phantom types for type safety
- Returns a `CrackResult` containing the decoded text and metadata
- Is registered in three places: `mod.rs`, `DECODER_MAP`, and `filtration_system/mod.rs`

## Step-by-Step Guide

### Step 1: Create Your Decoder File

Create a new file in `src/decoders/` named `<your_decoder>_decoder.rs`.

```rust
//! Brief description of what this decoder does
//! Additional implementation notes

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// Documentation for your decoder struct.
/// Include a usage example:
/// ```
/// use ciphey::decoders::your_decoder::YourDecoder;
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decoder = Decoder::<YourDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decoder.crack("encoded_text", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "decoded_text");
/// ```
pub struct YourDecoder;
```

### Step 2: Implement the `Crack` Trait

```rust
impl Crack for Decoder<YourDecoder> {
    fn new() -> Decoder<YourDecoder> {
        Decoder {
            // The name shown to users and used in DECODER_MAP
            name: "YourDecoder",
            // Brief description (can be from Wikipedia)
            description: "Description of what this encoding/cipher is.",
            // Link to more information
            link: "https://en.wikipedia.org/wiki/Your_Encoding",
            // Tags for filtering (see "Understanding Tags" below)
            tags: vec!["your_tag", "decoder", "base"],
            // Popularity from 0.0 to 1.0 (affects search priority)
            popularity: 0.8,
            // Required for phantom type pattern
            phantom: std::marker::PhantomData,
        }
    }

    /// The main decoding function
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying YourDecoder with text {:?}", text);
        
        // Initialize the result
        let mut results = CrackResult::new(self, text.to_string());

        // Attempt decoding
        let decoded_text = match your_decode_function(text) {
            Some(decoded) => decoded,
            None => {
                debug!("YourDecoder decode failed");
                return results;
            }
        };

        // Verify the decoding produced a valid change
        if !check_string_success(&decoded_text, text) {
            info!("Failed: check_string_success returned false");
            return results;
        }

        // Run the checker to see if we found plaintext
        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);
        results.update_checker(&checker_result);

        results
    }

    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_description(&self) -> &str {
        self.description
    }

    fn get_link(&self) -> &str {
        self.link
    }
}
```

### Step 3: Implement Helper Functions

Add any helper functions needed for your decoding logic:

```rust
/// Helper function that performs the actual decoding
fn your_decode_function(text: &str) -> Option<String> {
    // Your decoding logic here
    // Return None if decoding fails
    // Return Some(decoded_string) on success
}
```

### Step 4: Add Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::YourDecoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // Helper to create a checker for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn successful_decoding() {
        let decoder = Decoder::<YourDecoder>::new();
        let result = decoder.crack("encoded_input", &get_athena_checker());
        let decoded = result.unencrypted_text.expect("Should decode successfully");
        assert_eq!(decoded[0], "expected_output");
    }

    #[test]
    fn empty_string_returns_none() {
        let decoder = Decoder::<YourDecoder>::new();
        let result = decoder.crack("", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn invalid_input_returns_none() {
        let decoder = Decoder::<YourDecoder>::new();
        let result = decoder.crack("invalid!@#$", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }
}
```

### Step 5: Register Your Decoder

You must register your decoder in **three** places:

#### 5a. Add to `mod.rs`

```rust
// At the top with other module declarations
/// Documentation for your decoder module
pub mod your_decoder;

// With other imports
use your_decoder::YourDecoder;

// Add to DecoderType enum
pub enum DecoderType {
    // ... existing variants ...
    /// Your decoder
    YourDecoder(your_decoder::YourDecoder),
}

// Add to DECODER_MAP
pub static DECODER_MAP: Lazy<HashMap<&str, DecoderBox>> = Lazy::new(|| {
    HashMap::from([
        // ... existing entries ...
        (
            "YourDecoder",  // Must match the `name` field exactly
            DecoderBox::new(Decoder::<YourDecoder>::new()),
        ),
    ])
});
```

#### 5b. Add to `filtration_system/mod.rs`

```rust
// Add import at the top
use crate::decoders::your_decoder::YourDecoder;

// In filter_and_get_decoders(), create instance and add to vector
pub fn filter_and_get_decoders(_text_struct: &DecoderResult) -> Decoders {
    // ... existing decoders ...
    let your_decoder = Decoder::<YourDecoder>::new();

    Decoders {
        components: vec![
            // ... existing entries ...
            Box::new(your_decoder),
        ],
    }
}
```

## Understanding Tags

Tags control how the decoder is filtered and affect the A* search heuristic.

### Important Tags

| Tag | Meaning | Effect on Search |
|-----|---------|------------------|
| `"decoder"` | This is an **encoder** (Base64, Hex, etc.) | Lower path cost, can be nested many times |
| `"base"` | Base-N family encoding | Used for filtering |
| `"decryption"` | This is a **cipher** (Caesar, Vigenère) | Higher path cost, multiple ciphers penalized |
| `"reciprocal"` | Self-inverse (encoding = decoding) | Used for search optimization |
| `"classic"` | Classical/historical cipher | Used for filtering |

### Encoder vs Cipher

The distinction is critical for the A* search algorithm:

- **Encoders** (tag: `"decoder"`): Can be nested many times. In CTFs, you might see `base64(base64(base64(text)))`. These have low path cost.
- **Ciphers** (no `"decoder"` tag): Rarely used, and multiple ciphers are extremely unlikely. These have high and escalating path cost.

### Tag Examples

```rust
// An encoder (like Base64)
tags: vec!["base64", "decoder", "base"],

// A cipher (like Caesar)
tags: vec!["caesar", "decryption", "classic", "reciprocal"],

// A simple transformation (like Reverse)
tags: vec!["reverse", "decoder", "reciprocal"],
```

## Understanding `CrackResult`

The `CrackResult` struct contains all information about a decoding attempt:

```rust
pub struct CrackResult {
    pub success: bool,              // Set by checker if plaintext found
    pub encrypted_text: String,     // Original input
    pub unencrypted_text: Option<Vec<String>>,  // Decoded output(s)
    pub decoder: &'static str,      // Name of decoder used
    pub checker_name: &'static str, // Name of checker that succeeded
    pub checker_description: &'static str,
    pub key: Option<String>,        // For ciphers that use keys
    pub description: &'static str,  // Decoder description
    pub link: &'static str,         // Reference link
}
```

### Returning Multiple Results

Some decoders (like Caesar) try multiple keys and may return multiple possibilities:

```rust
// Single result (most decoders)
results.unencrypted_text = Some(vec![decoded_text]);

// Multiple results (e.g., Caesar tries all 25 shifts)
let mut all_results = Vec::new();
for key in possible_keys {
    let decoded = decode_with_key(text, key);
    all_results.push(decoded);
}
results.unencrypted_text = Some(all_results);
```

### Setting a Key

For ciphers that use keys:

```rust
results.key = Some(key.to_string());
```

## Using Checkers

The `checker` parameter validates if decoded text is meaningful:

```rust
// Basic usage
let checker_result = checker.check(&decoded_text);
results.update_checker(&checker_result);

// For ciphers needing different sensitivity (e.g., Caesar)
use gibberish_or_not::Sensitivity;
let checker_with_sensitivity = checker.with_sensitivity(Sensitivity::Low);
let checker_result = checker_with_sensitivity.check(&decoded_text);
```

## Popularity Field

The `popularity` field (0.0 to 1.0) affects search priority:

- `1.0` - Very common (Base64, Hex)
- `0.8` - Common (Caesar, URL encoding)
- `0.5` - Moderate (various Base58 variants)
- `0.2` - Rare (Reverse, obscure encodings)

## Complete Example: Simple Decoder

Here's a complete minimal example:

```rust
//! Decodes ROT13 text (Caesar cipher with shift 13)

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::trace;

/// ROT13 decoder - a special case of Caesar cipher
pub struct Rot13Decoder;

impl Crack for Decoder<Rot13Decoder> {
    fn new() -> Decoder<Rot13Decoder> {
        Decoder {
            name: "ROT13",
            description: "ROT13 is a letter substitution cipher that replaces each letter with the 13th letter after it.",
            link: "https://en.wikipedia.org/wiki/ROT13",
            tags: vec!["rot13", "decoder", "reciprocal"],
            popularity: 0.6,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying ROT13 with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        if text.is_empty() {
            return results;
        }

        let decoded_text = rot13(text);

        if !check_string_success(&decoded_text, text) {
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);
        results.update_checker(&checker_result);

        results
    }

    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_description(&self) -> &str {
        self.description
    }

    fn get_link(&self) -> &str {
        self.link
    }
}

fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (base + (c as u8 - base + 13) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes};

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn decodes_hello() {
        let decoder = Decoder::<Rot13Decoder>::new();
        let result = decoder.crack("uryyb", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "hello");
    }

    #[test]
    fn empty_returns_none() {
        let decoder = Decoder::<Rot13Decoder>::new();
        let result = decoder.crack("", &get_checker()).unencrypted_text;
        assert!(result.is_none());
    }
}
```

## Checklist

Before submitting your decoder:

- [ ] File created at `src/decoders/<name>_decoder.rs`
- [ ] `Crack` trait fully implemented
- [ ] Doc comments on all public items
- [ ] Usage example in struct documentation
- [ ] Module added to `mod.rs`
- [ ] Variant added to `DecoderType` enum
- [ ] Entry added to `DECODER_MAP`
- [ ] Decoder added to `filtration_system/mod.rs`
- [ ] Unit tests cover success, empty input, and invalid input
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` applied

## Additional Resources

- `interface.rs` - The `Crack` trait and `Decoder` struct definitions
- `crack_results.rs` - The `CrackResult` struct
- `base64_decoder.rs` - Good example of a simple encoder
- `caesar_decoder.rs` - Good example of a cipher with multiple key attempts
- `reverse_decoder.rs` - Minimal decoder example
- `../AGENTS.md` - Overall project architecture and heuristic design
