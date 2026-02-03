//! Main TUI entry point and event loop for Ciphey.
//!
//! This module provides the main `run_tui` function that initializes the terminal,
//! runs the decoding in a background thread, and handles the event loop.

use std::io::{self, Stdout};
use std::sync::mpsc;
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
/// * `input_text` - The text to decode
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
pub fn run_tui(input_text: &str, config: Config) -> TuiResult<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and colors
    let mut app = App::new(input_text.to_string());
    let colors = TuiColors::from_config(&config);

    // Channel for receiving decode result
    let (tx, rx) = mpsc::channel::<Option<DecoderResult>>();

    // Spawn background thread for decoding
    let input_for_thread = input_text.to_string();
    let config_for_thread = config.clone();
    thread::spawn(move || {
        // Set the global config for the worker thread
        crate::config::set_global_config(config_for_thread.clone());

        // Perform the cracking
        let result = crate::perform_cracking(&input_for_thread, config_for_thread);

        // Send result back (ignore error if receiver dropped)
        let _ = tx.send(result);
    });

    // Run the main loop
    let result = run_event_loop(&mut terminal, &mut app, &colors, rx);

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
fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    colors: &TuiColors,
    result_receiver: mpsc::Receiver<Option<DecoderResult>>,
) -> TuiResult<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();
    let start_time = Instant::now();
    let mut tick_count: usize = 0;

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

        // Check for decode result (non-blocking)
        if let Ok(result) = result_receiver.try_recv() {
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
