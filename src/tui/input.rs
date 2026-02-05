//! Keyboard input handling for Ciphey's TUI.
//!
//! This module processes keyboard events and translates them into application
//! actions based on the current application state.

use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppState, WordlistManagerFocus};
use crate::config::Config;

/// Actions that may need to be performed outside the input handler.
///
/// Some operations like clipboard access may require special handling
/// in the main event loop, so we return an action to indicate what
/// needs to happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Copy the given string to the system clipboard.
    CopyToClipboard(String),
    /// Rerun Ciphey with the given text as the new input.
    /// This is used when the user wants to continue decoding from a selected step.
    RerunFromSelected(String),
    /// Submit text from the Home screen to start decoding.
    SubmitHomeInput(String),
    /// Open settings panel.
    OpenSettings,
    /// Save settings and return to previous state.
    SaveSettings,
    /// Show results from a successful history entry.
    /// Contains (encoded_text, decoded_text, path as JSON strings).
    ShowHistoryResult {
        /// The original encoded text.
        encoded_text: String,
        /// The decoded plaintext.
        decoded_text: String,
        /// The decoder path as JSON strings.
        path: Vec<String>,
    },
    /// No action required.
    None,
}

/// Handles a keyboard event and updates the application state accordingly.
///
/// This function processes key events based on the current `AppState`:
///
/// - **All states**: `?` toggles help overlay, `Ctrl+C` quits
/// - **Home**: Text input for ciphertext, `Enter` submits, `Ctrl+Enter` inserts newline, `Ctrl+S` opens settings
/// - **Loading**: `q` or `Esc` quits, `Ctrl+S` opens settings
/// - **HumanConfirmation**: `Y`/`y`/`Enter` accepts, `N`/`n`/`Escape` rejects (`q` does NOT quit)
/// - **Results**: Navigation with arrow keys/vim bindings, `c` copies selected step, `Enter` reruns from selected, `q`/`Esc` quits
/// - **Failure**: `q` or `Esc` quits
/// - **Settings**: Navigate sections/fields, edit values, save/cancel
/// - **ListEditor**: Add/remove items, navigate items
/// - **WordlistManager**: Toggle wordlists, add paths
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Returns
///
/// An `Action` indicating if any follow-up operation is needed (e.g., clipboard copy, rerun).
///
/// # Examples
///
/// ```ignore
/// let action = handle_key_event(&mut app, key_event);
/// match action {
///     Action::CopyToClipboard(text) => copy_to_clipboard(&text)?,
///     Action::RerunFromSelected(text) => rerun_ciphey(&text),
///     Action::None => {}
/// }
/// ```
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Action {
    // Check if we're in a state that has its own key handling
    let in_home = matches!(app.state, AppState::Home { .. });
    let in_confirmation = matches!(app.state, AppState::HumanConfirmation { .. });
    let in_settings = app.is_in_settings();

    // Handle Ctrl+C to quit in all states (except settings editing mode and home input)
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        if !in_settings && !in_home {
            app.should_quit = true;
            return Action::None;
        }
    }

    // Handle special states first (they have their own key handling)
    match &app.state {
        AppState::Home { .. } => {
            return handle_home_keys(app, key);
        }
        AppState::Settings { editing_mode, .. } => {
            return handle_settings_keys(app, key, *editing_mode);
        }
        AppState::ListEditor { .. } => {
            return handle_list_editor_keys(app, key);
        }
        AppState::WordlistManager { focus, .. } => {
            return handle_wordlist_manager_keys(app, key, focus.clone());
        }
        AppState::ThemePicker { .. } => {
            return handle_theme_picker_keys(app, key);
        }
        AppState::SaveConfirmation { .. } => {
            return handle_save_confirmation_keys(app, key);
        }
        AppState::ToggleListEditor { .. } => {
            return handle_toggle_list_editor_keys(app, key);
        }
        _ => {}
    }

    // Handle global key bindings for non-settings states
    match key.code {
        KeyCode::Char('q') => {
            // q should NOT quit during confirmation or home (where it's text input)
            if !in_confirmation && !in_home {
                app.should_quit = true;
                return Action::None;
            }
        }
        KeyCode::Esc => {
            // In confirmation state, Escape means reject
            if in_confirmation {
                app.respond_to_confirmation(false);
                return Action::None;
            }
            app.should_quit = true;
            return Action::None;
        }
        KeyCode::Char('?') => {
            if !in_confirmation && !in_home {
                app.show_help = !app.show_help;
            }
            return Action::None;
        }
        // Ctrl+S opens settings (except during confirmation and home - home handles it separately)
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if !in_confirmation && !in_home {
                return Action::OpenSettings;
            }
        }
        _ => {}
    }

    // Handle state-specific key bindings
    match &app.state {
        AppState::Loading { .. } => {
            // Only quit, help, and settings work in loading state
            Action::None
        }
        AppState::HumanConfirmation { .. } => {
            // Handle confirmation keys: Y/y/Enter to accept, N/n to reject
            // Escape is handled in the global bindings above
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    app.respond_to_confirmation(true);
                    Action::None
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.respond_to_confirmation(false);
                    Action::None
                }
                _ => Action::None,
            }
        }
        AppState::Results {
            result,
            selected_step,
            ..
        } => {
            let path_len = result.path.len();
            let selected = *selected_step;
            // Get the selected step's unencrypted text for copy/rerun operations
            let selected_step_text = result
                .path
                .get(selected)
                .and_then(|step| step.unencrypted_text.as_ref())
                .and_then(|texts| texts.first().cloned());
            handle_results_keys(app, key, selected_step_text, path_len)
        }
        AppState::Failure { .. } => {
            // b/Backspace returns to home, otherwise nothing else works
            match key.code {
                KeyCode::Char('b') | KeyCode::Backspace => {
                    app.return_to_home();
                    Action::None
                }
                _ => Action::None,
            }
        }
        // Settings states are handled above
        _ => Action::None,
    }
}

/// Handles key events specific to the Results state.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
/// * `selected_step_text` - The output text from the currently selected step (if any)
/// * `path_len` - Length of the decoder path
///
/// # Returns
///
/// An `Action` if clipboard copy or rerun was requested, otherwise `Action::None`.
fn handle_results_keys(
    app: &mut App,
    key: KeyEvent,
    selected_step_text: Option<String>,
    path_len: usize,
) -> Action {
    match key.code {
        // Return to home screen
        KeyCode::Char('b') => {
            app.return_to_home();
            Action::None
        }
        // Navigation: previous step
        KeyCode::Left | KeyCode::Char('h') => {
            app.prev_step();
            Action::None
        }
        // Navigation: next step
        KeyCode::Right | KeyCode::Char('l') => {
            app.next_step();
            Action::None
        }
        // Copy selected step's output to clipboard (vim-style 'y' for yank)
        KeyCode::Char('c') | KeyCode::Char('y') => {
            if let Some(text) = selected_step_text {
                Action::CopyToClipboard(text)
            } else {
                Action::None
            }
        }
        // Rerun Ciphey from the selected step's output
        KeyCode::Enter => {
            if let Some(text) = selected_step_text {
                Action::RerunFromSelected(text)
            } else {
                Action::None
            }
        }
        // Go to first step
        KeyCode::Home => {
            if let AppState::Results { selected_step, .. } = &mut app.state {
                *selected_step = 0;
            }
            Action::None
        }
        // Go to last step
        KeyCode::End => {
            if let AppState::Results { selected_step, .. } = &mut app.state {
                if path_len > 0 {
                    *selected_step = path_len - 1;
                }
            }
            Action::None
        }
        _ => Action::None,
    }
}

/// Handles key events in the Home state.
///
/// The Home state allows users to input ciphertext:
/// - Regular characters are inserted at the cursor position
/// - Enter submits the text for decoding (or selects history entry)
/// - Ctrl+Enter inserts a newline
/// - Arrow keys move the cursor (or navigate history when history is focused)
/// - Backspace/Delete remove characters
/// - Ctrl+S opens settings
/// - Tab cycles between history panel and input
/// - Esc/q quits (or deselects history)
fn handle_home_keys(app: &mut App, key: KeyEvent) -> Action {
    if let AppState::Home {
        text_input,
        history,
        selected_history,
        history_scroll_offset,
    } = &mut app.state
    {
        // Check if history is focused (selected_history is Some)
        let history_focused = selected_history.is_some();

        match key.code {
            // Escape: deselect history if focused, otherwise quit
            KeyCode::Esc => {
                if history_focused {
                    *selected_history = None;
                    Action::None
                } else {
                    app.should_quit = true;
                    Action::None
                }
            }
            // Tab: cycle between input and history
            KeyCode::Tab => {
                if history.is_empty() {
                    // No history, stay on input
                    Action::None
                } else if history_focused {
                    // Switch to input
                    *selected_history = None;
                    Action::None
                } else {
                    // Switch to history
                    *selected_history = Some(0);
                    *history_scroll_offset = 0;
                    Action::None
                }
            }
            // Enter: submit text or select history entry
            KeyCode::Enter => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Ctrl+Enter inserts newline (only when input focused)
                    if !history_focused {
                        text_input.insert_newline();
                    }
                    Action::None
                } else if history_focused {
                    // Select history entry
                    if let Some(idx) = *selected_history {
                        if let Some(entry) = history.get(idx) {
                            if entry.successful {
                                // Successful entry: show results
                                return Action::ShowHistoryResult {
                                    encoded_text: entry.encoded_text_full.clone(),
                                    decoded_text: entry.decoded_text.clone(),
                                    path: entry.path.clone(),
                                };
                            } else {
                                // Failed entry: populate input and focus it
                                text_input.clear();
                                for c in entry.encoded_text_full.chars() {
                                    text_input.insert_char(c);
                                }
                                *selected_history = None;
                                return Action::None;
                            }
                        }
                    }
                    Action::None
                } else {
                    // Regular Enter submits
                    let text = text_input.get_text();
                    if text.trim().is_empty() {
                        // Show error if empty
                        app.set_status("Please enter some ciphertext first.".to_string());
                        Action::None
                    } else {
                        Action::SubmitHomeInput(text)
                    }
                }
            }
            // Ctrl+S opens settings
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Action::OpenSettings
            }
            // Navigation
            KeyCode::Left => {
                if history_focused {
                    // When in history, Left does nothing (or could switch to input)
                    Action::None
                } else if text_input.is_cursor_at_start() && !history.is_empty() {
                    // At start of input with history available, switch to history
                    *selected_history = Some(0);
                    *history_scroll_offset = 0;
                    Action::None
                } else {
                    text_input.move_cursor_left();
                    Action::None
                }
            }
            KeyCode::Right => {
                if history_focused {
                    // When in history, Right switches to input
                    *selected_history = None;
                    Action::None
                } else {
                    text_input.move_cursor_right();
                    Action::None
                }
            }
            KeyCode::Up => {
                if history_focused {
                    // Navigate history up
                    if let Some(idx) = selected_history {
                        if *idx > 0 {
                            *idx -= 1;
                            // Adjust scroll if needed
                            if *idx < *history_scroll_offset {
                                *history_scroll_offset = *idx;
                            }
                        }
                    }
                } else {
                    text_input.move_cursor_up();
                }
                Action::None
            }
            KeyCode::Down => {
                if history_focused {
                    // Navigate history down
                    if let Some(idx) = selected_history {
                        if *idx < history.len().saturating_sub(1) {
                            *idx += 1;
                            // Note: scroll adjustment happens in render
                        }
                    }
                } else {
                    text_input.move_cursor_down();
                }
                Action::None
            }
            // Vim-style navigation for history
            KeyCode::Char('j') if history_focused => {
                if let Some(idx) = selected_history {
                    if *idx < history.len().saturating_sub(1) {
                        *idx += 1;
                    }
                }
                Action::None
            }
            KeyCode::Char('k') if history_focused => {
                if let Some(idx) = selected_history {
                    if *idx > 0 {
                        *idx -= 1;
                        if *idx < *history_scroll_offset {
                            *history_scroll_offset = *idx;
                        }
                    }
                }
                Action::None
            }
            KeyCode::Home => {
                if history_focused {
                    *selected_history = Some(0);
                    *history_scroll_offset = 0;
                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                    text_input.move_cursor_to_start();
                } else {
                    text_input.move_cursor_home();
                }
                Action::None
            }
            KeyCode::End => {
                if history_focused {
                    if !history.is_empty() {
                        *selected_history = Some(history.len() - 1);
                    }
                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                    text_input.move_cursor_to_end();
                } else {
                    text_input.move_cursor_end();
                }
                Action::None
            }
            // Deletion (only when input focused)
            KeyCode::Backspace => {
                if !history_focused {
                    text_input.backspace();
                }
                Action::None
            }
            KeyCode::Delete => {
                if !history_focused {
                    text_input.delete();
                }
                Action::None
            }
            // Character input (only when input focused)
            KeyCode::Char(c) => {
                if !history_focused {
                    // Don't insert if Ctrl is held (except for Ctrl+Enter handled above)
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::ALT)
                    {
                        text_input.insert_char(c);
                    }
                }
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

/// Handles key events in the Settings state.
fn handle_settings_keys(app: &mut App, key: KeyEvent, editing_mode: bool) -> Action {
    if editing_mode {
        // In editing mode, handle text input
        match key.code {
            KeyCode::Esc => {
                app.cancel_field_edit();
                Action::None
            }
            KeyCode::Enter => {
                app.confirm_field_edit();
                Action::None
            }
            KeyCode::Backspace => {
                app.input_backspace();
                Action::None
            }
            KeyCode::Char(c) => {
                app.input_char(c);
                Action::None
            }
            _ => Action::None,
        }
    } else {
        // Not in editing mode, handle navigation
        match key.code {
            KeyCode::Esc => {
                // Always show save confirmation modal (user requested this behavior)
                app.show_save_confirmation();
                Action::None
            }
            // Ctrl+S saves settings
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if app.settings_have_changes() {
                    Action::SaveSettings
                } else {
                    app.close_settings();
                    Action::None
                }
            }
            // Tab cycles through sections
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    app.prev_settings_section();
                } else {
                    app.next_settings_section();
                }
                Action::None
            }
            // Arrow keys navigate fields
            KeyCode::Up | KeyCode::Char('k') => {
                app.prev_settings_field();
                Action::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.next_settings_field();
                Action::None
            }
            // Left/Right also switch sections
            KeyCode::Left | KeyCode::Char('h') => {
                app.prev_settings_section();
                Action::None
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.next_settings_section();
                Action::None
            }
            // Enter edits the current field
            KeyCode::Enter => {
                app.edit_current_field();
                Action::None
            }
            // Space toggles boolean fields
            KeyCode::Char(' ') => {
                app.edit_current_field(); // For booleans, this toggles; for others, it enters edit mode
                Action::None
            }
            _ => Action::None,
        }
    }
}

/// Handles key events in the ListEditor state.
fn handle_list_editor_keys(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => {
            app.finish_list_editor();
            Action::None
        }
        KeyCode::Enter => {
            app.list_editor_add_item();
            Action::None
        }
        KeyCode::Backspace => {
            // Check if input buffer is empty - if so, delete selected item
            if let AppState::ListEditor { text_input, .. } = &app.state {
                if text_input.is_empty() {
                    app.list_editor_remove_item();
                } else {
                    app.input_backspace();
                }
            }
            Action::None
        }
        KeyCode::Delete => {
            app.list_editor_remove_item();
            Action::None
        }
        KeyCode::Up | KeyCode::Char('k') if key.modifiers.is_empty() => {
            app.list_editor_prev_item();
            Action::None
        }
        KeyCode::Down | KeyCode::Char('j') if key.modifiers.is_empty() => {
            app.list_editor_next_item();
            Action::None
        }
        KeyCode::Char(c) => {
            app.input_char(c);
            Action::None
        }
        _ => Action::None,
    }
}

/// Handles key events in the WordlistManager state.
fn handle_wordlist_manager_keys(
    app: &mut App,
    key: KeyEvent,
    focus: WordlistManagerFocus,
) -> Action {
    use crate::storage::bloom::{build_bloom_filter_from_db, save_bloom_filter};
    use crate::storage::database::{delete_wordlist_file, import_wordlist_from_file};

    match focus {
        WordlistManagerFocus::Table => {
            match key.code {
                KeyCode::Esc => {
                    app.cancel_wordlist_manager();
                    Action::None
                }
                KeyCode::Tab => {
                    app.wordlist_manager_next_focus();
                    Action::None
                }
                KeyCode::Enter => {
                    app.finish_wordlist_manager();
                    Action::None
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    // Navigate up in table
                    if let AppState::WordlistManager {
                        selected_row,
                        wordlist_files,
                        ..
                    } = &mut app.state
                    {
                        if !wordlist_files.is_empty() {
                            *selected_row = if *selected_row == 0 {
                                wordlist_files.len() - 1
                            } else {
                                *selected_row - 1
                            };
                        }
                    }
                    Action::None
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    // Navigate down in table
                    if let AppState::WordlistManager {
                        selected_row,
                        wordlist_files,
                        ..
                    } = &mut app.state
                    {
                        if !wordlist_files.is_empty() {
                            *selected_row = (*selected_row + 1) % wordlist_files.len();
                        }
                    }
                    Action::None
                }
                KeyCode::Char(' ') => {
                    // Toggle selected wordlist
                    if let AppState::WordlistManager {
                        selected_row,
                        wordlist_files,
                        pending_changes,
                        ..
                    } = &mut app.state
                    {
                        if let Some(wl) = wordlist_files.get_mut(*selected_row) {
                            wl.enabled = !wl.enabled;
                            pending_changes.insert(wl.id, wl.enabled);
                        }
                    }
                    Action::None
                }
                KeyCode::Delete => {
                    // Remove selected wordlist from database
                    if let AppState::WordlistManager {
                        selected_row,
                        wordlist_files,
                        ..
                    } = &mut app.state
                    {
                        if let Some(wl) = wordlist_files.get(*selected_row) {
                            let file_id = wl.id;
                            // Delete from database (CASCADE deletes associated words)
                            if delete_wordlist_file(file_id).is_ok() {
                                wordlist_files.remove(*selected_row);
                                if *selected_row >= wordlist_files.len()
                                    && !wordlist_files.is_empty()
                                {
                                    *selected_row = wordlist_files.len() - 1;
                                }
                                // Rebuild bloom filter after deletion
                                if let Ok(bloom) = build_bloom_filter_from_db() {
                                    let _ = save_bloom_filter(&bloom);
                                }
                            }
                        }
                    }
                    Action::None
                }
                _ => Action::None,
            }
        }
        WordlistManagerFocus::AddPathInput => {
            match key.code {
                KeyCode::Esc => {
                    // Clear input and go back to table
                    if let AppState::WordlistManager {
                        focus, text_input, ..
                    } = &mut app.state
                    {
                        text_input.clear();
                        *focus = WordlistManagerFocus::Table;
                    }
                    Action::None
                }
                KeyCode::Tab => {
                    app.wordlist_manager_next_focus();
                    Action::None
                }
                KeyCode::Enter => {
                    // Import the wordlist file
                    if let AppState::WordlistManager {
                        text_input,
                        focus,
                        wordlist_files,
                        ..
                    } = &mut app.state
                    {
                        let path = text_input.get_text().to_string();
                        if !path.is_empty() {
                            // Import wordlist file from path
                            match import_wordlist_from_file(&path, "user_import", |_, _| {}) {
                                Ok(file_row) => {
                                    // Add to display list
                                    wordlist_files.push(super::app::state::WordlistFileInfo {
                                        id: file_row.id,
                                        filename: file_row.filename,
                                        file_path: file_row.file_path,
                                        source: file_row.source,
                                        word_count: file_row.word_count,
                                        enabled: file_row.enabled,
                                        added_date: file_row.added_date,
                                    });
                                    // Rebuild bloom filter after import
                                    if let Ok(bloom) = build_bloom_filter_from_db() {
                                        let _ = save_bloom_filter(&bloom);
                                    }
                                }
                                Err(_e) => {
                                    // Import failed - could set a status message here
                                    // but the App doesn't have a status field currently
                                }
                            }
                        }
                        text_input.clear();
                        *focus = WordlistManagerFocus::Table;
                    }
                    Action::None
                }
                KeyCode::Backspace => {
                    app.input_backspace();
                    Action::None
                }
                KeyCode::Char(c) => {
                    app.input_char(c);
                    Action::None
                }
                _ => Action::None,
            }
        }
        WordlistManagerFocus::DoneButton => match key.code {
            KeyCode::Esc => {
                app.cancel_wordlist_manager();
                Action::None
            }
            KeyCode::Tab => {
                app.wordlist_manager_next_focus();
                Action::None
            }
            KeyCode::Enter => {
                app.finish_wordlist_manager();
                Action::None
            }
            _ => Action::None,
        },
    }
}

/// Handles key events in the ThemePicker state.
fn handle_theme_picker_keys(app: &mut App, key: KeyEvent) -> Action {
    use crate::tui::setup_wizard::themes::THEMES;

    if let AppState::ThemePicker {
        selected_theme,
        custom_mode,
        custom_colors,
        custom_field,
        ..
    } = &mut app.state
    {
        if *custom_mode {
            // In custom color input mode
            match key.code {
                KeyCode::Esc => {
                    *custom_mode = false;
                    Action::None
                }
                KeyCode::Tab | KeyCode::Down => {
                    *custom_field = (*custom_field + 1) % 5;
                    Action::None
                }
                KeyCode::BackTab | KeyCode::Up => {
                    *custom_field = if *custom_field == 0 {
                        4
                    } else {
                        *custom_field - 1
                    };
                    Action::None
                }
                KeyCode::Enter => {
                    // Validate and apply if all fields valid
                    if let Some(scheme) = custom_colors.to_scheme() {
                        app.close_theme_picker(true, Some(scheme));
                    }
                    Action::None
                }
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() || c == ',' {
                        let field = custom_colors.get_field_mut(*custom_field);
                        field.push(c);
                    }
                    Action::None
                }
                KeyCode::Backspace => {
                    let field = custom_colors.get_field_mut(*custom_field);
                    field.pop();
                    Action::None
                }
                _ => Action::None,
            }
        } else {
            // Theme selection mode
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    *selected_theme = if *selected_theme == 0 {
                        THEMES.len() // Wrap to custom
                    } else {
                        *selected_theme - 1
                    };
                    Action::None
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    *selected_theme = if *selected_theme >= THEMES.len() {
                        0 // Wrap to first
                    } else {
                        *selected_theme + 1
                    };
                    Action::None
                }
                KeyCode::Enter => {
                    if *selected_theme == THEMES.len() {
                        // Custom option
                        *custom_mode = true;
                    } else {
                        // Apply selected theme
                        let scheme = THEMES[*selected_theme].scheme.clone();
                        app.close_theme_picker(true, Some(scheme));
                    }
                    Action::None
                }
                KeyCode::Esc => {
                    app.close_theme_picker(false, None);
                    Action::None
                }
                _ => Action::None,
            }
        }
    } else {
        Action::None
    }
}

/// Handles key events in the ToggleListEditor state.
fn handle_toggle_list_editor_keys(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        // Escape or Enter closes and saves
        KeyCode::Esc | KeyCode::Enter => {
            app.close_toggle_list_editor();
            Action::None
        }
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.toggle_list_cursor_up();
            Action::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.toggle_list_cursor_down();
            Action::None
        }
        // Space toggles the current item
        KeyCode::Char(' ') => {
            app.toggle_list_toggle_item();
            Action::None
        }
        // 'a' selects all
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.toggle_list_select_all();
            Action::None
        }
        // 'n' selects none (clears all)
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.toggle_list_select_none();
            Action::None
        }
        _ => Action::None,
    }
}

/// Handles key events in the SaveConfirmation state.
fn handle_save_confirmation_keys(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            // Save and close
            app.handle_save_confirmation(true)
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            // Discard and close
            app.handle_save_confirmation(false)
        }
        KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Esc => {
            // Cancel and return to settings
            app.cancel_save_confirmation();
            Action::None
        }
        _ => Action::None,
    }
}

/// Opens the settings panel with the given config.
///
/// This is called by the event loop when an OpenSettings action is received.
pub fn open_settings(app: &mut App, config: &Config) {
    app.open_settings(config);
}

/// Copies the given text to the system clipboard.
///
/// This function uses the `arboard` crate to access the system clipboard
/// in a cross-platform manner.
///
/// # Arguments
///
/// * `text` - The text to copy to the clipboard
///
/// # Returns
///
/// * `Ok(())` if the text was successfully copied
/// * `Err(String)` with an error message if the operation failed
///
/// # Errors
///
/// This function will return an error if:
/// - The clipboard is unavailable (e.g., no display server on Linux)
/// - The clipboard operation fails for any other reason
///
/// # Examples
///
/// ```ignore
/// match copy_to_clipboard("Hello, world!") {
///     Ok(()) => println!("Copied to clipboard!"),
///     Err(e) => eprintln!("Failed to copy: {}", e),
/// }
/// ```
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    /// Helper function to create a key event for testing.
    fn make_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    /// Helper function to create a simple key event without modifiers.
    fn make_simple_key(code: KeyCode) -> KeyEvent {
        make_key_event(code, KeyModifiers::NONE)
    }

    #[test]
    fn test_quit_with_q() {
        let mut app = App::new("test input".to_string());
        assert!(!app.should_quit);

        let action = handle_key_event(&mut app, make_simple_key(KeyCode::Char('q')));

        assert!(app.should_quit);
        assert_eq!(action, Action::None);
    }

    #[test]
    fn test_quit_with_escape() {
        let mut app = App::new("test input".to_string());
        assert!(!app.should_quit);

        let action = handle_key_event(&mut app, make_simple_key(KeyCode::Esc));

        assert!(app.should_quit);
        assert_eq!(action, Action::None);
    }

    #[test]
    fn test_quit_with_ctrl_c() {
        let mut app = App::new("test input".to_string());
        assert!(!app.should_quit);

        let action = handle_key_event(
            &mut app,
            make_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL),
        );

        assert!(app.should_quit);
        assert_eq!(action, Action::None);
    }

    #[test]
    fn test_toggle_help() {
        let mut app = App::new("test input".to_string());
        assert!(!app.show_help);

        handle_key_event(&mut app, make_simple_key(KeyCode::Char('?')));
        assert!(app.show_help);

        handle_key_event(&mut app, make_simple_key(KeyCode::Char('?')));
        assert!(!app.show_help);
    }

    #[test]
    fn test_action_enum_equality() {
        let action1 = Action::CopyToClipboard("test".to_string());
        let action2 = Action::CopyToClipboard("test".to_string());
        let action3 = Action::CopyToClipboard("different".to_string());
        let action4 = Action::None;

        assert_eq!(action1, action2);
        assert_ne!(action1, action3);
        assert_ne!(action1, action4);
        assert_eq!(Action::None, Action::None);
    }

    #[test]
    fn test_open_settings_action() {
        let mut app = App::new("test input".to_string());

        let action = handle_key_event(
            &mut app,
            make_key_event(KeyCode::Char('s'), KeyModifiers::CONTROL),
        );

        assert_eq!(action, Action::OpenSettings);
    }
}
