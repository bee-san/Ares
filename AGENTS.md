# AGENTS.md - Ciphey Codebase Guide

This document provides essential information for AI coding agents working in this repository.

## Project Overview

**Ciphey** is an automated decoding and decryption tool written in Rust. It's the successor to the Python-based [Ciphey](https://github.com/ciphey/ciphey), offering significant performance improvements through Rust's speed and parallel processing via Rayon.

- **Language**: Rust (Edition 2021)
- **Package Name**: `ciphey`
- **Entry Points**: `src/lib.rs` (library API), `src/main.rs` (CLI)

## Build Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build (optimized)
cargo check                    # Type-check without building
```

## Test Commands

```bash
cargo test                              # Run all tests
cargo test <test_name>                  # Run a single test by name
cargo test <test_name> -- --nocapture   # Run test with stdout visible
cargo test --test integration_test      # Run integration tests only
cargo test --lib                        # Run unit tests only
cargo test -- --ignored                 # Run ignored tests
```

### Running Specific Tests
```bash
cargo test test_perform_cracking_returns           # Exact test name
cargo test base64                                  # All tests containing "base64"
cargo test decoders::                              # All tests in decoders module
```

## Lint and Format Commands

```bash
cargo fmt --all -- --check    # Check formatting (CI uses this)
cargo fmt --all               # Auto-format all code
cargo clippy                  # Run linter
```

## Code Style Guidelines

### Documentation Requirements

The codebase enforces documentation via `#![warn(missing_docs, clippy::missing_docs_in_private_items)]`:
- All public items MUST have doc comments (`///`)
- Private items SHOULD have doc comments
- Use `# Panics` and `# Errors` sections where applicable

### Import Organization

Organize imports in this order, separated by blank lines:
```rust
// 1. Standard library
use std::collections::HashMap;
// 2. External crates
use rayon::prelude::*;
// 3. Crate-level imports
use crate::config::Config;
// 4. Super/self imports
use super::crack_results::CrackResult;
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Modules | snake_case | `base64_decoder` |
| Functions | snake_case | `perform_cracking` |
| Types/Structs | PascalCase | `DecoderResult` |
| Traits | PascalCase | `Crack`, `Check` |
| Constants | SCREAMING_SNAKE_CASE | `PRUNE_THRESHOLD` |

### Error Handling

- **`Option<T>`** for values that may or may not exist
- **`Result<T, E>`** for operations that can fail with specific errors
- **`expect()`** with descriptive messages for unrecoverable errors
- **Log errors** using the `log` crate (`trace!`, `debug!`, `info!`, `warn!`)

### Testing Patterns

- Unit tests go in `#[cfg(test)] mod tests` at the bottom of each file
- Use `#[serial_test::serial]` for tests that access the database
- Use `#[serial_test::parallel]` for tests that can run concurrently
- Create test database fixtures with `TestDatabase::default()` and `set_test_db_path()`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_successful_decoding() {
        let _test_db = TestDatabase::default();
        set_test_db_path();
        // test implementation
    }
}
```

## Architecture Overview

### Core Components

| Directory | Purpose |
|-----------|---------|
| `src/decoders/` | Decoding implementations (Base64, Caesar, etc.) |
| `src/checkers/` | Plaintext detection (English, LemmeKnow, Regex) |
| `src/searchers/` | Search algorithms (A*, BFS) |
| `src/config/` | Global configuration management |
| `src/cli/` | Command-line interface |
| `src/storage/` | Database and caching |
| `src/tui/` | Terminal User Interface (Ratatui-based) |

### Key Traits

- **`Crack`** (`src/decoders/interface.rs`): All decoders implement this
- **`Check`** (`src/checkers/checker_type.rs`): All checkers implement this

### Adding a New Decoder

1. Create `src/decoders/my_decoder.rs`
2. Implement the `Crack` trait for `Decoder<MyDecoder>` (use phantom types)
3. Add module to `src/decoders/mod.rs`
4. Register in `DECODER_MAP` in `src/decoders/mod.rs`
5. Add to filtration system in `src/filtration_system/mod.rs`

### Adding a New Checker

1. Create `src/checkers/my_checker.rs`
2. Implement the `Check` trait for `Checker<MyChecker>`
3. Add module to `src/checkers/mod.rs`
4. Add variant to `CheckerTypes` enum and register in `CHECKER_MAP`

## Common Patterns

### Global Config Access
```rust
use crate::config::get_config;
let config = get_config();
```

### Creating Checker for Tests
```rust
let athena_checker = Checker::<Athena>::new();
let checker = CheckerTypes::CheckAthena(athena_checker);
```

## Dependencies Note

Dependencies in `Cargo.toml` should be kept in **alphabetical order**.

## CI/CD

The GitHub Actions workflow (`.github/workflows/quickstart.yml`) runs:
1. `cargo check` - Compilation check
2. `cargo test` - All tests
3. `cargo fmt --all -- --check` - Format verification
4. `cargo clippy` - Lint check

Ensure all four pass before submitting PRs.

## A* Search Algorithm & Heuristic Design

### Overview

The primary search algorithm is A* (`src/searchers/astar.rs`), which finds the correct sequence of decoders to transform ciphertext into plaintext. The algorithm uses `f = g + h` where:
- **g** = path complexity (cost so far)
- **h** = heuristic (estimated cost to reach plaintext)

### Occam's Razor Principle

The heuristic is designed with **Occam's Razor** in mind: simpler explanations (shorter/less complex decoding paths) are preferred. This is critical because:

1. **Encoders** (Base64, Hex, URL, etc.) can be nested many times — this is common in CTFs and real-world obfuscation
2. **Ciphers** (Caesar, Vigenère, etc.) are rarely used, and multiple ciphers are extremely unlikely

### Encoder vs Cipher Classification

Decoders are classified by their `tags` field:
- **Encoders**: Have the `"decoder"` tag (e.g., Base64, Base32, Hex, Binary, URL)
- **Ciphers**: Do NOT have `"decoder"` tag (e.g., Caesar, Vigenère, Railfence)

Check `is_encoder()` in `src/searchers/helper_functions.rs` for the implementation.

### Path Complexity Calculation

The `calculate_path_complexity()` function implements Occam's Razor:

| Decoder Type | Cost |
|--------------|------|
| First encoder in path | 0.7 |
| Repeated same encoder (e.g., base64 → base64) | 0.2 |
| Different encoder | 0.7 |
| First cipher | 2.0 |
| Second cipher | 4.0 (escalating) |
| Third cipher | 6.0 (escalating) |

**Example costs:**
- `base64 × 10` = 0.7 + 0.2×9 = **2.5** (cheap — nested encoding expected)
- `caesar → vigenere` = 2.0 + 4.0 = **6.0** (expensive — multiple ciphers unlikely)
- `base64 × 5 → caesar` = 1.5 + 2.0 = **3.5** (moderate)

### Heuristic Components

The `generate_heuristic()` function estimates distance to plaintext using:

1. **Shannon Entropy** (`calculate_entropy()`): Lower entropy = more plaintext-like
   - Plaintext English: ~0.4-0.5 normalized
   - Base64 encoded: ~0.75-0.85 normalized
   - Random/encrypted: ~0.95-1.0 normalized

2. **Decoder Success Rate**: Uses learned statistics from `DECODER_SUCCESS_RATES`

3. **String Quality**: Penalizes non-printable characters and garbled text

### Key Files

| File | Purpose |
|------|---------|
| `src/searchers/astar.rs` | A* search implementation |
| `src/searchers/helper_functions.rs` | Heuristic calculations, path complexity, entropy |
| `src/searchers/bfs.rs` | Alternative BFS search (simpler, less optimal) |
| `src/filtration_system/mod.rs` | Decoder filtering and selection |

### Modifying the Heuristic

When adjusting the heuristic, keep in mind:
1. Path complexity (`g`) should favor repeated same-encoders over diverse decoders
2. Ciphers should always be expensive (real-world data rarely has multiple ciphers)
3. Entropy is a good proxy for "plaintext-likeness"
4. Test with both simple cases (single base64) and complex cases (nested encoding + cipher)

## Database Schema

The SQLite database is stored at `~/.ciphey/database.sqlite` and serves two purposes:

### Cache Table (Performance + Analytics)

Stores previously decoded results to avoid re-computation:

```sql
CREATE TABLE cache (
    encoded_text TEXT PRIMARY KEY NOT NULL,  -- Original input
    decoded_text TEXT NOT NULL,               -- Final plaintext
    path JSON NOT NULL,                       -- Decoder sequence (serialized CrackResult[])
    successful BOOLEAN NOT NULL DEFAULT true,
    execution_time_ms INTEGER NOT NULL,
    input_length INTEGER NOT NULL,            -- For analytics: size → performance correlation
    decoder_count INTEGER NOT NULL,           -- Path length without JSON parsing
    checker_name TEXT,                        -- Which checker confirmed the result
    key_used TEXT,                            -- For ciphers (e.g., Caesar shift)
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Human Rejection Table (Checker Analytics)

Tracks false positives rejected by the human checker for future analysis:

```sql
CREATE TABLE human_rejection (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plaintext TEXT NOT NULL,                  -- The rejected candidate
    encoded_text TEXT,                        -- Original input (NULL if unavailable)
    checker TEXT NOT NULL,                    -- Which checker suggested this
    checker_description TEXT,
    check_description TEXT,                   -- What the checker thought it found
    decoder_path JSON,                        -- Path that led here (NULL if unavailable)
    rejection_count INTEGER NOT NULL DEFAULT 1,  -- Upsert counter
    first_rejected DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_rejected DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(plaintext, checker)                -- For upsert logic
);
```

### Key Database Functions

| Function | Purpose |
|----------|---------|
| `insert_cache(&CacheEntry)` | Store new decode result |
| `read_cache(&String)` | Look up by encoded_text |
| `insert_human_rejection(plaintext, check_result, encoded_text?, decoder_path?)` | Record rejection with upsert |
| `read_human_rejection(&String)` | Look up by plaintext |

### Database Structs

- **`CacheEntry`**: Input struct for inserting cache rows
- **`CacheRow`**: Output struct when reading cache rows  
- **`HumanRejectionRow`**: Output struct when reading rejection rows

**Note**: Users upgrading from older versions must delete `~/.ciphey/database.sqlite` due to schema changes.

## TUI Architecture

The Terminal User Interface (`src/tui/`) is built with [Ratatui](https://ratatui.rs/) and provides an interactive experience for decoding operations.

### TUI Components

| File | Purpose |
|------|---------|
| `src/tui/mod.rs` | Module exports and public API |
| `src/tui/app.rs` | Application state machine (`AppState` enum) |
| `src/tui/run.rs` | Main entry point and event loop |
| `src/tui/ui.rs` | Screen rendering functions |
| `src/tui/input.rs` | Keyboard input handling |
| `src/tui/colors.rs` | Color scheme from config |
| `src/tui/spinner.rs` | Loading animation and quotes |
| `src/tui/human_checker_bridge.rs` | Channel-based human checker communication |
| `src/tui/widgets/` | Reusable UI widgets (text panels, path viewer, step details) |

### Application States

The TUI uses a state machine defined in `AppState`:

```rust
pub enum AppState {
    Loading { ... },           // Decoding in progress
    HumanConfirmation { ... }, // Waiting for user to confirm plaintext
    Results { ... },           // Successful decode, showing results
    Failure { ... },           // Decode failed
}
```

### Human Checker Bridge

When running in TUI mode, the human checker cannot use stdin directly (terminal is in raw mode). The `human_checker_bridge` module provides channel-based communication:

```rust
// In run.rs (TUI initialization):
init_tui_confirmation_channel();
let confirmation_receiver = take_confirmation_receiver();

// In human_checker.rs (detection):
if is_tui_confirmation_active() {
    // Use channel-based confirmation
    request_tui_confirmation(check_result)
} else {
    // Fall back to CLI stdin
}
```

**Key functions:**
- `init_tui_confirmation_channel()` - Initialize before starting TUI
- `take_confirmation_receiver()` - Get receiver for event loop (one-time)
- `request_tui_confirmation(&CheckResult)` - Request confirmation, blocks until response
- `is_tui_confirmation_active()` - Check if TUI mode is active

### Adding a New TUI State

1. Add variant to `AppState` enum in `src/tui/app.rs`
2. Add transition method to `App` struct (e.g., `set_my_state()`)
3. Add rendering function in `src/tui/ui.rs` (e.g., `draw_my_screen()`)
4. Update `draw()` function to handle the new state
5. Update `handle_key_event()` in `src/tui/input.rs` for state-specific keys

### TUI Color Scheme

Colors are derived from the config's `colourscheme` hashmap in `src/tui/colors.rs`:

```rust
let colors = TuiColors::from_config(&config);
// Available styles: colors.primary, colors.success, colors.error,
// colors.text, colors.muted, colors.highlight, colors.accent, etc.
```

