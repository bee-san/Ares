//! Application state machine for the Ciphey TUI.
//!
//! This module defines the core state management for the terminal user interface,
//! handling transitions between loading, results, settings, and failure states.

use std::sync::mpsc;
use std::time::{Duration, Instant};

// Submodules
pub mod list_editor;
pub mod navigation;
pub mod settings;
pub mod state;
pub mod wordlist;

// Re-export commonly used types
pub use state::{
    AppState, HistoryEntry, HumanConfirmationRequest, PreviousState, SettingsStateSnapshot,
    WordlistFileInfo, WordlistManagerFocus,
};

use crate::DecoderResult;

use super::multiline_text_input::MultilineTextInput;

/// Main application struct managing TUI state and user interactions.
#[derive(Debug)]
pub struct App {
    /// Current state of the application state machine.
    pub state: AppState,
    /// The original input text being decoded.
    pub input_text: String,
    /// Flag indicating the application should exit.
    pub should_quit: bool,
    /// Flag indicating whether the help overlay is visible.
    pub show_help: bool,
    /// Optional status message for user feedback (e.g., clipboard operations).
    pub status_message: Option<String>,
}

impl App {
    /// Creates a new App instance in the Loading state.
    ///
    /// # Arguments
    ///
    /// * `input_text` - The text to be decoded
    ///
    /// # Returns
    ///
    /// A new `App` instance initialized in the `Loading` state.
    pub fn new(input_text: String) -> Self {
        Self {
            state: AppState::Loading {
                start_time: Instant::now(),
                current_quote: 0,
                spinner_frame: 0,
            },
            input_text,
            should_quit: false,
            show_help: false,
            status_message: None,
        }
    }

    /// Creates a new App instance in the Home state (homescreen for input).
    ///
    /// This is used when the user runs Ciphey without providing input text,
    /// allowing them to paste ciphertext directly in the TUI.
    ///
    /// # Returns
    ///
    /// A new `App` instance initialized in the `Home` state.
    pub fn new_home() -> Self {
        // Load history from database
        let history = match crate::storage::database::read_cache_history() {
            Ok(rows) => rows.iter().map(HistoryEntry::from_cache_row).collect(),
            Err(_) => Vec::new(),
        };

        Self {
            state: AppState::Home {
                text_input: MultilineTextInput::new(),
                history,
                selected_history: None,
                history_scroll_offset: 0,
            },
            input_text: String::new(),
            should_quit: false,
            show_help: false,
            status_message: None,
        }
    }

    /// Refreshes the history list from the database.
    ///
    /// Call this after returning from a decode attempt to update the history panel.
    pub fn refresh_history(&mut self) {
        if let AppState::Home {
            history,
            selected_history,
            history_scroll_offset,
            ..
        } = &mut self.state
        {
            *history = match crate::storage::database::read_cache_history() {
                Ok(rows) => rows.iter().map(HistoryEntry::from_cache_row).collect(),
                Err(_) => Vec::new(),
            };
            // Reset selection and scroll when refreshing
            *selected_history = None;
            *history_scroll_offset = 0;
        }
    }

    /// Checks if the app is currently in the Home state.
    ///
    /// # Returns
    ///
    /// `true` if in Home state, `false` otherwise.
    pub fn is_home(&self) -> bool {
        matches!(self.state, AppState::Home { .. })
    }

    /// Gets the text from the Home state text input.
    ///
    /// # Returns
    ///
    /// The text entered by the user, or an empty string if not in Home state.
    pub fn get_home_input(&self) -> String {
        match &self.state {
            AppState::Home { text_input, .. } => text_input.get_text(),
            _ => String::new(),
        }
    }

    /// Transitions from Home state to Loading state with the entered text.
    ///
    /// # Returns
    ///
    /// `Some(input_text)` if transition was successful, `None` if not in Home state
    /// or input is empty.
    pub fn submit_home_input(&mut self) -> Option<String> {
        if let AppState::Home { text_input, .. } = &self.state {
            let input = text_input.get_text();
            if input.trim().is_empty() {
                return None;
            }

            self.input_text = input.clone();
            self.state = AppState::Loading {
                start_time: Instant::now(),
                current_quote: 0,
                spinner_frame: 0,
            };
            Some(input)
        } else {
            None
        }
    }

    /// Updates animation state for the loading screen.
    ///
    /// This method should be called on each tick to advance the spinner
    /// animation and rotate through loading quotes.
    pub fn tick(&mut self) {
        match &mut self.state {
            AppState::Loading {
                spinner_frame,
                current_quote,
                ..
            } => {
                *spinner_frame = spinner_frame.wrapping_add(1);
                // Rotate quotes every ~20 ticks (assuming ~10 ticks/sec, change every 2 seconds)
                if *spinner_frame % 20 == 0 {
                    *current_quote = current_quote.wrapping_add(1);
                }
            }
            AppState::HumanConfirmation {
                spinner_frame,
                current_quote,
                ..
            } => {
                *spinner_frame = spinner_frame.wrapping_add(1);
                if *spinner_frame % 20 == 0 {
                    *current_quote = current_quote.wrapping_add(1);
                }
            }
            _ => {}
        }
    }

    /// Transitions the application to the Results state.
    ///
    /// The selected step defaults to the last step in the path (the plaintext),
    /// so pressing 'c' to copy will copy the final output by default.
    ///
    /// # Arguments
    ///
    /// * `result` - The successful decoding result to display
    pub fn set_result(&mut self, result: DecoderResult) {
        let last_step = result.path.len().saturating_sub(1);
        self.state = AppState::Results {
            result,
            selected_step: last_step,
            scroll_offset: 0,
        };
    }

    /// Transitions the application to the Failure state.
    ///
    /// # Arguments
    ///
    /// * `elapsed` - How long the decoding attempt took
    pub fn set_failure(&mut self, elapsed: Duration) {
        self.state = AppState::Failure {
            input_text: self.input_text.clone(),
            elapsed,
        };
    }

    /// Transitions to the HumanConfirmation state to ask the user to verify plaintext.
    ///
    /// # Arguments
    ///
    /// * `request` - The confirmation request with candidate text details
    /// * `response_sender` - Channel to send the user's response
    pub fn set_human_confirmation(
        &mut self,
        request: HumanConfirmationRequest,
        response_sender: mpsc::Sender<bool>,
    ) {
        // Preserve loading state animation values
        let (start_time, current_quote, spinner_frame) = match &self.state {
            AppState::Loading {
                start_time,
                current_quote,
                spinner_frame,
            } => (*start_time, *current_quote, *spinner_frame),
            AppState::HumanConfirmation {
                start_time,
                current_quote,
                spinner_frame,
                ..
            } => (*start_time, *current_quote, *spinner_frame),
            _ => (Instant::now(), 0, 0),
        };

        self.state = AppState::HumanConfirmation {
            start_time,
            current_quote,
            spinner_frame,
            request,
            response_sender,
        };
    }

    /// Sends a response to the human confirmation request and returns to Loading state.
    ///
    /// # Arguments
    ///
    /// * `accepted` - Whether the user accepted the plaintext candidate
    pub fn respond_to_confirmation(&mut self, accepted: bool) {
        if let AppState::HumanConfirmation {
            start_time,
            current_quote,
            spinner_frame,
            response_sender,
            ..
        } = &self.state
        {
            // Send the response (ignore error if receiver dropped)
            let _ = response_sender.send(accepted);

            // Return to loading state
            self.state = AppState::Loading {
                start_time: *start_time,
                current_quote: *current_quote,
                spinner_frame: *spinner_frame,
            };
        }
    }

    /// Toggles the visibility of the help overlay.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Sets a temporary status message for user feedback.
    ///
    /// # Arguments
    ///
    /// * `msg` - The status message to display
    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some(msg);
    }

    /// Clears the current status message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Returns to the Home state from Results or Failure state.
    ///
    /// Clears the input text and refreshes history from the database.
    /// This allows users to decode another message without restarting the app.
    pub fn return_to_home(&mut self) {
        // Load fresh history from database
        let history = match crate::storage::database::read_cache_history() {
            Ok(rows) => rows.iter().map(HistoryEntry::from_cache_row).collect(),
            Err(_) => Vec::new(),
        };

        self.state = AppState::Home {
            text_input: MultilineTextInput::new(),
            history,
            selected_history: None,
            history_scroll_offset: 0,
        };
        self.input_text.clear();
        self.clear_status();
    }
}
