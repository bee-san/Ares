//! Main TUI entry point and event loop for Ciphey.
//!
//! This module provides the main `run_tui` function that initializes the terminal,
//! runs the decoding in a background thread, and handles the event loop.

use std::io::{self, Stdout};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::config::Config;
use crate::DecoderResult;

use super::app::App;
use super::colors::TuiColors;
use super::human_checker_bridge::{
    init_tui_confirmation_channel, reinit_tui_confirmation_channel, take_confirmation_receiver,
    TuiConfirmationRequest,
};
use super::input::{copy_to_clipboard, handle_key_event, Action};
use super::ui::draw;

/// Result type for TUI operations.
type TuiResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Tick rate for UI updates (in milliseconds).
const TICK_RATE_MS: u64 = 100;

/// How often to rotate quotes (in ticks).
const QUOTE_ROTATION_TICKS: usize = 30;

/// Runs the TUI for Ciphey.
///
/// This function initializes the terminal in raw mode, spawns a background thread
/// for decoding, and runs the main event loop until the user quits or decoding completes.
///
/// # Arguments
///
/// * `input_text` - Optional text to decode. If `None`, shows the homescreen for input.
/// * `config` - The Ciphey configuration
///
/// # Returns
///
/// Returns `Ok(())` on successful completion, or an error if terminal setup fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Terminal raw mode cannot be enabled
/// - The alternate screen cannot be entered
/// - Terminal initialization fails
///
/// # Panics
///
/// May panic if terminal initialization fails in an unrecoverable way.
pub fn run_tui(input_text: Option<&str>, config: Config) -> TuiResult<()> {
    // Initialize the human checker bridge channel BEFORE spawning the cracker thread
    // This allows the human checker to communicate with the TUI
    init_tui_confirmation_channel();

    // Take the confirmation receiver for the event loop
    let confirmation_receiver = take_confirmation_receiver();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and colors based on whether we have input text
    let mut app = match input_text {
        Some(text) => App::new(text.to_string()),
        None => App::new_home(),
    };
    let colors = TuiColors::from_config(&config);

    // Only spawn decode thread if we have input text
    // Otherwise, we're in homescreen mode and will spawn later
    let (result_receiver, confirmation_receiver) = if let Some(text) = input_text {
        // Channel for receiving decode result
        let (tx, rx) = mpsc::channel::<Option<DecoderResult>>();

        // Spawn background thread for decoding
        let input_for_thread = text.to_string();
        let config_for_thread = config.clone();
        thread::spawn(move || {
            // Set the global config for the worker thread
            crate::config::set_global_config(config_for_thread.clone());

            // Perform the cracking
            let result = crate::perform_cracking(&input_for_thread, config_for_thread);

            // Send result back (ignore error if receiver dropped)
            let _ = tx.send(result);
        });

        (Some(rx), confirmation_receiver)
    } else {
        // No input - homescreen mode, no decode thread yet
        (None, confirmation_receiver)
    };

    // Run the main loop (pass config for potential reruns)
    let result = run_event_loop(
        &mut terminal,
        &mut app,
        &colors,
        &config,
        result_receiver,
        confirmation_receiver,
    );

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Runs the main event loop.
///
/// Handles UI updates, keyboard input, and checks for decode completion.
/// Also handles human checker confirmation requests from the cracker thread.
/// Supports rerunning Ciphey from a selected step by spawning a new decode thread.
fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    colors: &TuiColors,
    config: &Config,
    initial_result_receiver: Option<mpsc::Receiver<Option<DecoderResult>>>,
    initial_confirmation_receiver: Option<Receiver<TuiConfirmationRequest>>,
) -> TuiResult<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();
    let mut start_time = Instant::now();
    let mut tick_count: usize = 0;

    // Use Option so we can replace the receivers when rerunning
    let mut result_receiver = initial_result_receiver;
    let mut confirmation_receiver = initial_confirmation_receiver;

    loop {
        // Draw the UI
        terminal.draw(|frame| draw(frame, app, colors))?;

        // Calculate timeout for event polling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        // Poll for events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release)
                if key.kind == KeyEventKind::Press {
                    let action = handle_key_event(app, key);

                    // Handle actions
                    match action {
                        Action::CopyToClipboard(text) => match copy_to_clipboard(&text) {
                            Ok(()) => {
                                app.set_status("Copied to clipboard!".to_string());
                            }
                            Err(e) => {
                                app.set_status(format!("Copy failed: {}", e));
                            }
                        },
                        Action::SubmitHomeInput(new_input) => {
                            // User submitted text from the home screen
                            // Transition from Home to Loading and spawn decode thread
                            crate::checkers::reset_human_checker_state();
                            crate::timer::resume();
                            crate::storage::wait_athena_storage::clear_plaintext_results();

                            // Update app state
                            app.input_text = new_input.clone();
                            app.state = super::app::AppState::Loading {
                                start_time: Instant::now(),
                                current_quote: 0,
                                spinner_frame: 0,
                            };
                            app.clear_status();

                            // Reset timing
                            start_time = Instant::now();
                            tick_count = 0;

                            // Initialize confirmation channel for the decode thread
                            confirmation_receiver = reinit_tui_confirmation_channel();

                            // Spawn decode thread
                            let (tx, rx) = mpsc::channel::<Option<DecoderResult>>();
                            result_receiver = Some(rx);

                            let config_clone = config.clone();
                            thread::spawn(move || {
                                crate::config::set_global_config(config_clone.clone());
                                let result = crate::perform_cracking(&new_input, config_clone);
                                let _ = tx.send(result);
                            });

                            app.set_status("Decoding...".to_string());
                        }
                        Action::RerunFromSelected(new_input) => {
                            // Reset all global state for a fresh run
                            // This ensures the rerun behaves exactly like a fresh CLI invocation
                            crate::checkers::reset_human_checker_state();
                            crate::timer::resume();
                            crate::storage::wait_athena_storage::clear_plaintext_results();

                            // Reset app to loading state with new input
                            app.input_text = new_input.clone();
                            app.state = super::app::AppState::Loading {
                                start_time: Instant::now(),
                                current_quote: 0,
                                spinner_frame: 0,
                            };
                            app.clear_status();

                            // Reset timing
                            start_time = Instant::now();
                            tick_count = 0;

                            // Reinitialize the human checker confirmation channel for the new run
                            // This creates a fresh channel so the new decode thread can communicate
                            confirmation_receiver = reinit_tui_confirmation_channel();

                            // Create new channel and spawn new decode thread
                            let (tx, rx) = mpsc::channel::<Option<DecoderResult>>();
                            result_receiver = Some(rx);

                            let config_clone = config.clone();
                            thread::spawn(move || {
                                crate::config::set_global_config(config_clone.clone());
                                let result = crate::perform_cracking(&new_input, config_clone);
                                let _ = tx.send(result);
                            });

                            app.set_status("Rerunning Ciphey...".to_string());
                        }
                        Action::OpenSettings => {
                            app.open_settings(config);
                            app.set_status("Settings opened. Press Esc to close.".to_string());
                        }
                        Action::SaveSettings => {
                            // Apply settings to config and save
                            if let super::app::AppState::Settings { settings, .. } = &app.state {
                                // Clone the settings to apply
                                let mut new_config = config.clone();
                                settings.apply_to_config(&mut new_config);

                                // Update the global config
                                crate::config::set_global_config(new_config.clone());

                                // Save config to disk
                                if let Err(e) = crate::config::save_config(&new_config) {
                                    app.set_status(format!("Error saving settings: {}", e));
                                } else {
                                    app.set_status("Settings saved!".to_string());
                                }
                            }
                            app.close_settings();
                        }
                        Action::ShowHistoryResult {
                            encoded_text,
                            decoded_text,
                            path,
                        } => {
                            // Reconstruct a DecoderResult from the history entry
                            // Parse the path JSON strings back into CrackResults
                            let crack_results: Vec<crate::decoders::crack_results::CrackResult> =
                                path.iter()
                                    .filter_map(|json_str| serde_json::from_str(json_str).ok())
                                    .collect();

                            let result = DecoderResult {
                                text: vec![decoded_text],
                                path: crack_results,
                            };

                            // Update input_text for display purposes
                            app.input_text = encoded_text;

                            // Set the result state
                            app.set_result(result);
                            app.set_status("Showing saved result from history.".to_string());
                        }
                        Action::None => {}
                    }
                }
            }
        }

        // Check for tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            tick_count += 1;

            // Rotate quote periodically
            if tick_count % QUOTE_ROTATION_TICKS == 0 {
                app.tick(); // Extra tick to ensure quote rotation
            }

            // Clear status message after a few seconds
            if tick_count % 30 == 0 {
                app.clear_status();
            }

            last_tick = Instant::now();
        }

        // Check if should quit
        if app.should_quit {
            break;
        }

        // Check for human confirmation requests (non-blocking)
        if let Some(ref conf_rx) = confirmation_receiver {
            if let Ok(conf_request) = conf_rx.try_recv() {
                // Transition to the human confirmation state
                app.set_human_confirmation(conf_request.request, conf_request.response_tx);
            }
        }

        // Check for decode result (non-blocking)
        if let Some(ref rx) = result_receiver {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Some(decoder_result) => {
                        app.set_result(decoder_result);
                    }
                    None => {
                        let elapsed = start_time.elapsed();
                        app.set_failure(elapsed);
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_rate_reasonable() {
        assert!(TICK_RATE_MS >= 50);
        assert!(TICK_RATE_MS <= 200);
    }

    #[test]
    fn test_quote_rotation_ticks() {
        assert!(QUOTE_ROTATION_TICKS >= 10);
        assert!(QUOTE_ROTATION_TICKS <= 60);
    }
}
