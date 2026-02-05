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

### Batched Decoder Exploration

The A* algorithm uses **batched decoder exploration** to balance speed and correctness:

#### The Problem

Running all decoders in parallel at each node causes race conditions where slow decoders (like Vigenère) can find false positives that beat correct results from faster decoder chains (like Base64 → Caesar).

#### The Solution

At each node, only the **top X decoders** (ranked by estimated cost) are tried. Remaining decoders are queued as "continuation nodes" for later exploration.

#### Config Options

| Option | Default | Purpose |
|--------|---------|---------|
| `decoder_batch_size` | 5 | Number of decoders to try per node expansion |
| `depth_penalty` | 0.5 | Cost added per depth level to favor shallow exploration |

#### Decoder Ranking (`rank_decoders()`)

Decoders are ranked by estimated cost considering:
1. **Path complexity**: Encoders cheap (~0.7), ciphers expensive (~2.0+)
2. **Same-encoder bonus**: Repeated same encoder very cheap (0.2)
3. **Historical success rates**: From `DECODER_SUCCESS_RATES`
4. **Input entropy**: High entropy favors encoders

#### Continuation Nodes

When a node has remaining untried decoders:
1. Results from tried decoders become child nodes (for further exploration)
2. A "continuation node" is created with the remaining decoders
3. The continuation node has a small penalty (+0.05) to prefer exploring results first
4. A* will eventually revisit the continuation node if other paths don't succeed

#### Depth Penalty

The depth penalty ensures shallow unexplored paths eventually become competitive:

```
total_cost = path_complexity + (depth × depth_penalty) + heuristic
```

Example with `depth_penalty = 0.5`:
- Base64 at depth 1: 0.7 + 0.5 = 1.2
- Base64 × 5 at depth 5: 1.5 + 2.5 = 4.0
- Caesar at depth 1: 3.0 + 0.5 = 3.5 (eventually competitive with deep encoder paths)

#### Node Structure

```rust
struct AStarNode {
    state: DecoderResult,      // Current text and path
    depth: usize,              // Path length
    cost: f32,                 // g = path_complexity + depth_penalty
    heuristic: f32,            // h = estimated cost to plaintext
    total_cost: f32,           // f = g + h
    node_type: NodeType,       // Regular (with untried_decoders) or Result
}

enum NodeType {
    Regular {
        next_decoder: Option<String>,  // Specific decoder to try (if any)
        untried_decoders: Vec<String>, // Decoders not yet tried at this node
    },
    Result,  // Successfully found plaintext
}
```

#### Why This Works

1. **Encoders naturally rank first** (cost ~0.7 vs ~3.0 for ciphers)
2. **Depth penalty prevents infinite encoder chains** (eventually ciphers become competitive)
3. **Continuation nodes ensure all decoders eventually get tried**
4. **A* priority queue orders everything optimally**

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

### Wordlist Table (Fast Dictionary Lookup)

Stores words for the wordlist checker with bloom filter acceleration:

```sql
CREATE TABLE wordlist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT NOT NULL UNIQUE,           -- The dictionary word
    source TEXT NOT NULL,                 -- Origin (e.g., "user_import", "builtin")
    enabled BOOLEAN NOT NULL DEFAULT true, -- Whether word is active for matching
    added_date DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_wordlist_word ON wordlist(word);
CREATE INDEX idx_wordlist_enabled ON wordlist(enabled);
```

### Key Wordlist Functions

| Function | Purpose |
|----------|---------|
| `insert_word(&str, &str)` | Add single word with source (enabled by default) |
| `insert_words_batch(&[(&str, &str)])` | Bulk insert words |
| `word_exists(&str) -> bool` | Check if word exists AND is enabled |
| `import_wordlist(&HashSet<String>, &str)` | Import words from HashSet |
| `get_word_count() -> i64` | Count of enabled words |
| `set_word_enabled(&str, bool)` | Enable/disable a word |
| `set_words_enabled_batch(&[&str], bool)` | Batch enable/disable |
| `get_disabled_words() -> Vec<WordlistRow>` | List all disabled words |
| `get_disabled_word_count() -> i64` | Count of disabled words |
| `read_word(&str) -> Option<WordlistRow>` | Get word details (regardless of enabled) |

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
    Settings { ... },          // Runtime settings configuration
    ListEditor { ... },        // String list editor modal
    WordlistManager { ... },   // Wordlist file management
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

Ciphey uses a **consistent 5-color scheme** throughout the entire application (CLI, TUI, and setup wizard):

| Color | Purpose | Default RGB | Used For |
|-------|---------|-------------|----------|
| `informational` | Primary/neutral color | 255,215,0 (Gold) | Status messages, neutral text, primary accent |
| `warning` | Warning messages | 255,0,0 (Red) | Cautions, alerts, potential issues |
| `success` | Success indicators | 0,255,0 (Green) | Successful operations, confirmations |
| `error` | Error messages | 255,0,0 (Red) | Error messages, failures (same as warning by default) |
| `question` | Interactive prompts | 255,215,0 (Gold) | Questions, prompts, user input requests |

**Color Storage:**

Colors are stored in the config file (`~/.ciphey/config.toml`) in the `colourscheme` HashMap:

```toml
[colourscheme]
informational = "255,215,0"
warning = "255,0,0"
success = "0,255,0"
error = "255,0,0"
question = "255,215,0"
```

**Predefined Themes:**

Ciphey includes 6 predefined themes (available in setup wizard and TUI settings):

1. **Cappuccino** - Warm, cozy colors
2. **Darcula** - Dark theme with vibrant accents
3. **GirlyPop** - Pink and pastel theme
4. **Autumnal Vibes** - Earth tones and autumn colors
5. **Skeletal** - High contrast black and white
6. **Default** - Classic terminal colors

All themes are defined in `src/tui/setup_wizard/themes.rs` (exported as `pub mod`).

**Theme Picker in TUI Settings:**

Users can change their color theme at runtime via the TUI Settings panel:

1. Press `Ctrl+S` to open settings
2. Navigate to "Colors" section
3. Select "Theme Preset" field (first field in Colors section)
4. Press `Enter` to open theme picker modal
5. Navigate themes with `Up/Down` or `j/k`
6. Press `Enter` to apply selected theme
7. Individual color fields can be manually tweaked after applying a theme
8. Press `Ctrl+S` to save changes to config

**Custom Themes:**

- Select "Custom..." option in theme picker (last option)
- Enter RGB values for each color (format: `255,128,64`)
- Use `Tab` to cycle between fields
- Press `Enter` to apply
- Custom colors are not saved unless applied and config saved with `Ctrl+S`

**Color Usage Guidelines for Developers:**

When adding new UI elements:

- **TUI**: Use `TuiColors` struct from `src/tui/colors.rs` - colors are loaded from config
- **CLI**: Use functions from `src/cli_pretty_printing/mod.rs` (e.g., `statement()`, `success()`)
- **Setup Wizard**: Uses `ColorScheme` from `src/tui/setup_wizard/themes.rs`

**Do NOT:**
- Hardcode RGB colors in UI code
- Add new color categories without updating this document
- Use `statement` color (removed - use `informational` instead)

**Example TUI usage:**
```rust
let colors = TuiColors::from_config(&config);
let text_style = colors.success;  // Use success color
let title_style = colors.title;   // Uses informational with bold
```

**Example CLI usage:**
```rust
use crate::cli_pretty_printing::{statement, success, warning};

println!("{}", statement("Processing...", Some("informational")));
println!("{}", success("Done!"));
println!("{}", warning("Low disk space"));
```

Colors are derived from the config's `colourscheme` hashmap in `src/tui/colors.rs`:

```rust
let colors = TuiColors::from_config(&config);
// Available styles: colors.primary, colors.success, colors.error,
// colors.text, colors.muted, colors.highlight, colors.accent, etc.
```

## TUI Settings Panel

The TUI includes a comprehensive **Settings Panel** that allows users to modify their configuration at runtime without editing files or restarting the application.

### Accessing Settings

- **Global Keybinding**: Press `Ctrl+S` from any state (except HumanConfirmation) to open settings
- The settings panel opens as a full-screen overlay

### Settings Architecture

| File | Purpose |
|------|---------|
| `src/tui/settings/mod.rs` | Module exports and validation functions |
| `src/tui/settings/model.rs` | `SettingsModel`, `SettingsSection`, `SettingField` data structures |
| `src/tui/settings/validation.rs` | Field validation with `parse_input()` function |
| `src/tui/widgets/settings_panel.rs` | Main settings form renderer |
| `src/tui/widgets/list_editor.rs` | String list editor modal |
| `src/tui/widgets/wordlist_manager.rs` | Wordlist management modal |

### Settings Organization

Settings are organized into **5 sections**:

#### 1. General
- `verbose` - Enable verbose logging (Boolean)
- `timeout` - Search timeout in seconds (Integer)
- `top_results` - Number of results to show (Integer)
- `api_mode` - Enable API mode (Boolean)
- `regex` - Custom regex pattern (String, optional)

#### 2. Checkers
- `human_checker_on` - Enable human checker (Boolean)
- `enhanced_detection` - Enable enhanced detection (Boolean)
- `model_path` - Path to language model (String, optional)
- `wordlist_manager` - Open wordlist manager (ActionButton)

#### 3. LemmeKnow
- `min_rarity` - Minimum rarity (Float, 0.0-1.0)
- `max_rarity` - Maximum rarity (Float, 0.0-1.0)
- `boundaryless` - Enable boundaryless mode (Boolean)
- `tags` - Include tags (StringList)
- `exclude_tags` - Exclude tags (StringList)

#### 4. Search Tuning
- `depth_penalty` - Depth penalty factor (Float)
- `decoder_batch_size` - Decoders per batch (Integer)

#### 5. Colors
- `informational` - Informational color (Color)
- `warning` - Warning color (Color)
- `success` - Success color (Color)
- `error` - Error color (Color)
- `question` - Question color (Color)

### Settings Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Cycle through sections |
| `↑/↓` | Navigate fields within section |
| `Enter` | Edit selected field |
| `Space` | Toggle boolean fields |
| `Esc` | Cancel edit or close settings |
| `Ctrl+S` | Save settings and return |

### Field Types

#### Boolean
- Toggle with `Space` key
- Display: `[✓]` for true, `[ ]` for false

#### Integer/Float
- Press `Enter` to edit
- Type new value
- Validation enforced (range checks, type checks)

#### String
- Press `Enter` to edit
- Type new value or leave empty for `None`

#### StringList (e.g., tags, exclude_tags)
- Press `Enter` to open **List Editor Modal**
- Navigate items with arrow keys
- Add new items by typing and pressing `Enter`
- Delete items with `Delete` or `Backspace`
- Press `Esc` to return to settings

#### ActionButton (e.g., wordlist_manager)
- Press `Enter` to trigger action
- Opens the **Wordlist Manager Modal**

### List Editor Modal

Appears when editing `StringList` fields like `tags` or `exclude_tags`.

**Layout:**
- Top: Current items list with selection
- Bottom: Input field for new items

**Keybindings:**
| Key | Action |
|-----|--------|
| `↑/↓` | Navigate current items |
| `Delete` | Remove selected item |
| `Enter` | Add new item from input field |
| `Esc` | Save and return to settings |

### Wordlist Manager Modal

Manages wordlist files used by the wordlist checker.

**Layout:**
- Table view showing: Path, Enabled status
- Input field at bottom for adding new wordlists
- Pending changes indicator

**Keybindings:**
| Key | Action |
|-----|--------|
| `↑/↓` | Navigate wordlist rows |
| `Space` | Toggle enabled/disabled |
| `Tab` | Cycle focus (Table → AddPath → Done) |
| `Enter` | Add new wordlist (when focused on input) or confirm (when focused on Done) |
| `Esc` | Discard changes and return |

**Future work**: Database integration for persisting wordlist files and bloom filter rebuild on save.

### Validation System

The validation system (`src/tui/settings/validation.rs`) provides real-time feedback:

1. **Type checking**: Ensures integers, floats, colors are valid
2. **Range checking**: Min/max values (e.g., timeout > 0, rarity 0.0-1.0)
3. **Inline errors**: Validation errors display below the field in red

**Example validation errors:**
- "Must be a positive integer"
- "Must be between 0.0 and 1.0"
- "Invalid color format (use red, blue, #RRGGBB, etc.)"

### Change Tracking

- **Modified fields** are highlighted with `*` indicator
- **Unsaved changes** show a warning message in the header
- **Save action** (`Ctrl+S`) writes changes to `Config` and returns to previous state
- **Cancel action** (`Esc`) discards changes

### Adding a New Settings Field

1. **Add field to `SettingsModel`** (`src/tui/settings/model.rs`):
   ```rust
   pub struct SettingsModel {
       pub my_new_field: SettingField,
       // ...
   }
   ```

2. **Initialize field in `from_config()`**:
   ```rust
   my_new_field: SettingField {
       id: "my_new_field".to_string(),
       label: "My New Field".to_string(),
       field_type: FieldType::Integer,
       value: config.my_new_field.to_string(),
       original_value: config.my_new_field.to_string(),
       description: Some("Description here".to_string()),
   },
   ```

3. **Add field to appropriate section** in `build_sections()`.

4. **Add validation logic** in `validation.rs` if needed.

5. **Add save logic** in `App::save_settings()` (`src/tui/app.rs`).

### Testing Settings

```bash
cargo test settings  # Run all settings-related tests
cargo test --lib settings_panel  # Test settings panel widget
cargo test --lib list_editor  # Test list editor widget
cargo test --lib wordlist_manager  # Test wordlist manager widget
```

### Settings State Transitions

```
Any State (except HumanConfirmation)
    ↓ (Ctrl+S)
Settings
    ↓ (Enter on StringList field)
ListEditor
    ↓ (Esc)
Settings
    ↓ (Enter on wordlist_manager)
WordlistManager
    ↓ (Esc or Done)
Settings
    ↓ (Ctrl+S or Esc)
Previous State
```

## Lessons Learned & Common Pitfalls

### Parallel Processing with Human Checker

The A* search runs decoders in parallel via Rayon. When the human checker is enabled, multiple decoders may trigger human confirmation prompts simultaneously. Key considerations:

1. **Race conditions**: Multiple decoders can find "valid" plaintext at the same time. The human might confirm one result while another decoder's result wins the race.

2. **Solution**: Store the human-confirmed text (`get_human_confirmed_text()` in `src/checkers/human_checker.rs`) and filter result nodes in A* to only accept results matching what the human confirmed.

3. **Result matching**: Compare normalized versions (lowercase, no punctuation) since the human sees cleaned-up text.

### Don't Hardcode Decoder Priority

**Anti-pattern**: Running "encoder-tagged" decoders first, then ciphers in a separate pass.

**Correct approach**: Use the **batched decoder exploration** system with A*'s heuristic (`rank_decoders()`) to naturally prioritize decoders. The ranking system already makes:
- Encoders cheap (0.7 base, 0.2 for repeated same-encoder)
- Ciphers expensive (2.0+, escalating for multiple ciphers)

The depth penalty ensures ciphers eventually get tried at shallow depths before exploring too deep with encoders. See "Batched Decoder Exploration" section above.

### Node Expansion and Continuation Nodes

The A* algorithm uses batched exploration where only top-ranked decoders are tried per expansion. Key points:

In `create_node_from_result()`, setting `next_decoder: Some(decoder_name)` limits the next expansion to only that decoder. This breaks cipher detection because:

1. Base64 decodes `dXJ5eWIgamJleXE=` → `uryyb jbeyq`
2. If `next_decoder: Some("Base64")`, only Base64 runs on `uryyb jbeyq`
3. Caesar (which could decode ROT13) never gets tried!

**Fix**: Set `next_decoder: None` to allow all decoders at each node.

### Decoder `success` Field Semantics

A decoder's `CrackResult.success` should ONLY be `true` when:
1. The decoder produced output, AND
2. A checker confirmed the output is valid plaintext

**Anti-pattern** (caused bugs):
```rust
// BAD: Setting success just because we got output
if !decoded_strings.is_empty() {
    results.success = true;
}
```

**Correct pattern**:
```rust
// GOOD: Only set success if checker confirmed
if inner_decoder_result.success {
    results.success = true;
    // ... collect the confirmed text
}
```

### UTF-8 String Safety

When creating string previews for logging/debugging, don't slice by byte index:

```rust
// BAD: Panics on multi-byte UTF-8 characters
let preview = &text[..20];

// GOOD: Safe for any UTF-8 string
let preview = text.chars().take(20).collect::<String>();
```

### Testing Human Checker Flows

When testing human checker integration:
- Use `printf 'n\ny\n'` to simulate saying "no" then "yes" to prompts
- The order of prompts is non-deterministic due to parallel processing
- Test should verify the FINAL result matches expected plaintext, not prompt order

## Bloom Filter Integration

### Overview

The wordlist checker uses a two-tier lookup system for fast dictionary matching:

1. **Bloom Filter** (fast, ~1% false positive rate) - stored at `~/.ciphey/wordlist_bloom.dat`
2. **SQLite Database** (accurate) - verifies bloom filter positives
3. **Config Fallback** - `config.wordlist` HashSet for backward compatibility

### Architecture

```
Input Word
    ↓
Bloom Filter Check (O(1), ~1% false positives)
    ↓
┌───────────────────┬─────────────────────┐
│ "Definitely not"  │ "Maybe present"     │
│ → Fast rejection  │ → SQLite query      │
│                   │ → Confirm/reject    │
└───────────────────┴─────────────────────┘
    ↓ (if no DB)
Fallback to config.wordlist HashSet
```

### Key Files

| File | Purpose |
|------|---------|
| `src/storage/bloom.rs` | Bloom filter build/save/load functions |
| `src/storage/database.rs` | Wordlist table CRUD operations |
| `src/checkers/wordlist.rs` | Two-tier lookup implementation |
| `src/cli/first_run.rs` | Wordlist import during setup |

### Bloom Filter Functions

| Function | Purpose |
|----------|---------|
| `build_bloom_filter_from_db()` | Create bloom filter from all DB words |
| `save_bloom_filter(&Bloom)` | Serialize to `~/.ciphey/wordlist_bloom.dat` |
| `load_bloom_filter() -> Option<Bloom>` | Deserialize from disk |
| `bloom_filter_exists() -> bool` | Check if cached filter exists |
| `delete_bloom_filter()` | Remove cached filter |

### Important: Bloom Filter Rebuild

The bloom filter is NOT automatically rebuilt when the database changes. After modifying the wordlist database (insert/delete), you must manually rebuild:

```rust
use crate::storage::bloom::{build_bloom_filter_from_db, save_bloom_filter};

let bloom = build_bloom_filter_from_db();
save_bloom_filter(&bloom);
```

**Future work**: Auto-rebuild bloom filter on wordlist DB modifications.

### Why Two Tiers?

- **Bloom filters** have false positives but NO false negatives
- A "definitely not in set" response is 100% reliable → fast rejection
- A "maybe in set" response requires DB verification → accurate final answer
- This gives O(1) rejection for most non-words while maintaining accuracy

## TUI Widget Design & Best Practices

### Widget Architecture

The TUI uses a modular widget system in `src/tui/widgets/`:

| Widget | File | Purpose |
|--------|------|---------|
| **PathViewer** | `path_viewer.rs` | Displays decoder chain as horizontal boxes with arrows |
| **StepDetails** | `step_details.rs` | Shows detailed info about selected decoder step |
| **TextPanel** | `text_panel.rs` | Reusable bordered text display with scrolling |

### Selection Visualization Best Practices

When highlighting selected items in the TUI, use **multiple visual cues** for maximum clarity:

#### Recommended Approach (PathViewer Implementation)
```rust
// For selected decoder box in path viewer:
let text_style = if is_selected {
    colors
        .accent                          // 1. Use accent color (from user's color scheme)
        .add_modifier(Modifier::BOLD)    // 2. Bold text
        .add_modifier(Modifier::REVERSED) // 3. Reversed background (colored fill)
} else {
    colors.text
};

let border_type = if is_selected {
    symbols::border::DOUBLE  // 4. Double border (thicker)
} else {
    symbols::border::PLAIN
};
```

**Why multiple cues?**
- Some users may have limited color vision
- Different terminal emulators render colors differently
- Multiple cues ensure selection is **always obvious**

#### Anti-pattern: Single Visual Cue
```rust
// BAD: Only using border style (hard to see!)
let border_type = if is_selected {
    symbols::border::DOUBLE
} else {
    symbols::border::PLAIN
};
```

### Metadata Display in Widgets

When showing text transformations (input/output), provide **rich metadata** to help users understand what's happening:

#### Text Metadata Helper Function
```rust
/// Calculates metadata for a text string.
fn calculate_text_metadata(text: &str) -> String {
    let char_count = text.chars().count();
    let byte_size = text.len();
    let line_count = text.lines().count().max(1);
    
    let printable_count = text
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t' || *c == '\r')
        .count();
    
    let printable_pct = if char_count > 0 {
        (printable_count * 100) / char_count
    } else {
        100
    };
    
    format!(
        "{} chars, {} bytes, {} line{}, {}% printable",
        char_count,
        byte_size,
        line_count,
        if line_count == 1 { "" } else { "s" },
        printable_pct
    )
}
```

**Why this metadata matters:**
- **Character count** - Shows text length (Unicode-aware)
- **Byte size** - Important for UTF-8 encoded text (can differ from char count)
- **Line count** - Indicates multiline text
- **Printable percentage** - Helps detect binary/control characters

#### Usage Example (StepDetails Widget)
```rust
// Display with inline metadata
let input_metadata = calculate_text_metadata(&result.encrypted_text);
let input_label = format!("Input to this step ({})", input_metadata);

Line::from(vec![Span::styled(
    input_label,
    colors.label.add_modifier(Modifier::BOLD),
)]),
Line::from(vec![
    Span::styled("  ", colors.text), // Indent the actual text
    Span::styled(before_text, colors.text_before),
]),
```

**Result:**
```
Input to this step (42 chars, 42 bytes, 1 line, 100% printable)
  SGVsbG8gV29ybGQ=

Output from this step (11 chars, 11 bytes, 1 line, 100% printable)
  hello world
```

### Widget Styling with User Color Scheme

Always use colors from `TuiColors` (derived from user's config) rather than hardcoded colors:

```rust
// GOOD: Uses user's color scheme
let text_style = colors.text_before;
let label_style = colors.label.add_modifier(Modifier::BOLD);

// BAD: Hardcoded colors (ignores user preferences)
let text_style = Style::default().fg(Color::Yellow);
```

**Available color styles:**
- `colors.accent` - Primary accent (from config's "informational")
- `colors.success` - Success messages (from config's "success")
- `colors.error` - Error messages (from config's "error")
- `colors.text` - Normal text
- `colors.muted` - Dimmed/secondary text
- `colors.highlight` - Highlighted items (accent + bold)
- `colors.label` - Field labels (cyan)
- `colors.text_before` - Input text (yellow)
- `colors.text_after` - Output text (success color)

### String Borrowing in Widget Rendering

When building widget content, **avoid temporary string borrows** that get dropped before rendering:

```rust
// BAD: Temporary format!() result gets dropped
let lines = vec![
    Line::from(vec![Span::styled(
        &format!("Label: {}", value),  // ❌ Temporary dropped!
        colors.label,
    )]),
];

// GOOD: Store formatted string in a variable
let label_text = format!("Label: {}", value);
let lines = vec![
    Line::from(vec![Span::styled(
        label_text,  // ✅ Owned string lives long enough
        colors.label,
    )]),
];
```

**Error you'll see:**
```
error[E0716]: temporary value dropped while borrowed
```

### Widget Testing

Always add unit tests for widget helper functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_text_metadata_simple() {
        let result = calculate_text_metadata("hello");
        assert!(result.contains("5 chars"));
        assert!(result.contains("5 bytes"));
        assert!(result.contains("1 line"));
        assert!(result.contains("100% printable"));
    }
    
    #[test]
    fn test_calculate_text_metadata_unicode() {
        let result = calculate_text_metadata("hello 世界");
        assert!(result.contains("8 chars"));  // 5 + space + 2 CJK
        assert!(result.contains("12 bytes")); // ASCII + UTF-8 encoded CJK
    }
}
```

### TUI Widget Modification Checklist

When modifying TUI widgets:

1. ✅ **Test with different terminal sizes** - Use small and large terminals
2. ✅ **Use user's color scheme** - Never hardcode colors
3. ✅ **Add multiple selection cues** - Color + bold + reversed + border
4. ✅ **Handle edge cases** - Empty text, very long text, special characters
5. ✅ **Add unit tests** - Test helper functions and calculations
6. ✅ **Document behavior** - Add doc comments explaining what the widget does
7. ✅ **Run `cargo test --lib <widget_name>`** - Verify tests pass
8. ✅ **Check string borrowing** - Ensure no temporary values get dropped

### TUI Performance Considerations

- **Avoid expensive calculations in render loops** - Cache metadata when possible
- **Truncate long text** - Use `MAX_TEXT_LENGTH` constants (e.g., 200 chars)
- **Use `chars().take(n)` not `[..n]`** - Safe for UTF-8 strings
- **Wrap text properly** - Use `Wrap { trim: false }` to preserve formatting

