//! Keyboard input handling for the setup wizard.
//!
//! This module processes keyboard events and translates them into
//! state transitions and updates for the setup wizard.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{SetupApp, SetupState, WordlistFocus};
use super::themes::THEMES;

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
            WordlistFocus::PredefinedList => {
                // Navigating predefined wordlist checkboxes
                let predefined_wordlists = crate::storage::download::get_predefined_wordlists();
                let _max_index = if predefined_wordlists.is_empty() {
                    0
                } else {
                    predefined_wordlists.len() - 1
                };

                match key.code {
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        // Navigate up (only if we have wordlists)
                        if !predefined_wordlists.is_empty() {
                            // For now, we'll just stay at the first item
                            // In a full implementation, we'd track the cursor position
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        // Navigate down
                        if !predefined_wordlists.is_empty() {
                            // For now, we'll just stay at the first item
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        // Toggle selection of first wordlist (simplified)
                        // In a full implementation, we'd track which item is highlighted
                        if selected_predefined.is_empty() {
                            selected_predefined.push(0);
                        } else if selected_predefined.contains(&0) {
                            selected_predefined.retain(|&x| x != 0);
                        } else {
                            selected_predefined.push(0);
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
                            *focus = WordlistFocus::PredefinedList;
                        }
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
                    KeyCode::Up | KeyCode::Tab | KeyCode::BackTab => {
                        // Move focus back to predefined list
                        *focus = WordlistFocus::PredefinedList;
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

/// Handles keys on the complete screen.
fn handle_complete_keys(app: &mut SetupApp, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
}
