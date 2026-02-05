# TUI Improvements Implementation Plan

This document outlines implementation plans for identified issues in the `src/tui` module.

---

## Issue #1: SaveConfirmation Modal Blank Background

**Status**: ✅ FIXED

**Problem**: When the SaveConfirmation modal was displayed, only the modal itself was rendered, leaving a blank/stale background instead of showing the settings screen underneath.

**Solution Applied** (`src/tui/ui.rs`):
```rust
AppState::SaveConfirmation { parent_settings } => {
    // Render the settings screen in the background first
    draw_settings_screen(
        frame, area,
        &parent_settings.settings,
        parent_settings.selected_section,
        parent_settings.selected_field,
        false, "", 0,
        parent_settings.scroll_offset,
        &parent_settings.validation_errors,
        parent_settings.settings.has_changes(),
        colors,
    );
    // Then render the confirmation modal on top
    draw_save_confirmation_modal(&area, &mut frame.buffer_mut(), colors);
}
```

---

## Issue #2: Silent Wordlist Import Failures

**Status**: ✅ PARTIALLY FIXED (logging added)

**Problem**: When wordlist import failed in `handle_wordlist_manager_keys`, errors were silently swallowed with no feedback to the user.

**Solution Applied** (`src/tui/input.rs`):
```rust
Err(e) => {
    log::warn!("Failed to import wordlist from '{}': {}", path, e);
}
```

**Remaining Work**: Add visual feedback to user (see Issue #3).

---

## Issue #3: User Feedback for Wordlist Import Errors

**Status**: 🔴 NOT IMPLEMENTED

**Problem**: Even with logging, the user receives no visual feedback when wordlist import fails. The file just doesn't appear in the list, which is confusing.

**Files to Modify**:
- `src/tui/input.rs` - Add error state to WordlistManager
- `src/tui/app/state.rs` - Add error message field to WordlistManager state
- `src/tui/widgets/wordlist_manager.rs` - Render error message

**Implementation Plan**:

### Step 1: Add error message field to WordlistManager state

In `src/tui/app/state.rs`, modify the `WordlistManager` variant:

```rust
WordlistManager {
    /// List of wordlist files from database.
    wordlist_files: Vec<WordlistFileInfo>,
    /// Currently selected row.
    selected_row: usize,
    /// Scroll offset for long lists.
    scroll_offset: usize,
    /// Parent settings state snapshot to return to.
    parent_settings: Box<SettingsStateSnapshot>,
    /// Current focus (table, add button, done button).
    focus: WordlistManagerFocus,
    /// Text input component for new wordlist path.
    text_input: TextInput,
    /// Pending changes (file_id -> new_enabled_state).
    pending_changes: HashMap<i64, bool>,
    /// Error message to display (clears after a few seconds).
    error_message: Option<String>,  // NEW FIELD
}
```

### Step 2: Update state initialization

In `src/tui/app/wordlist.rs`, update `open_wordlist_manager()`:

```rust
pub fn open_wordlist_manager(&mut self) {
    // ... existing code ...
    self.state = AppState::WordlistManager {
        wordlist_files,
        selected_row: 0,
        scroll_offset: 0,
        parent_settings: Box::new(snapshot),
        focus: WordlistManagerFocus::Table,
        text_input: TextInput::new(),
        pending_changes: HashMap::new(),
        error_message: None,  // NEW
    };
}
```

### Step 3: Set error message on import failure

In `src/tui/input.rs`, in `handle_wordlist_manager_keys`:

```rust
Err(e) => {
    log::warn!("Failed to import wordlist from '{}': {}", path, e);
    // Set error message for display
    if let AppState::WordlistManager { error_message, .. } = &mut app.state {
        *error_message = Some(format!("Import failed: {}", e));
    }
}
```

### Step 4: Render error message in widget

In `src/tui/widgets/wordlist_manager.rs`, update `render_wordlist_manager`:

```rust
pub fn render_wordlist_manager(
    area: Rect,
    buf: &mut Buffer,
    wordlist_files: &[WordlistFileInfo],
    selected_row: usize,
    focus: WordlistFocus,
    path_input: &str,
    has_pending_changes: bool,
    error_message: Option<&str>,  // NEW PARAMETER
    colors: &TuiColors,
) {
    // ... existing layout code ...
    
    // Render error message if present
    if let Some(msg) = error_message {
        let error_line = Line::from(Span::styled(msg, colors.error));
        let error_para = Paragraph::new(error_line);
        // Render at bottom of area before Done button
        error_para.render(error_area, buf);
    }
}
```

### Step 5: Clear error after timeout

In `src/tui/run.rs`, add logic to clear error message after ~3 seconds (30 ticks):

```rust
// In the tick handler section
if tick_count % 30 == 0 {
    // Clear wordlist manager error message
    if let AppState::WordlistManager { error_message, .. } = &mut app.state {
        *error_message = None;
    }
}
```

**Testing**:
1. Try importing a non-existent file path
2. Try importing a file with invalid permissions
3. Verify error message appears and disappears after timeout

---

## Issue #4: Misleading Documentation - "Three-Column Layout"

**Status**: ✅ FIXED

**Problem**: The doc comment for `draw_results_screen` says "Three-column layout" but the implementation uses a full-width path panel with step details below.

**File Modified**: `src/tui/ui.rs`

**Solution Applied**: Updated the doc comment to accurately reflect the current layout:

```rust
/// Renders the results screen with path and details layout.
///
/// Layout:
/// - Top: Path panel showing decoder chain (full width)
///   - If branches exist: 55% path viewer, 45% branch list
/// - Middle: Visual separator line
/// - Bottom: Step details panel showing input/output
/// - Footer: Status bar with keybindings
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `input_text` - The original input text
/// * `result` - The successful decoding result
/// * `selected_step` - Index of the currently selected step
/// * `branch_path` - Current position in branch hierarchy
/// * `current_branches` - Branches for the currently selected step
/// * `highlighted_branch` - Index of highlighted branch (if any)
/// * `branch_scroll_offset` - Scroll offset for branch list
/// * `colors` - The color scheme to use
```

Also update the main `draw` function doc comment:

```rust
/// Main draw function that renders the TUI based on current application state.
///
/// This function is called on each frame to render the appropriate screen based
/// on the current [`AppState`]. It handles:
///
/// - [`AppState::Home`]: Two-panel homescreen (30% history, 70% input)
/// - [`AppState::Loading`]: Centered spinner with rotating quotes
/// - [`AppState::Results`]: Path viewer with step details (full-width layout)
/// - [`AppState::Failure`]: Failure message with tips
///
/// Additionally, it renders overlays such as the help popup and status messages.
```

---

## Issue #6: Hardcoded Status Message Timeout

**Status**: ✅ FIXED

**Problem**: Status messages are cleared after 30 ticks (3 seconds), which is hardcoded. This should be configurable with a reasonable default.

**Files Modified**:
- `src/config/mod.rs` - Added `status_message_timeout` field to Config
- `src/tui/run.rs` - Uses config value instead of hardcoded constant
- `src/tui/settings/model.rs` - Added to settings panel for runtime editing

**Implementation Plan**:

### Step 1: Add configuration field

In `src/config/mod.rs`, add to the `Config` struct:

```rust
/// How long status messages display before auto-clearing (in seconds).
/// Default: 10 seconds.
#[serde(default = "default_status_message_timeout")]
pub status_message_timeout: u64,

fn default_status_message_timeout() -> u64 {
    10
}
```

### Step 2: Update run.rs to use config

In `src/tui/run.rs`, replace the hardcoded 30 ticks:

```rust
/// Tick rate for UI updates (in milliseconds).
const TICK_RATE_MS: u64 = 100;

// In run_event_loop, calculate the tick count for clearing:
let status_clear_ticks = (config.status_message_timeout * 1000 / TICK_RATE_MS) as usize;

// In the tick handler:
if tick_count % status_clear_ticks == 0 && status_clear_ticks > 0 {
    app.clear_status();
}
```

### Step 3: Add to settings panel

In `src/tui/settings/model.rs`, add the field to the General section:

```rust
SettingField {
    id: "status_message_timeout".to_string(),
    label: "Status Message Timeout".to_string(),
    field_type: FieldType::Integer,
    value: config.status_message_timeout.to_string(),
    original_value: config.status_message_timeout.to_string(),
    description: Some("Seconds before status messages auto-clear (0 = never)".to_string()),
},
```

### Step 4: Handle in settings save

In `src/tui/settings/model.rs`, `apply_to_config`:

```rust
"status_message_timeout" => {
    if let Ok(val) = field.value.parse::<u64>() {
        config.status_message_timeout = val;
    }
}
```

**Solution Applied**:
- Added `status_message_timeout: u64` field to Config struct with default value of 10 seconds
- Updated run.rs to calculate tick count based on config value (0 = never auto-clear)
- Added "Status Timeout" field to Settings > General section

---

## Issue #11: Incomplete Help Overlay Keybindings

**Status**: ✅ FIXED

**Problem**: The help overlay is missing some keybindings and doesn't update based on context (Results vs Home vs Settings).

**Files Modified**:
- `src/tui/app/state.rs` - Added `HelpContext` enum
- `src/tui/app/mod.rs` - Added `help_context()` method to App
- `src/tui/ui.rs` - Updated `draw_help_overlay` to show context-aware keybindings

**Implementation Plan**:

### Step 1: Add context enum for help

In `src/tui/app/state.rs`:

```rust
/// Context for showing appropriate help keybindings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HelpContext {
    /// Home screen keybindings.
    Home,
    /// Results screen keybindings.
    Results,
    /// Settings screen keybindings.
    Settings,
    /// Loading screen (minimal keybindings).
    Loading,
}
```

### Step 2: Add method to get help context

In `src/tui/app/mod.rs`:

```rust
impl App {
    /// Gets the appropriate help context based on current state.
    pub fn help_context(&self) -> HelpContext {
        match &self.state {
            AppState::Home { .. } => HelpContext::Home,
            AppState::Results { .. } => HelpContext::Results,
            AppState::Settings { .. } 
            | AppState::ListEditor { .. }
            | AppState::WordlistManager { .. }
            | AppState::ThemePicker { .. }
            | AppState::ToggleListEditor { .. }
            | AppState::SaveConfirmation { .. } => HelpContext::Settings,
            AppState::Loading { .. } 
            | AppState::HumanConfirmation { .. }
            | AppState::Failure { .. }
            | AppState::BranchModePrompt { .. } => HelpContext::Loading,
        }
    }
}
```

### Step 3: Update draw_help_overlay

In `src/tui/ui.rs`, refactor `draw_help_overlay`:

```rust
fn draw_help_overlay(frame: &mut Frame, area: Rect, app: &App, colors: &TuiColors) {
    let context = app.help_context();
    
    let keybindings = match context {
        HelpContext::Home => vec![
            ("Navigation", ""),
            ("Tab", "Switch between history and input"),
            ("↑ / k", "Navigate history up"),
            ("↓ / j", "Navigate history down"),
            ("← / →", "Move cursor / switch panels"),
            ("", ""),
            ("Actions", ""),
            ("Enter", "Submit input / Select history entry"),
            ("Ctrl+Enter", "Insert newline in input"),
            ("Ctrl+S", "Open settings panel"),
            ("Esc", "Quit / Deselect history"),
        ],
        HelpContext::Results => vec![
            ("Navigation", ""),
            ("← / h", "Select previous step"),
            ("→ / l", "Select next step"),
            ("↑ / k", "Select previous branch"),
            ("↓ / j", "Select next branch"),
            ("gg", "Go to first step"),
            ("G / End", "Go to last step"),
            ("Home", "Go to first step"),
            ("", ""),
            ("Actions", ""),
            ("y / c", "Yank (copy) output to clipboard"),
            ("Enter", "Select branch or create new branch"),
            ("Backspace", "Return to parent branch"),
            ("/", "Search and run specific decoder"),
            ("b", "Return to home screen"),
            ("", ""),
            ("General", ""),
            ("Ctrl+S", "Open settings panel"),
            ("?", "Toggle this help overlay"),
            ("q / Esc", "Quit the application"),
        ],
        HelpContext::Settings => vec![
            ("Navigation", ""),
            ("Tab / Shift+Tab", "Cycle through sections"),
            ("↑ / k", "Previous field"),
            ("↓ / j", "Next field"),
            ("← / h", "Previous section"),
            ("→ / l", "Next section"),
            ("", ""),
            ("Actions", ""),
            ("Enter", "Edit selected field"),
            ("Space", "Toggle boolean field"),
            ("Ctrl+S", "Save settings and close"),
            ("Esc", "Show save confirmation / Cancel edit"),
        ],
        HelpContext::Loading => vec![
            ("General", ""),
            ("Ctrl+S", "Open settings panel"),
            ("q / Esc", "Quit the application"),
        ],
    };
    
    // ... rest of rendering code ...
}
```

### Step 4: Update draw function to pass app

Change the `draw_help_overlay` call in `draw`:

```rust
if app.show_help {
    draw_help_overlay(frame, area, app, colors);
}
```

**Solution Applied**:
- Added `HelpContext` enum with variants: `Home`, `Results`, `Settings`, `Loading`
- Added `help_context()` method to App that maps AppState to HelpContext
- Updated `draw_help_overlay` to accept HelpContext and show context-specific keybindings:
  - **Home**: Tab, arrow keys, Enter, Ctrl+Enter, Ctrl+S, Esc
  - **Results**: h/l navigation, j/k branch, gg/G/Home/End, y/c copy, Enter, Backspace, /, b, Ctrl+S, ?, q/Esc
  - **Settings**: Tab, arrow keys, Enter, Space, Ctrl+S, Esc
  - **Loading**: Ctrl+S, q/Esc

---

## Summary

| Issue | Description | Status | Priority |
|-------|-------------|--------|----------|
| #1 | SaveConfirmation blank background | ✅ Fixed | - |
| #2 | Silent wordlist import (logging) | ✅ Fixed | - |
| #3 | User feedback for wordlist errors | 🔴 Pending | High |
| #4 | Misleading documentation | ✅ Fixed | - |
| #6 | Hardcoded status timeout | ✅ Fixed | - |
| #11 | Incomplete help overlay | ✅ Fixed | - |

---

## Implementation Order

Recommended order based on complexity and dependencies:

1. **Issue #4** (Documentation) - ✅ Completed - Simple text change
2. **Issue #6** (Status timeout) - ✅ Completed - Config change + settings panel
3. **Issue #11** (Help overlay) - ✅ Completed - Context-aware keybindings
4. **Issue #3** (Wordlist error feedback) - 🔴 Deferred - Lower priority

## Remaining Work

Issue #3 (User feedback for wordlist import errors) was deferred as it's lower priority compared to the other improvements. The logging is already in place, so users can check logs if imports fail. Visual feedback would be a nice enhancement but is not critical.
