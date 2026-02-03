//! Keyboard input handling for Ciphey's TUI.
//!
//! This module processes keyboard events and translates them into application
//! actions based on the current application state.

use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppState};

/// Actions that may need to be performed outside the input handler.
///
/// Some operations like clipboard access may require special handling
/// in the main event loop, so we return an action to indicate what
/// needs to happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Copy the given string to the system clipboard.
    CopyToClipboard(String),
    /// No action required.
    None,
}

/// Handles a keyboard event and updates the application state accordingly.
///
/// This function processes key events based on the current `AppState`:
///
/// - **All states**: `q` or `Esc` quits, `?` toggles help overlay
/// - **Loading**: Only quit and help are available
/// - **Results**: Navigation with arrow keys/vim bindings, copy with `c`
/// - **Failure**: Only quit and help are available
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Returns
///
/// An `Action` indicating if any follow-up operation is needed (e.g., clipboard copy).
///
/// # Examples
///
/// ```ignore
/// let action = handle_key_event(&mut app, key_event);
/// if let Action::CopyToClipboard(text) = action {
///     copy_to_clipboard(&text)?;
/// }
/// ```
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Action {
    // Handle global key bindings that work in all states
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
        // Ctrl+C should also quit
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
            return Action::None;
        }
        _ => {}
    }

    // Handle state-specific key bindings
    match &app.state {
        AppState::Loading { .. } => {
            // Only quit and help work in loading state
            Action::None
        }
        AppState::Results { result, .. } => {
            // Clone the output text if we might need it for clipboard
            let output_text = result.text.first().cloned();
            let path_len = result.path.len();
            handle_results_keys(app, key, output_text, path_len)
        }
        AppState::Failure { .. } => {
            // Only quit and help work in failure state
            Action::None
        }
    }
}

/// Handles key events specific to the Results state.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
/// * `output_text` - The final output text (if any)
/// * `path_len` - Length of the decoder path
///
/// # Returns
///
/// An `Action` if clipboard copy was requested, otherwise `Action::None`.
fn handle_results_keys(
    app: &mut App,
    key: KeyEvent,
    output_text: Option<String>,
    path_len: usize,
) -> Action {
    match key.code {
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
        // Copy final output to clipboard
        KeyCode::Char('c') => {
            if let Some(text) = output_text {
                Action::CopyToClipboard(text)
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
}
