//! Keyboard input handling for Ciphey's TUI.
//!
//! This module processes keyboard events and translates them into application
//! actions based on the current application state.
//!
//! ## Design Principles
//!
//! - **Input handlers return `Action`s** — they do NOT perform I/O, database
//!   access, or storage-layer operations. Side effects are executed by the
//!   event loop in `run.rs` via [`Action`] dispatch.
//! - **Each state has its own handler function** — global keybindings only
//!   apply to states that don't override them (Loading, Results).
//! - **Complex payloads use named structs** rather than inline tuple fields.

use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppState, BranchContext, WordlistManagerFocus};
use crate::config::Config;

// ============================================================================
// Action payload structs (DTOs)
// ============================================================================

/// Data for showing a cached history result in the Results screen.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowHistoryData {
    /// The database cache ID for branch linking.
    pub cache_id: i64,
    /// The original encoded text.
    pub encoded_text: String,
    /// The decoded plaintext.
    pub decoded_text: String,
    /// The decoder path as JSON strings.
    pub path: Vec<String>,
}

/// Data for requesting an AI explanation of a decoder step.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplainStepData {
    /// The name of the decoder.
    pub decoder_name: String,
    /// The input text to the decoder step.
    pub input_text: String,
    /// The output text from the decoder step.
    pub output_text: String,
    /// Optional key used by the decoder.
    pub key: Option<String>,
}

// ============================================================================
// Action enum
// ============================================================================

/// Actions that may need to be performed outside the input handler.
///
/// Input handlers produce these; the event loop in `run.rs` consumes them.
/// All I/O, database, and storage operations happen in the event loop —
/// input handlers only mutate in-memory `App` state.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Copy the given string to the system clipboard.
    CopyToClipboard(String),
    /// Rerun Ciphey with the given text as the new input.
    RerunFromSelected(String),
    /// Submit text from the Home screen to start decoding.
    SubmitHomeInput(String),
    /// Open settings panel.
    OpenSettings,
    /// Save settings and return to previous state.
    SaveSettings,
    /// Show results from a successful history entry.
    ShowHistoryResult(ShowHistoryData),
    /// Switch to the highlighted branch.
    SwitchToBranch(i64),
    /// Open the branch mode prompt for creating a new branch.
    OpenBranchPrompt,
    /// Open the decoder search modal.
    OpenDecoderSearch,
    /// Open the quick search overlay (browser search providers).
    OpenQuickSearch,
    /// Launch a quick search URL in the system browser.
    LaunchQuickSearch(String),
    /// Return to parent branch (Backspace when viewing a branch).
    ReturnToParent,
    /// Run a full A* search as a branch, with branch context for database linkage.
    RunBranchFullSearch(String, Option<BranchContext>),
    /// Run single layer decoding as a branch.
    RunBranchSingleLayer(String),
    /// Run a specific decoder as a branch.
    RunBranchDecoder(String, String),
    /// Request AI explanation for the selected decoder step.
    ExplainStep(ExplainStepData),
    /// Open the Ask AI modal for the selected step.
    OpenAskAi,
    /// Submit a question to AI about the selected step.
    SubmitAskAi(String),
    /// Close the Ask AI modal.
    CloseAskAi,
    /// Import a wordlist file from the given path (I/O handled by event loop).
    ImportWordlist(String),
    /// Delete a wordlist file by database ID (I/O handled by event loop).
    DeleteWordlist(i64),
    /// Cancel the current decode and return to Home.
    CancelDecode,
    /// No action required.
    None,
}

// ============================================================================
// Top-level dispatch
// ============================================================================

/// Handles a keyboard event and updates the application state accordingly.
///
/// ## Dispatch order
///
/// 1. **Overlays** (Ask AI, Decoder Search, Quick Search) — these float on
///    top and consume all input when active.
/// 2. **State-specific handlers** — each `AppState` variant that needs custom
///    key handling dispatches to its own function and returns immediately.
/// 3. **Global keybindings** — `Ctrl+C`, `q`, `Esc`, `?`, `Ctrl+S` apply to
///    the remaining states (Loading, Results) that don't override them.
/// 4. **Results-specific keys** — handled last for the Results state.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Returns
///
/// An `Action` indicating if any follow-up operation is needed.
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Action {
    // ── 0. Acknowledge persistent status messages on any keypress ──────
    app.acknowledge_status();

    // ── 1. Overlays (float on top of Results) ──────────────────────────
    if app.is_ask_ai_active() {
        return handle_ask_ai_keys(app, key);
    }
    if app.is_decoder_search_active() {
        return handle_decoder_search_keys(app, key);
    }
    if app.is_quick_search_active() {
        return handle_quick_search_keys(app, key);
    }

    // ── 2. State-specific handlers (early return) ──────────────────────
    match &app.state {
        AppState::Home(_) => return handle_home_keys(app, key),
        AppState::Loading(_) => return handle_loading_keys(app, key),
        AppState::HumanConfirmation(_) => return handle_confirmation_keys(app, key),
        AppState::Settings(ss) => return handle_settings_keys(app, key, ss.editing_mode),
        AppState::ListEditor(_) => return handle_list_editor_keys(app, key),
        AppState::WordlistManager(wm) => {
            return handle_wordlist_manager_keys(app, key, wm.focus.clone())
        }
        AppState::ThemePicker(_) => return handle_theme_picker_keys(app, key),
        AppState::SaveConfirmation(_) => return handle_save_confirmation_keys(app, key),
        AppState::ToggleListEditor(_) => return handle_toggle_list_editor_keys(app, key),
        AppState::BranchModePrompt(_) => return handle_branch_mode_prompt_keys(app, key),
        AppState::Failure(_) => return handle_failure_keys(app, key),
        // Results falls through to global + state-specific handling below
        AppState::Results(_) => {}
    }

    // ── 3. Global keybindings (Results only now) ───────────────────────
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return Action::None;
    }

    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            return Action::None;
        }
        KeyCode::Esc => {
            app.should_quit = true;
            return Action::None;
        }
        KeyCode::Char('?') => {
            app.show_help = !app.show_help;
            return Action::None;
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return Action::OpenSettings;
        }
        _ => {}
    }

    // ── 4. Results-specific keys ───────────────────────────────────────
    if let AppState::Results(ref rs) = app.state {
        let selected = rs.selected_step;
        let selected_step_text = rs
            .result
            .path
            .get(selected)
            .and_then(|step| step.unencrypted_text.as_ref())
            .and_then(|texts| texts.first().cloned());
        return handle_results_keys(app, key, selected_step_text);
    }

    Action::None
}

// ============================================================================
// State-specific handlers
// ============================================================================

/// Handles key events specific to the Results state.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
/// * `selected_step_text` - The output text from the currently selected step (if any)
///
/// # Returns
///
/// An `Action` if clipboard copy, rerun, branching, etc. was requested.
fn handle_results_keys(app: &mut App, key: KeyEvent, selected_step_text: Option<String>) -> Action {
    use super::app::ResultsFocus;

    let is_viewing_branch = if let AppState::Results(ref rs) = app.state {
        rs.branch_path.is_branch()
    } else {
        false
    };

    let current_focus = if let AppState::Results(ref rs) = app.state {
        rs.focus
    } else {
        ResultsFocus::TreeView
    };

    let has_branches = app.has_branches();
    let highlighted_branch_id = app.get_highlighted_branch().map(|b| b.cache_id);

    match key.code {
        KeyCode::Char('g') => {
            if current_focus == ResultsFocus::TreeView {
                if app.pending_g {
                    app.pending_g = false;
                    app.first_step();
                } else {
                    app.pending_g = true;
                }
            } else {
                app.pending_g = false;
            }
            Action::None
        }
        KeyCode::Char('b') => {
            app.pending_g = false;
            app.return_to_home();
            Action::None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.pending_g = false;
            if current_focus == ResultsFocus::TreeView {
                app.prev_step();
            }
            Action::None
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.pending_g = false;
            if current_focus == ResultsFocus::TreeView {
                app.next_step();
            }
            Action::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.pending_g = false;
            if current_focus == ResultsFocus::LevelDetail && has_branches {
                app.prev_branch();
            }
            Action::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.pending_g = false;
            if current_focus == ResultsFocus::LevelDetail && has_branches {
                app.next_branch();
            }
            Action::None
        }
        KeyCode::Home => {
            app.pending_g = false;
            if current_focus == ResultsFocus::TreeView {
                app.first_step();
            }
            Action::None
        }
        KeyCode::End | KeyCode::Char('G') => {
            app.pending_g = false;
            if current_focus == ResultsFocus::TreeView {
                app.last_step();
            }
            Action::None
        }
        KeyCode::Char('c') | KeyCode::Char('y') => {
            app.pending_g = false;
            if let Some(text) = selected_step_text {
                Action::CopyToClipboard(text)
            } else {
                Action::None
            }
        }
        KeyCode::Enter => {
            app.pending_g = false;
            match current_focus {
                ResultsFocus::LevelDetail => {
                    if let Some(cache_id) = highlighted_branch_id {
                        Action::SwitchToBranch(cache_id)
                    } else if has_branches {
                        app.next_branch();
                        Action::None
                    } else {
                        Action::None
                    }
                }
                ResultsFocus::TreeView => {
                    if selected_step_text.is_some() {
                        Action::OpenBranchPrompt
                    } else {
                        Action::None
                    }
                }
                ResultsFocus::StepDetails => Action::None,
            }
        }
        KeyCode::Backspace => {
            app.pending_g = false;
            if is_viewing_branch {
                Action::ReturnToParent
            } else {
                Action::None
            }
        }
        KeyCode::Char('e') => {
            app.pending_g = false;
            if !crate::ai::is_ai_configured() {
                app.set_status("AI not configured. Enable in Settings (Ctrl+S).".to_string());
                return Action::None;
            }
            if let AppState::Results(ref mut rs) = app.state {
                if rs.ai_loading {
                    app.set_status("AI explanation already loading...".to_string());
                    return Action::None;
                }
                if let Some(step) = rs.result.path.get(rs.selected_step) {
                    let data = ExplainStepData {
                        decoder_name: step.decoder.to_string(),
                        input_text: step.encrypted_text.clone(),
                        output_text: step
                            .unencrypted_text
                            .as_ref()
                            .and_then(|t| t.first().cloned())
                            .unwrap_or_default(),
                        key: step.key.clone(),
                    };
                    rs.ai_loading = true;
                    return Action::ExplainStep(data);
                }
            }
            Action::None
        }
        KeyCode::Char('a') => {
            app.pending_g = false;
            if !crate::ai::is_ai_configured() {
                app.set_status("AI not configured. Enable in Settings (Ctrl+S).".to_string());
                return Action::None;
            }
            Action::OpenAskAi
        }
        KeyCode::Char('/') => {
            app.pending_g = false;
            if selected_step_text.is_some() {
                Action::OpenDecoderSearch
            } else {
                Action::None
            }
        }
        KeyCode::Char('o') => {
            app.pending_g = false;
            if selected_step_text.is_some() {
                Action::OpenQuickSearch
            } else {
                Action::None
            }
        }
        KeyCode::Tab => {
            app.pending_g = false;
            app.switch_focus();
            Action::None
        }
        _ => {
            app.pending_g = false;
            Action::None
        }
    }
}

/// Handles key events in the Loading state.
///
/// - `Esc` / `b` cancels the decode and returns to Home
/// - `Ctrl+C` quits the application
/// - `Ctrl+S` opens settings
/// - `?` toggles help
/// - `q` quits the application
fn handle_loading_keys(app: &mut App, key: KeyEvent) -> Action {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return Action::None;
    }

    match key.code {
        KeyCode::Esc | KeyCode::Char('b') => Action::CancelDecode,
        KeyCode::Char('q') => {
            app.should_quit = true;
            Action::None
        }
        KeyCode::Char('?') => {
            app.show_help = !app.show_help;
            Action::None
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenSettings,
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
/// - Ctrl+Left/Right for word-level movement
/// - Backspace/Delete remove characters
/// - Ctrl+S opens settings
/// - Tab cycles between history panel and input
/// - Esc quits (or deselects history)
fn handle_home_keys(app: &mut App, key: KeyEvent) -> Action {
    if let AppState::Home(ref mut home) = app.state {
        let text_input = &mut home.text_input;
        let history = &mut home.history;
        let selected_history = &mut home.selected_history;
        let history_scroll_offset = &mut home.history_scroll_offset;
        let history_focused = selected_history.is_some();

        match key.code {
            KeyCode::Esc => {
                if history_focused {
                    *selected_history = None;
                    Action::None
                } else {
                    app.should_quit = true;
                    Action::None
                }
            }
            KeyCode::Tab => {
                if history.is_empty() {
                    Action::None
                } else if history_focused {
                    *selected_history = None;
                    Action::None
                } else {
                    *selected_history = Some(0);
                    *history_scroll_offset = 0;
                    Action::None
                }
            }
            KeyCode::Enter => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if !history_focused {
                        text_input.insert_newline();
                    }
                    Action::None
                } else if history_focused {
                    if let Some(idx) = *selected_history {
                        if let Some(entry) = history.get(idx) {
                            if entry.successful {
                                return Action::ShowHistoryResult(ShowHistoryData {
                                    cache_id: entry.id,
                                    encoded_text: entry.encoded_text_full.clone(),
                                    decoded_text: entry.decoded_text.clone(),
                                    path: entry.path.clone(),
                                });
                            } else {
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
                    let text = text_input.get_text();
                    if text.trim().is_empty() {
                        app.set_status("Please enter some ciphertext first.".to_string());
                        Action::None
                    } else {
                        Action::SubmitHomeInput(text)
                    }
                }
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Action::OpenSettings
            }
            KeyCode::Left => {
                if history_focused {
                    Action::None
                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                    text_input.move_cursor_word_left();
                    Action::None
                } else if text_input.is_cursor_at_start() && !history.is_empty() {
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
                    *selected_history = None;
                    Action::None
                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                    text_input.move_cursor_word_right();
                    Action::None
                } else {
                    text_input.move_cursor_right();
                    Action::None
                }
            }
            KeyCode::Up => {
                if history_focused {
                    if let Some(idx) = selected_history {
                        if *idx > 0 {
                            *idx -= 1;
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
                    if let Some(idx) = selected_history {
                        if *idx < history.len().saturating_sub(1) {
                            *idx += 1;
                        }
                    }
                } else {
                    text_input.move_cursor_down();
                }
                Action::None
            }
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
            KeyCode::Char(c) => {
                if !history_focused
                    && !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    text_input.insert_char(c);
                }
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

/// Handles key events in the HumanConfirmation state.
///
/// - `Y`/`y`/`Enter` accepts the plaintext candidate
/// - `N`/`n`/`Esc` rejects the plaintext candidate
/// - `q` does NOT quit (unlike other states)
fn handle_confirmation_keys(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            app.respond_to_confirmation(true);
            Action::None
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.respond_to_confirmation(false);
            Action::None
        }
        _ => Action::None,
    }
}

/// Handles key events in the Failure state.
///
/// - `b`/`Backspace` returns to home
/// - `q`/`Esc` quits
/// - `?` toggles help
/// - `Ctrl+S` opens settings
fn handle_failure_keys(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('b') | KeyCode::Backspace => {
            app.return_to_home();
            Action::None
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
            Action::None
        }
        KeyCode::Char('?') => {
            app.show_help = !app.show_help;
            Action::None
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenSettings,
        _ => Action::None,
    }
}

/// Handles key events in the Settings state.
///
/// - `Esc` closes settings without saving (discards changes).
/// - `Ctrl+S` saves and closes.
fn handle_settings_keys(app: &mut App, key: KeyEvent, editing_mode: bool) -> Action {
    if editing_mode {
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
        match key.code {
            KeyCode::Esc => {
                // Directly close settings without saving (consistent Esc = cancel)
                app.close_settings();
                Action::None
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if app.settings_have_changes() {
                    Action::SaveSettings
                } else {
                    app.close_settings();
                    Action::None
                }
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    app.prev_settings_section();
                } else {
                    app.next_settings_section();
                }
                Action::None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.prev_settings_field();
                Action::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.next_settings_field();
                Action::None
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.prev_settings_section();
                Action::None
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.next_settings_section();
                Action::None
            }
            KeyCode::Enter => {
                app.edit_current_field();
                Action::None
            }
            KeyCode::Char(' ') => {
                app.edit_current_field();
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
            if let AppState::ListEditor(ref le) = app.state {
                if le.text_input.is_empty() {
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
///
/// I/O operations (import, delete, bloom filter rebuild) are NOT performed
/// here. Instead, `Action::ImportWordlist` or `Action::DeleteWordlist` is
/// returned and the event loop in `run.rs` executes the side effects.
fn handle_wordlist_manager_keys(
    app: &mut App,
    key: KeyEvent,
    focus: WordlistManagerFocus,
) -> Action {
    match focus {
        WordlistManagerFocus::Table => match key.code {
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
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    let selected_row = &mut wm.selected_row;
                    let wordlist_files = &wm.wordlist_files;
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
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    let selected_row = &mut wm.selected_row;
                    let wordlist_files = &wm.wordlist_files;
                    if !wordlist_files.is_empty() {
                        *selected_row = (*selected_row + 1) % wordlist_files.len();
                    }
                }
                Action::None
            }
            KeyCode::Char(' ') => {
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    let selected_row = &wm.selected_row;
                    let wordlist_files = &mut wm.wordlist_files;
                    let pending_changes = &mut wm.pending_changes;
                    if let Some(wl) = wordlist_files.get_mut(*selected_row) {
                        wl.enabled = !wl.enabled;
                        pending_changes.insert(wl.id, wl.enabled);
                    }
                }
                Action::None
            }
            KeyCode::Delete => {
                // Extract the file ID, then return an Action for the event loop to handle I/O
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    let selected_row = &mut wm.selected_row;
                    let wordlist_files = &mut wm.wordlist_files;
                    if let Some(wl) = wordlist_files.get(*selected_row) {
                        let file_id = wl.id;
                        // Remove from in-memory list immediately for responsive UI
                        wordlist_files.remove(*selected_row);
                        if *selected_row >= wordlist_files.len() && !wordlist_files.is_empty() {
                            *selected_row = wordlist_files.len() - 1;
                        }
                        return Action::DeleteWordlist(file_id);
                    }
                }
                Action::None
            }
            _ => Action::None,
        },
        WordlistManagerFocus::AddPathInput => match key.code {
            KeyCode::Esc => {
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    wm.text_input.clear();
                    wm.focus = WordlistManagerFocus::Table;
                }
                Action::None
            }
            KeyCode::Tab => {
                app.wordlist_manager_next_focus();
                Action::None
            }
            KeyCode::Enter => {
                // Extract the path and return an Action for the event loop to handle I/O
                if let AppState::WordlistManager(ref mut wm) = app.state {
                    let path = wm.text_input.get_text().to_string();
                    wm.text_input.clear();
                    wm.focus = WordlistManagerFocus::Table;
                    if !path.is_empty() {
                        return Action::ImportWordlist(path);
                    }
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
        },
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

    if let AppState::ThemePicker(ref mut tp) = app.state {
        let selected_theme = &mut tp.selected_theme;
        let custom_mode = &mut tp.custom_mode;
        let custom_colors = &mut tp.custom_colors;
        let custom_field = &mut tp.custom_field;
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

/// Handles key events in the BranchModePrompt state.
fn handle_branch_mode_prompt_keys(app: &mut App, key: KeyEvent) -> Action {
    use super::app::BranchMode;

    if let AppState::BranchModePrompt(ref mut bmp) = app.state {
        let selected_mode = &mut bmp.selected_mode;
        let branch_context = &mut bmp.branch_context;
        match key.code {
            // Navigate between modes
            KeyCode::Up | KeyCode::Char('k') => {
                *selected_mode = BranchMode::FullSearch;
                Action::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                *selected_mode = BranchMode::SingleLayer;
                Action::None
            }
            // Confirm selection
            KeyCode::Enter => {
                let mode = *selected_mode;
                let context = branch_context.clone();
                // Close the modal and return an action
                app.close_branch_mode_prompt();
                match mode {
                    BranchMode::FullSearch => {
                        let text = context.text_to_decode.clone();
                        Action::RunBranchFullSearch(text, Some(context))
                    }
                    BranchMode::SingleLayer => Action::RunBranchSingleLayer(context.text_to_decode),
                }
            }
            // Cancel
            KeyCode::Esc => {
                app.close_branch_mode_prompt();
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

// ============================================================================
// Overlay handlers
// ============================================================================

/// Handles key events for the DecoderSearch overlay.
fn handle_decoder_search_keys(app: &mut App, key: KeyEvent) -> Action {
    if let Some(ref mut overlay) = app.decoder_search {
        match key.code {
            // Navigate list
            KeyCode::Up | KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if overlay.selected_index > 0 {
                    overlay.selected_index -= 1;
                }
                Action::None
            }
            KeyCode::Down | KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if !overlay.filtered_decoders.is_empty()
                    && overlay.selected_index < overlay.filtered_decoders.len() - 1
                {
                    overlay.selected_index += 1;
                }
                Action::None
            }
            // Also allow arrow keys without modifier
            KeyCode::Up => {
                if overlay.selected_index > 0 {
                    overlay.selected_index -= 1;
                }
                Action::None
            }
            KeyCode::Down => {
                if !overlay.filtered_decoders.is_empty()
                    && overlay.selected_index < overlay.filtered_decoders.len() - 1
                {
                    overlay.selected_index += 1;
                }
                Action::None
            }
            // Confirm selection - run the selected decoder
            KeyCode::Enter => {
                if let Some(decoder_name) = overlay.filtered_decoders.get(overlay.selected_index) {
                    let decoder_to_run = decoder_name.to_string();
                    let text = overlay.branch_context.text_to_decode.clone();
                    app.close_decoder_search();
                    Action::RunBranchDecoder(text, decoder_to_run)
                } else {
                    Action::None
                }
            }
            // Cancel
            KeyCode::Esc => {
                app.close_decoder_search();
                Action::None
            }
            // Text input
            KeyCode::Char(c) => {
                overlay.text_input.insert_char(c);
                // Update filtered list
                let query = overlay.text_input.get_text().to_lowercase();
                overlay.filtered_decoders = overlay
                    .all_decoders
                    .iter()
                    .filter(|name| name.to_lowercase().contains(&query))
                    .copied()
                    .collect();
                overlay.selected_index = 0;
                Action::None
            }
            KeyCode::Backspace => {
                overlay.text_input.backspace();
                // Update filtered list
                let query = overlay.text_input.get_text().to_lowercase();
                if query.is_empty() {
                    overlay.filtered_decoders = overlay.all_decoders.clone();
                } else {
                    overlay.filtered_decoders = overlay
                        .all_decoders
                        .iter()
                        .filter(|name| name.to_lowercase().contains(&query))
                        .copied()
                        .collect();
                }
                overlay.selected_index = 0;
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

/// Handles key events for the QuickSearch overlay.
fn handle_quick_search_keys(app: &mut App, key: KeyEvent) -> Action {
    if let Some(ref mut overlay) = app.quick_search {
        match key.code {
            // Navigate list
            KeyCode::Up | KeyCode::Char('k') => {
                if overlay.selected_index > 0 {
                    overlay.selected_index -= 1;
                }
                Action::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !overlay.entries.is_empty() && overlay.selected_index < overlay.entries.len() - 1
                {
                    overlay.selected_index += 1;
                }
                Action::None
            }
            // Confirm selection - build URL and open in browser
            KeyCode::Enter => {
                if let Some((_name, url_template)) = overlay.entries.get(overlay.selected_index) {
                    let url = if url_template.contains("{base64}") {
                        use base64::Engine;
                        let b64_text =
                            base64::engine::general_purpose::STANDARD.encode(&overlay.output_text);
                        url_template.replace("{base64}", &b64_text)
                    } else {
                        let encoded_text = urlencoding::encode(&overlay.output_text).to_string();
                        url_template.replace("{}", &encoded_text)
                    };
                    app.close_quick_search();
                    Action::LaunchQuickSearch(url)
                } else {
                    Action::None
                }
            }
            // Cancel
            KeyCode::Esc => {
                app.close_quick_search();
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

/// Handles key events for the Ask AI overlay.
fn handle_ask_ai_keys(app: &mut App, key: KeyEvent) -> Action {
    if let Some(ref mut overlay) = app.ask_ai {
        match key.code {
            // Escape closes the modal
            KeyCode::Esc => Action::CloseAskAi,
            // Ctrl+Enter submits the question
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let question = overlay.text_input.get_text();
                if question.trim().is_empty() || overlay.loading {
                    Action::None
                } else {
                    Action::SubmitAskAi(question)
                }
            }
            // Ctrl+C copies the response
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(ref response) = overlay.response {
                    Action::CopyToClipboard(response.clone())
                } else {
                    Action::None
                }
            }
            // Regular Enter inserts newline in question input
            KeyCode::Enter => {
                overlay.text_input.insert_newline();
                Action::None
            }
            // Text input
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    overlay.text_input.insert_char(c);
                }
                Action::None
            }
            KeyCode::Backspace => {
                overlay.text_input.backspace();
                Action::None
            }
            KeyCode::Delete => {
                overlay.text_input.delete();
                Action::None
            }
            KeyCode::Left => {
                overlay.text_input.move_cursor_left();
                Action::None
            }
            KeyCode::Right => {
                overlay.text_input.move_cursor_right();
                Action::None
            }
            KeyCode::Up => {
                if overlay.response.is_some() {
                    overlay.response_scroll = overlay.response_scroll.saturating_sub(1);
                } else {
                    overlay.text_input.move_cursor_up();
                }
                Action::None
            }
            KeyCode::Down => {
                if overlay.response.is_some() {
                    overlay.response_scroll = overlay.response_scroll.saturating_add(1);
                } else {
                    overlay.text_input.move_cursor_down();
                }
                Action::None
            }
            KeyCode::Home => {
                overlay.text_input.move_cursor_home();
                Action::None
            }
            KeyCode::End => {
                overlay.text_input.move_cursor_end();
                Action::None
            }
            _ => Action::None,
        }
    } else {
        Action::None
    }
}

// ============================================================================
// Utility functions
// ============================================================================

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
    fn test_cancel_with_escape_in_loading() {
        let mut app = App::new("test input".to_string());
        assert!(!app.should_quit);

        // In Loading state, Esc cancels the decode (returns CancelDecode)
        let action = handle_key_event(&mut app, make_simple_key(KeyCode::Esc));

        assert!(!app.should_quit); // Does NOT quit
        assert_eq!(action, Action::CancelDecode);
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
