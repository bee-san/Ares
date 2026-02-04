//! First-run setup wizard for Ciphey TUI.
//!
//! This module provides a beautiful card-based setup wizard that guides users
//! through initial configuration when Ciphey is run for the first time.
//! Features include:
//!
//! - Live theme preview as users browse color schemes
//! - Inline progress bars for downloads
//! - Keyboard-driven navigation with vim bindings
//! - Skip option for users who want defaults

mod app;
mod input;
mod themes;
mod ui;

use std::collections::HashMap;
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

pub use app::{DownloadProgress, SetupApp, SetupState, WordlistFocus};
use input::handle_setup_key_event;
pub use themes::{ColorScheme, Theme, THEMES};
use ui::draw_setup;

/// Result type for setup wizard operations.
type SetupResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Tick rate for UI updates (in milliseconds).
const TICK_RATE_MS: u64 = 50;

/// Message from the download thread.
enum DownloadMessage {
    /// Progress update (0.0 to 1.0)
    Progress(f32, String),
    /// Download completed successfully
    Complete(String),
    /// Download failed with error
    Failed(String),
}

/// Message from the wordlist download thread.
enum WordlistDownloadMessage {
    /// Progress update (current index, status message)
    Progress(usize, String),
    /// Single wordlist download completed
    WordlistComplete(String),
    /// Single wordlist download failed
    WordlistFailed(String),
    /// All downloads complete
    AllComplete,
}

/// Runs the first-time setup wizard in TUI mode.
///
/// This function initializes the terminal, runs the setup wizard event loop,
/// and returns the user's configuration choices as a HashMap.
///
/// # Returns
///
/// Returns `Ok(Some(HashMap))` with the configuration if setup was completed,
/// `Ok(None)` if the user chose to skip setup (use defaults),
/// or an error if terminal initialization fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Terminal raw mode cannot be enabled
/// - The alternate screen cannot be entered
/// - Terminal initialization fails
pub fn run_setup_wizard() -> SetupResult<Option<HashMap<String, String>>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create the setup app
    let mut app = SetupApp::new();

    // Run the event loop
    let result = run_setup_event_loop(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Return the result
    result.map(|_| {
        if app.skipped {
            None
        } else {
            Some(app.build_config())
        }
    })
}

/// Runs the setup wizard event loop.
fn run_setup_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut SetupApp,
) -> SetupResult<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();

    // Channel for download progress (only created when downloading)
    let mut download_rx: Option<mpsc::Receiver<DownloadMessage>> = None;
    // Channel for wordlist download progress
    let mut wordlist_download_rx: Option<mpsc::Receiver<WordlistDownloadMessage>> = None;

    loop {
        // Draw the UI
        terminal.draw(|frame| draw_setup(frame, app))?;

        // Check if we need to start wordlist downloads
        if let SetupState::WordlistConfig {
            selected_predefined,
            custom_paths,
            download_progress,
            ..
        } = &mut app.state
        {
            // Check if we need to initiate downloads (user clicked Done and we have selections)
            if download_progress.is_none()
                && (!selected_predefined.is_empty() || !custom_paths.is_empty())
                && wordlist_download_rx.is_none()
            {
                // This would be triggered when transitioning to next step
                // For now, we'll handle it in the next_step() transition
            }

            // If download_progress is Some and we don't have a channel, start downloads
            if let Some(progress) = download_progress {
                if wordlist_download_rx.is_none() && progress.current == 0 {
                    let (tx, rx) = mpsc::channel();
                    wordlist_download_rx = Some(rx);

                    let selected_indices = selected_predefined.clone();
                    let custom_path_list = custom_paths.clone();

                    // Spawn download thread
                    thread::spawn(move || {
                        let predefined = crate::storage::download::get_predefined_wordlists();
                        let mut current_index = 0;
                        let total = selected_indices.len() + custom_path_list.len();

                        // Download predefined wordlists
                        for &idx in &selected_indices {
                            if idx < predefined.len() {
                                current_index += 1;
                                let wordlist = &predefined[idx];

                                let _ = tx.send(WordlistDownloadMessage::Progress(
                                    current_index,
                                    format!("Downloading {}...", wordlist.name),
                                ));

                                match crate::storage::download::download_wordlist_from_url(
                                    &wordlist.url,
                                ) {
                                    Ok(words) => {
                                        match crate::storage::download::import_wordlist_with_bloom_rebuild(
                                            &words,
                                            &wordlist.source_id,
                                        ) {
                                            Ok(_) => {
                                                let _ = tx.send(
                                                    WordlistDownloadMessage::WordlistComplete(
                                                        wordlist.name.clone(),
                                                    ),
                                                );
                                            }
                                            Err(e) => {
                                                let _ = tx.send(
                                                    WordlistDownloadMessage::WordlistFailed(
                                                        format!("{}: {}", wordlist.name, e),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let _ = tx.send(WordlistDownloadMessage::WordlistFailed(
                                            format!("{}: {}", wordlist.name, e),
                                        ));
                                    }
                                }
                            }
                        }

                        // Import custom file paths
                        for path in &custom_path_list {
                            current_index += 1;

                            let _ = tx.send(WordlistDownloadMessage::Progress(
                                current_index,
                                format!("Importing {}...", path),
                            ));

                            // Extract filename for source
                            let source = std::path::Path::new(path)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("custom")
                                .to_string();

                            match crate::storage::download::import_wordlist_from_file(path, &source)
                            {
                                Ok(_) => {
                                    let _ = tx.send(WordlistDownloadMessage::WordlistComplete(
                                        path.clone(),
                                    ));
                                }
                                Err(e) => {
                                    let _ = tx.send(WordlistDownloadMessage::WordlistFailed(
                                        format!("{}: {}", path, e),
                                    ));
                                }
                            }
                        }

                        // All done
                        let _ = tx.send(WordlistDownloadMessage::AllComplete);
                    });
                }
            }
        }

        // Check for wordlist download progress updates
        if let Some(ref rx) = wordlist_download_rx {
            match rx.try_recv() {
                Ok(WordlistDownloadMessage::Progress(current, status)) => {
                    if let SetupState::WordlistConfig {
                        download_progress, ..
                    } = &mut app.state
                    {
                        if let Some(progress) = download_progress {
                            progress.current = current;
                            progress.status = status;
                        }
                    }
                }
                Ok(WordlistDownloadMessage::WordlistComplete(_name)) => {
                    // Individual wordlist completed, continue
                }
                Ok(WordlistDownloadMessage::WordlistFailed(error)) => {
                    if let SetupState::WordlistConfig {
                        download_progress, ..
                    } = &mut app.state
                    {
                        if let Some(progress) = download_progress {
                            progress.failed.push(error);
                        }
                    }
                }
                Ok(WordlistDownloadMessage::AllComplete) => {
                    // Save wordlist selections before moving to next step
                    if let SetupState::WordlistConfig {
                        custom_paths,
                        selected_predefined,
                        ..
                    } = &app.state
                    {
                        app.wordlist_paths = custom_paths.clone();
                        app.selected_predefined_wordlists = selected_predefined.clone();
                    }
                    // All downloads complete, move to next step
                    app.state = SetupState::EnhancedDetection { selected: 0 };
                    wordlist_download_rx = None;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No message yet
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Thread disconnected unexpectedly
                    if let SetupState::WordlistConfig {
                        download_progress, ..
                    } = &mut app.state
                    {
                        if let Some(progress) = download_progress {
                            progress
                                .failed
                                .push("Download thread disconnected unexpectedly".to_string());
                        }
                    }
                    wordlist_download_rx = None;
                }
            }
        }

        // Check if we need to start a download
        if let SetupState::Downloading {
            progress,
            status: _,
            failed,
            ..
        } = &app.state
        {
            // Only start download if progress is 0 and we don't already have a receiver
            if *progress == 0.0 && !*failed && download_rx.is_none() {
                if let Some(token) = &app.hf_token {
                    let (tx, rx) = mpsc::channel();
                    download_rx = Some(rx);

                    // Get the model path
                    let mut model_path = crate::config::get_config_file_path();
                    model_path.pop();
                    model_path.push("models");

                    // Create models directory
                    let _ = std::fs::create_dir_all(&model_path);
                    model_path.push("model.bin");

                    // Store model path in app
                    app.model_path = Some(model_path.display().to_string());

                    let token_clone = token.clone();
                    let path_clone = model_path.clone();

                    // Spawn download thread
                    thread::spawn(move || {
                        // Send initial progress
                        let _ = tx.send(DownloadMessage::Progress(
                            0.05,
                            "Starting download...".to_string(),
                        ));

                        // Attempt to download
                        match gibberish_or_not::download_model_with_progress_bar(
                            &path_clone,
                            Some(&token_clone),
                        ) {
                            Ok(_) => {
                                let _ = tx.send(DownloadMessage::Complete(
                                    path_clone.display().to_string(),
                                ));
                            }
                            Err(e) => {
                                let _ = tx.send(DownloadMessage::Failed(e.to_string()));
                            }
                        }
                    });
                }
            }
        }

        // Check for download progress updates
        if let Some(ref rx) = download_rx {
            match rx.try_recv() {
                Ok(DownloadMessage::Progress(progress, status)) => {
                    if let SetupState::Downloading {
                        progress: p,
                        status: s,
                        ..
                    } = &mut app.state
                    {
                        *p = progress;
                        *s = status;
                    }
                }
                Ok(DownloadMessage::Complete(path)) => {
                    app.model_path = Some(path);
                    app.next_step();
                    download_rx = None;
                }
                Ok(DownloadMessage::Failed(error)) => {
                    if let SetupState::Downloading {
                        failed,
                        error: err,
                        status,
                        ..
                    } = &mut app.state
                    {
                        *failed = true;
                        *err = Some(error);
                        *status = "Download failed".to_string();
                    }
                    download_rx = None;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No message yet, simulate progress for visual feedback
                    if let SetupState::Downloading {
                        progress, status, ..
                    } = &mut app.state
                    {
                        if *progress < 0.95 {
                            // Slowly increment progress for visual feedback
                            // The actual completion will come from the thread
                            *progress += 0.001;
                            if *progress > 0.1 && *progress < 0.9 {
                                *status = "Downloading model...".to_string();
                            }
                        }
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Thread disconnected unexpectedly
                    if let SetupState::Downloading {
                        failed,
                        error,
                        status,
                        ..
                    } = &mut app.state
                    {
                        *failed = true;
                        *error = Some("Download thread disconnected unexpectedly".to_string());
                        *status = "Download failed".to_string();
                    }
                    download_rx = None;
                }
            }
        }

        // Calculate timeout for event polling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        // Poll for events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release)
                if key.kind == KeyEventKind::Press {
                    handle_setup_key_event(app, key);
                }
            }
        }

        // Check for tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        // Check if we should exit
        if app.should_quit || matches!(app.state, SetupState::Complete) {
            break;
        }
    }

    Ok(())
}
