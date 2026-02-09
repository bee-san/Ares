//! Keyboard input handling for the setup wizard.
//!
//! This module processes keyboard events and translates them into
//! state transitions and updates for the setup wizard.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{SetupApp, SetupState, WordlistFocus};
use super::themes::THEMES;
use super::ui::ai::AiConfigFocus;

/// Handles a keyboard event for the setup wizard.
///
/// # Arguments
///
/// * `app` - Mutable reference to the setup application state
/// * `key` - The keyboard event to process
pub fn handle_setup_key_event(app: &mut SetupApp, key: KeyEvent) {
    // Global quit handling
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.should_quit = true;
        return;
    }

    // Dispatch to state-specific handler
    match &mut app.state {
        SetupState::Welcome => handle_welcome_keys(app, key),
        SetupState::Tutorial => handle_tutorial_keys(app, key),
        SetupState::ThemeSelection { .. } => handle_theme_selection_keys(app, key),
        SetupState::ResultsMode { .. } => handle_results_mode_keys(app, key),
        SetupState::TimeoutConfig { .. } => handle_timeout_keys(app, key),
        SetupState::WordlistConfig { .. } => handle_wordlist_keys(app, key),
        SetupState::EnhancedDetection { .. } => handle_enhanced_detection_keys(app, key),
        SetupState::TokenInput { .. } => handle_token_input_keys(app, key),
        SetupState::Downloading { .. } => handle_downloading_keys(app, key),
        SetupState::CuteCat => handle_cute_cat_keys(app, key),
        SetupState::ShowingCat => handle_showing_cat_keys(app, key),
        SetupState::AiConfig { .. } => handle_ai_config_keys(app, key),
        SetupState::QuickSearches { .. } => handle_quick_searches_keys(app, key),
        SetupState::Complete => handle_complete_keys(app, key),
    }
}

/// Handles keys on the welcome screen.
fn handle_welcome_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') => app.next_step(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.skip_setup(),
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        _ => {}
    }
}

/// Handles keys on the tutorial screen.
fn handle_tutorial_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('n') => app.next_step(),
        KeyCode::Backspace | KeyCode::Char('p') | KeyCode::Left => app.prev_step(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.skip_setup(),
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        _ => {}
    }
}

/// Handles keys on the theme selection screen.
fn handle_theme_selection_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::ThemeSelection {
        selected,
        custom_mode,
        custom_colors,
        custom_field,
    } = &mut app.state
    {
        if *custom_mode {
            // In custom color input mode
            match key.code {
                KeyCode::Esc => {
                    *custom_mode = false;
                }
                KeyCode::Tab | KeyCode::Down => {
                    *custom_field = (*custom_field + 1) % 5;
                }
                KeyCode::BackTab | KeyCode::Up => {
                    *custom_field = if *custom_field == 0 {
                        4
                    } else {
                        *custom_field - 1
                    };
                }
                KeyCode::Enter => {
                    // Validate and proceed if all fields are valid
                    if custom_colors.to_scheme().is_some() {
                        app.next_step();
                    }
                }
                KeyCode::Char(c) => {
                    let field = custom_colors.get_field_mut(*custom_field);
                    // Only allow digits and commas
                    if c.is_ascii_digit() || c == ',' {
                        field.push(c);
                    }
                }
                KeyCode::Backspace => {
                    let field = custom_colors.get_field_mut(*custom_field);
                    field.pop();
                }
                _ => {}
            }
        } else {
            // Normal theme selection mode
            match key.code {
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                    // Navigate up with wrapping
                    if *selected == 0 {
                        *selected = THEMES.len(); // Wrap to Custom option
                    } else {
                        *selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                    // Navigate down with wrapping
                    if *selected >= THEMES.len() {
                        *selected = 0; // Wrap to first theme
                    } else {
                        *selected += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if *selected == THEMES.len() {
                        // Custom option selected
                        *custom_mode = true;
                    } else {
                        app.next_step();
                    }
                }
                KeyCode::Backspace | KeyCode::Left | KeyCode::Char('p') => {
                    app.prev_step();
                }
                KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                _ => {}
            }
        }
    }
}

/// Handles keys on the results mode screen.
fn handle_results_mode_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::ResultsMode { selected } = &mut app.state {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                *selected = if *selected == 0 { 1 } else { 0 };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                *selected = if *selected == 1 { 0 } else { 1 };
            }
            KeyCode::Char('1') => *selected = 0,
            KeyCode::Char('2') => *selected = 1,
            KeyCode::Enter | KeyCode::Char(' ') => app.next_step(),
            KeyCode::Backspace | KeyCode::Left | KeyCode::Char('p') => app.prev_step(),
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            _ => {}
        }
    }
}

/// Handles keys on the timeout configuration screen.
fn handle_timeout_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::TimeoutConfig { value, .. } = &mut app.state {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if *value < 500 {
                    *value += 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if *value > 1 {
                    *value -= 1;
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let digit = c.to_digit(10).unwrap() as u32;
                let new_value = *value * 10 + digit;
                if new_value <= 500 {
                    *value = new_value;
                }
            }
            KeyCode::Backspace => {
                *value /= 10;
                if *value == 0 {
                    *value = 1;
                }
            }
            KeyCode::Enter => app.next_step(),
            KeyCode::Left | KeyCode::Char('p') => app.prev_step(),
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            _ => {}
        }
    }
}

/// Handles keys on the wordlist configuration screen.
fn handle_wordlist_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::WordlistConfig {
        custom_paths,
        current_input,
        cursor,
        selected_predefined,
        focus,
        custom_url,
        custom_url_source,
        download_progress,
    } = &mut app.state
    {
        // Handle input during downloads
        if let Some(progress) = download_progress {
            // Check if download is complete (current == total)
            let download_complete = progress.current == progress.total;

            match key.code {
                KeyCode::Esc => {
                    // Cancel download (not implemented fully, just mark as failed)
                    progress.failed.push("Cancelled by user".to_string());
                }
                KeyCode::Enter if download_complete => {
                    // Downloads are done, user acknowledged - proceed to next step
                    app.wordlist_paths = custom_paths.clone();
                    app.selected_predefined_wordlists = selected_predefined.clone();
                    app.state = SetupState::EnhancedDetection { selected: 0 };
                }
                _ => {}
            }
            return;
        }

        match focus {
            WordlistFocus::PredefinedList {
                cursor: list_cursor,
            } => {
                // Navigating predefined wordlist checkboxes
                let predefined_wordlists = crate::storage::download::get_predefined_wordlists();
                let max_index = if predefined_wordlists.is_empty() {
                    0
                } else {
                    predefined_wordlists.len() - 1
                };

                match key.code {
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        // Navigate up with wrapping
                        if !predefined_wordlists.is_empty() {
                            if *list_cursor == 0 {
                                *list_cursor = max_index;
                            } else {
                                *list_cursor -= 1;
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        // Navigate down with wrapping
                        if !predefined_wordlists.is_empty() {
                            if *list_cursor >= max_index {
                                *list_cursor = 0;
                            } else {
                                *list_cursor += 1;
                            }
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        // Toggle selection of the currently highlighted wordlist
                        let current_idx = *list_cursor;
                        if selected_predefined.contains(&current_idx) {
                            selected_predefined.retain(|&x| x != current_idx);
                        } else {
                            selected_predefined.push(current_idx);
                        }
                    }
                    KeyCode::Char('1') => {
                        // Quick toggle first wordlist
                        if selected_predefined.contains(&0) {
                            selected_predefined.retain(|&x| x != 0);
                        } else {
                            selected_predefined.push(0);
                        }
                    }
                    KeyCode::Char('2') => {
                        // Quick toggle second wordlist
                        if predefined_wordlists.len() > 1 {
                            if selected_predefined.contains(&1) {
                                selected_predefined.retain(|&x| x != 1);
                            } else {
                                selected_predefined.push(1);
                            }
                        }
                    }
                    KeyCode::Tab | KeyCode::Right => {
                        // Move to custom input
                        *focus = WordlistFocus::CustomInput;
                    }
                    KeyCode::Esc => {
                        app.prev_step();
                    }
                    _ => {}
                }
            }
            WordlistFocus::CustomInput => {
                // Typing custom file path
                match key.code {
                    KeyCode::Char(c) => {
                        current_input.insert(*cursor, c);
                        *cursor += 1;
                    }
                    KeyCode::Backspace => {
                        if *cursor > 0 {
                            *cursor -= 1;
                            current_input.remove(*cursor);
                        }
                    }
                    KeyCode::Delete => {
                        if *cursor < current_input.len() {
                            current_input.remove(*cursor);
                        }
                    }
                    KeyCode::Left => {
                        if *cursor > 0 {
                            *cursor -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if *cursor < current_input.len() {
                            *cursor += 1;
                        }
                    }
                    KeyCode::Up => {
                        // Navigate up to predefined list
                        *focus = WordlistFocus::PredefinedList { cursor: 0 };
                    }
                    KeyCode::Down => {
                        // Navigate down to custom URL input
                        *focus = WordlistFocus::CustomUrlInput;
                    }
                    KeyCode::Home => {
                        *cursor = 0;
                    }
                    KeyCode::End => {
                        *cursor = current_input.len();
                    }
                    KeyCode::Enter => {
                        // Try to add the current path
                        if !current_input.is_empty() {
                            match SetupApp::validate_wordlist_path(current_input) {
                                Ok(()) => {
                                    // Valid path - add to list and clear input
                                    custom_paths.push(current_input.clone());
                                    current_input.clear();
                                    *cursor = 0;
                                }
                                Err(_) => {
                                    // Invalid path - clear and let user try again
                                    current_input.clear();
                                    *cursor = 0;
                                }
                            }
                        }
                    }
                    KeyCode::Tab => {
                        // Move to custom URL input
                        *focus = WordlistFocus::CustomUrlInput;
                    }
                    KeyCode::Esc => {
                        if !current_input.is_empty() {
                            // Clear current input
                            current_input.clear();
                            *cursor = 0;
                        } else {
                            // Go back to predefined list
                            *focus = WordlistFocus::PredefinedList { cursor: 0 };
                        }
                    }
                    KeyCode::BackTab => {
                        // Move back to predefined list (Shift+Tab)
                        *focus = WordlistFocus::PredefinedList { cursor: 0 };
                    }
                    _ => {}
                }
            }
            WordlistFocus::CustomUrlInput => {
                // Typing custom URL
                match key.code {
                    KeyCode::Char(c) => {
                        custom_url.push(c);
                    }
                    KeyCode::Backspace => {
                        custom_url.pop();
                    }
                    KeyCode::Up => {
                        // Navigate up to custom file path input
                        *focus = WordlistFocus::CustomInput;
                    }
                    KeyCode::Down => {
                        // Navigate down to Done button
                        *focus = WordlistFocus::Done;
                    }
                    KeyCode::Enter => {
                        // Move to source name input if URL is not empty
                        if !custom_url.is_empty() {
                            *focus = WordlistFocus::CustomUrlSource;
                        }
                    }
                    KeyCode::Tab => {
                        // Move to Done
                        *focus = WordlistFocus::Done;
                    }
                    KeyCode::Esc => {
                        if !custom_url.is_empty() {
                            custom_url.clear();
                        } else {
                            *focus = WordlistFocus::CustomInput;
                        }
                    }
                    _ => {}
                }
            }
            WordlistFocus::CustomUrlSource => {
                // Typing custom URL source name
                match key.code {
                    KeyCode::Char(c) => {
                        custom_url_source.push(c);
                    }
                    KeyCode::Backspace => {
                        custom_url_source.pop();
                    }
                    KeyCode::Up => {
                        // Navigate up to custom URL input
                        *focus = WordlistFocus::CustomUrlInput;
                    }
                    KeyCode::Down => {
                        // Navigate down to Done button
                        *focus = WordlistFocus::Done;
                    }
                    KeyCode::Enter => {
                        // Add URL to a list (we'd need to add a field for this)
                        // For now, just go back to URL input
                        if !custom_url_source.is_empty() && !custom_url.is_empty() {
                            // Would add to custom URL list here
                            custom_url.clear();
                            custom_url_source.clear();
                            *focus = WordlistFocus::CustomUrlInput;
                        }
                    }
                    KeyCode::Esc => {
                        if !custom_url_source.is_empty() {
                            custom_url_source.clear();
                        } else {
                            *focus = WordlistFocus::CustomUrlInput;
                        }
                    }
                    _ => {}
                }
            }
            WordlistFocus::Done => {
                // Focused on Done button
                match key.code {
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        // Start downloads and proceed
                        app.next_step();
                    }
                    KeyCode::BackTab | KeyCode::Left => {
                        // Move focus back to custom URL input
                        *focus = WordlistFocus::CustomUrlInput;
                    }
                    KeyCode::Up => {
                        // Move focus back to custom URL input (logical up from Done)
                        *focus = WordlistFocus::CustomUrlInput;
                    }
                    KeyCode::Esc => {
                        app.prev_step();
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Handles keys on the enhanced detection screen.
fn handle_enhanced_detection_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::EnhancedDetection { selected } = &mut app.state {
        match key.code {
            KeyCode::Up | KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('j') => {
                *selected = if *selected == 0 { 1 } else { 0 };
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => *selected = 1,
            KeyCode::Char('n') | KeyCode::Char('N') => *selected = 0,
            KeyCode::Enter | KeyCode::Char(' ') => app.next_step(),
            KeyCode::Backspace | KeyCode::Left | KeyCode::Char('p') => app.prev_step(),
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            _ => {}
        }
    }
}

/// Handles keys on the token input screen.
fn handle_token_input_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::TokenInput { token, cursor } = &mut app.state {
        match key.code {
            KeyCode::Char(c) => {
                token.insert(*cursor, c);
                *cursor += 1;
            }
            KeyCode::Backspace => {
                if *cursor > 0 {
                    *cursor -= 1;
                    token.remove(*cursor);
                }
            }
            KeyCode::Delete => {
                if *cursor < token.len() {
                    token.remove(*cursor);
                }
            }
            KeyCode::Left => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
            }
            KeyCode::Right => {
                if *cursor < token.len() {
                    *cursor += 1;
                }
            }
            KeyCode::Home => *cursor = 0,
            KeyCode::End => *cursor = token.len(),
            KeyCode::Enter => {
                if !token.is_empty() {
                    app.next_step();
                }
            }
            KeyCode::Esc => app.prev_step(),
            _ => {}
        }
    }
}

/// Handles keys on the downloading screen.
fn handle_downloading_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::Downloading { failed, .. } = &app.state {
        match key.code {
            KeyCode::Enter if *failed => app.next_step(),
            KeyCode::Esc if *failed => app.next_step(),
            KeyCode::Char('q') if *failed => app.should_quit = true,
            _ => {}
        }
    }
}

/// Handles keys on the AI configuration screen.
fn handle_ai_config_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::AiConfig {
        selected,
        api_url,
        api_key,
        model,
        focus,
        cursor,
    } = &mut app.state
    {
        match focus {
            AiConfigFocus::EnableToggle => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        *selected = 1;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        *selected = 0;
                    }
                    KeyCode::Up | KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('j') => {
                        *selected = if *selected == 0 { 1 } else { 0 };
                    }
                    KeyCode::Tab if *selected == 1 => {
                        // Move to API URL field if AI is enabled
                        *focus = AiConfigFocus::ApiUrl;
                        *cursor = api_url.len();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if *selected == 0 {
                            // AI disabled, skip to next step
                            app.next_step();
                        } else {
                            // Move to API URL field
                            *focus = AiConfigFocus::ApiUrl;
                            *cursor = api_url.len();
                        }
                    }
                    KeyCode::Backspace | KeyCode::Left | KeyCode::Char('p') => {
                        app.prev_step();
                    }
                    KeyCode::Esc => app.prev_step(),
                    _ => {}
                }
            }
            AiConfigFocus::ApiUrl => match key.code {
                KeyCode::Char(c) => {
                    api_url.insert(*cursor, c);
                    *cursor += 1;
                }
                KeyCode::Backspace => {
                    if *cursor > 0 {
                        *cursor -= 1;
                        api_url.remove(*cursor);
                    }
                }
                KeyCode::Delete => {
                    if *cursor < api_url.len() {
                        api_url.remove(*cursor);
                    }
                }
                KeyCode::Left => {
                    if *cursor > 0 {
                        *cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if *cursor < api_url.len() {
                        *cursor += 1;
                    }
                }
                KeyCode::Home => *cursor = 0,
                KeyCode::End => *cursor = api_url.len(),
                KeyCode::Tab | KeyCode::Down | KeyCode::Enter => {
                    *focus = AiConfigFocus::ApiKey;
                    *cursor = api_key.len();
                }
                KeyCode::BackTab | KeyCode::Up => {
                    *focus = AiConfigFocus::EnableToggle;
                }
                KeyCode::Esc => {
                    *focus = AiConfigFocus::EnableToggle;
                }
                _ => {}
            },
            AiConfigFocus::ApiKey => match key.code {
                KeyCode::Char(c) => {
                    api_key.insert(*cursor, c);
                    *cursor += 1;
                }
                KeyCode::Backspace => {
                    if *cursor > 0 {
                        *cursor -= 1;
                        api_key.remove(*cursor);
                    }
                }
                KeyCode::Delete => {
                    if *cursor < api_key.len() {
                        api_key.remove(*cursor);
                    }
                }
                KeyCode::Left => {
                    if *cursor > 0 {
                        *cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if *cursor < api_key.len() {
                        *cursor += 1;
                    }
                }
                KeyCode::Home => *cursor = 0,
                KeyCode::End => *cursor = api_key.len(),
                KeyCode::Tab | KeyCode::Down | KeyCode::Enter => {
                    *focus = AiConfigFocus::Model;
                    *cursor = model.len();
                }
                KeyCode::BackTab | KeyCode::Up => {
                    *focus = AiConfigFocus::ApiUrl;
                    *cursor = api_url.len();
                }
                KeyCode::Esc => {
                    *focus = AiConfigFocus::EnableToggle;
                }
                _ => {}
            },
            AiConfigFocus::Model => {
                match key.code {
                    KeyCode::Char(c) => {
                        model.insert(*cursor, c);
                        *cursor += 1;
                    }
                    KeyCode::Backspace => {
                        if *cursor > 0 {
                            *cursor -= 1;
                            model.remove(*cursor);
                        }
                    }
                    KeyCode::Delete => {
                        if *cursor < model.len() {
                            model.remove(*cursor);
                        }
                    }
                    KeyCode::Left => {
                        if *cursor > 0 {
                            *cursor -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if *cursor < model.len() {
                            *cursor += 1;
                        }
                    }
                    KeyCode::Home => *cursor = 0,
                    KeyCode::End => *cursor = model.len(),
                    KeyCode::Enter => {
                        // Confirm and proceed to next step
                        app.next_step();
                    }
                    KeyCode::Tab | KeyCode::Down => {
                        // Wrap back to enable toggle
                        *focus = AiConfigFocus::EnableToggle;
                    }
                    KeyCode::BackTab | KeyCode::Up => {
                        *focus = AiConfigFocus::ApiKey;
                        *cursor = api_key.len();
                    }
                    KeyCode::Esc => {
                        *focus = AiConfigFocus::EnableToggle;
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Handles keys on the cute cat screen.
fn handle_cute_cat_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('y') => {
            app.show_cat = true;
            app.next_step();
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.show_cat = false;
            app.next_step();
        }
        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('p') => app.prev_step(),
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        _ => {}
    }
}

/// Handles keys on the showing cat screen.
///
/// The cat screen auto-advances after 3 seconds, but we allow quitting early.
fn handle_showing_cat_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        // Allow quitting, but otherwise ignore input (auto-advances via timer)
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        // Allow skipping ahead manually if user doesn't want to wait
        KeyCode::Enter | KeyCode::Char(' ') => app.next_step(),
        _ => {}
    }
}

/// Handles keys on the quick searches configuration screen.
fn handle_quick_searches_keys(app: &mut SetupApp, key: KeyEvent) {
    if let SetupState::QuickSearches {
        entries,
        selected,
        current_input,
        cursor,
    } = &mut app.state
    {
        match key.code {
            // Navigate entries
            KeyCode::Up | KeyCode::Char('k') if current_input.is_empty() => {
                if *selected > 0 {
                    *selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') if current_input.is_empty() => {
                if !entries.is_empty() && *selected < entries.len().saturating_sub(1) {
                    *selected += 1;
                }
            }
            // Delete selected entry
            KeyCode::Delete => {
                if !entries.is_empty() {
                    entries.remove(*selected);
                    if *selected >= entries.len() && !entries.is_empty() {
                        *selected = entries.len() - 1;
                    }
                }
            }
            // Backspace: delete from input, or remove selected entry if input is empty
            KeyCode::Backspace => {
                if *cursor > 0 {
                    *cursor -= 1;
                    current_input.remove(*cursor);
                } else if current_input.is_empty() && !entries.is_empty() {
                    entries.remove(*selected);
                    if *selected >= entries.len() && !entries.is_empty() {
                        *selected = entries.len() - 1;
                    }
                }
            }
            // Enter: add new entry if input has text, otherwise proceed to next step
            KeyCode::Enter => {
                if !current_input.is_empty() {
                    // Validate: must contain '=' and '{}'
                    if current_input.contains('=') && current_input.contains("{}") {
                        entries.push(current_input.clone());
                        current_input.clear();
                        *cursor = 0;
                        *selected = entries.len().saturating_sub(1);
                    }
                    // Invalid format - just clear so user can retry
                    else {
                        current_input.clear();
                        *cursor = 0;
                    }
                } else {
                    app.next_step();
                }
            }
            // Text input for new entry
            KeyCode::Char(c) => {
                current_input.insert(*cursor, c);
                *cursor += 1;
            }
            KeyCode::Left => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
            }
            KeyCode::Right => {
                if *cursor < current_input.len() {
                    *cursor += 1;
                }
            }
            KeyCode::Home => *cursor = 0,
            KeyCode::End => *cursor = current_input.len(),
            KeyCode::Esc => app.prev_step(),
            _ => {}
        }
    }
}

/// Handles keys on the complete screen.
fn handle_complete_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
}
