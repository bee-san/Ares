//! Application state machine for the Ciphey TUI.
//!
//! This module defines the core state management for the terminal user interface,
//! handling transitions between loading, results, and failure states.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::checkers::checker_result::CheckResult;
use crate::decoders::crack_results::CrackResult;
use crate::DecoderResult;

/// A request for human confirmation of a potential plaintext.
#[derive(Debug, Clone)]
pub struct HumanConfirmationRequest {
    /// The potential plaintext text to confirm.
    pub text: String,
    /// Description of why this might be plaintext (e.g., "English words detected").
    pub description: String,
    /// Name of the checker that found this candidate.
    pub checker_name: String,
}

impl From<&CheckResult> for HumanConfirmationRequest {
    fn from(check_result: &CheckResult) -> Self {
        Self {
            text: check_result.text.clone(),
            description: check_result.description.clone(),
            checker_name: check_result.checker_name.to_string(),
        }
    }
}

/// Represents the current state of the TUI application.
#[derive(Debug)]
pub enum AppState {
    /// The application is processing input and waiting for results.
    Loading {
        /// When the loading started, used to calculate elapsed time.
        start_time: Instant,
        /// Current index into the quotes array for display rotation.
        current_quote: usize,
        /// Current frame of the spinner animation.
        spinner_frame: usize,
    },
    /// Waiting for human confirmation of a potential plaintext.
    HumanConfirmation {
        /// When the loading started (preserved from Loading state).
        start_time: Instant,
        /// Current quote index (preserved from Loading state).
        current_quote: usize,
        /// Current spinner frame (preserved from Loading state).
        spinner_frame: usize,
        /// The confirmation request details.
        request: HumanConfirmationRequest,
        /// Channel to send the user's response back to the cracker thread.
        response_sender: mpsc::Sender<bool>,
    },
    /// Decoding completed successfully with results to display.
    Results {
        /// The successful decoding result containing the path and plaintext.
        result: DecoderResult,
        /// Currently selected step in the decoding path.
        selected_step: usize,
        /// Vertical scroll offset for long content.
        scroll_offset: usize,
    },
    /// Decoding failed to find a solution.
    Failure {
        /// The original input text that could not be decoded.
        input_text: String,
        /// How long the decoding attempt took before failing.
        elapsed: Duration,
    },
}

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

    /// Navigates to the next step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// beginning when reaching the end of the path.
    pub fn next_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = (*selected_step + 1) % path_len;
                *scroll_offset = 0;
            }
        }
    }

    /// Navigates to the previous step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// end when at the beginning of the path.
    pub fn prev_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = if *selected_step == 0 {
                    path_len - 1
                } else {
                    *selected_step - 1
                };
                *scroll_offset = 0;
            }
        }
    }

    /// Gets the currently selected step in the decoding path.
    ///
    /// # Returns
    ///
    /// `Some(&CrackResult)` if in Results state with a valid selection,
    /// `None` otherwise.
    pub fn get_current_step(&self) -> Option<&CrackResult> {
        if let AppState::Results {
            result,
            selected_step,
            ..
        } = &self.state
        {
            result.path.get(*selected_step)
        } else {
            None
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
}
