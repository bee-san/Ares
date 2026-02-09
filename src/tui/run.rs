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

use crate::checkers::checker_type::Check;
use crate::config::Config;
use crate::decoders::crack_results::CrackResult;
use crate::storage::database::{
    get_cache_by_id, insert_branch, link_as_branch, BranchType, CacheEntry,
};
use crate::{CrackingResult, DecoderResult};

use super::app::App;
use super::colors::TuiColors;
use super::human_checker_bridge::{
    init_tui_confirmation_channel, reinit_tui_confirmation_channel, take_confirmation_receiver,
    TuiConfirmationRequest,
};
use super::input::{copy_to_clipboard, handle_key_event, Action};
use super::spinner::random_quote_index;
use super::ui::draw;

/// Result type for TUI operations.
type TuiResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Tick rate for UI updates (in milliseconds).
const TICK_RATE_MS: u64 = 100;

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
        // Channel for receiving decode result with cache_id
        let (tx, rx) = mpsc::channel::<CrackingResult>();

        // Spawn background thread for decoding
        let input_for_thread = text.to_string();
        let config_for_thread = config.clone();
        thread::spawn(move || {
            // Set the global config for the worker thread
            crate::config::set_global_config(config_for_thread.clone());

            // Perform the cracking with cache_id support for TUI branching
            let result =
                crate::perform_cracking_with_cache_id(&input_for_thread, config_for_thread);

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
    initial_result_receiver: Option<mpsc::Receiver<CrackingResult>>,
    initial_confirmation_receiver: Option<Receiver<TuiConfirmationRequest>>,
) -> TuiResult<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();
    let mut start_time = Instant::now();
    let mut tick_count: usize = 0;

    // Use Option so we can replace the receivers when rerunning
    let mut result_receiver = initial_result_receiver;
    let mut confirmation_receiver = initial_confirmation_receiver;

    // Branch context for linking RunBranchFullSearch results after the background thread returns.
    // Set when RunBranchFullSearch fires, consumed when the result arrives.
    let mut pending_branch_context: Option<super::app::BranchContext> = None;

    // Channel for receiving AI explanation results from background thread
    let mut ai_result_receiver: Option<mpsc::Receiver<Result<String, crate::ai::error::AiError>>> =
        None;

    loop {
        // Draw the UI
        let completed_frame = terminal.draw(|frame| draw(frame, app, colors))?;

        // Update level_visible_rows based on actual terminal size
        // Layout: main area = terminal height - 1 (status bar)
        // Right panel = 62% of main area width, split 55%/45% vertically
        // Level detail panel = 45% of right panel height - 2 (borders)
        // Branch list = level detail inner - 1 (header)
        if let super::app::AppState::Results {
            level_visible_rows, ..
        } = &mut app.state
        {
            let term_height = completed_frame.area.height;
            let main_height = term_height.saturating_sub(1); // minus status bar
            let right_panel_height = main_height; // same as main area
            let level_panel_height = (right_panel_height as f32 * 0.45) as u16;
            let level_inner = level_panel_height.saturating_sub(2); // borders
            let branch_rows = level_inner.saturating_sub(1); // header line
            *level_visible_rows = (branch_rows as usize).max(1);
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
                                current_quote: random_quote_index(),
                                spinner_frame: 0,
                            };
                            app.clear_status();

                            // Reset timing
                            start_time = Instant::now();
                            tick_count = 0;

                            // Initialize confirmation channel for the decode thread
                            confirmation_receiver = reinit_tui_confirmation_channel();

                            // Spawn decode thread with cache_id support for branching
                            let (tx, rx) = mpsc::channel::<CrackingResult>();
                            result_receiver = Some(rx);

                            let config_clone = config.clone();
                            thread::spawn(move || {
                                crate::config::set_global_config(config_clone.clone());
                                let result =
                                    crate::perform_cracking_with_cache_id(&new_input, config_clone);
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
                                current_quote: random_quote_index(),
                                spinner_frame: 0,
                            };
                            app.clear_status();

                            // Reset timing
                            start_time = Instant::now();
                            tick_count = 0;

                            // Reinitialize the human checker confirmation channel for the new run
                            // This creates a fresh channel so the new decode thread can communicate
                            confirmation_receiver = reinit_tui_confirmation_channel();

                            // Create new channel and spawn new decode thread with cache_id support
                            let (tx, rx) = mpsc::channel::<CrackingResult>();
                            result_receiver = Some(rx);

                            let config_clone = config.clone();
                            thread::spawn(move || {
                                crate::config::set_global_config(config_clone.clone());
                                let result =
                                    crate::perform_cracking_with_cache_id(&new_input, config_clone);
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
                        Action::SwitchToBranch(cache_id) => {
                            // Extract current cache_id and selected_step from Results state
                            // before we modify it
                            let current_context = if let super::app::AppState::Results {
                                cache_id: current_cache_id,
                                selected_step,
                                branch_path,
                                ..
                            } = &app.state
                            {
                                Some((
                                    current_cache_id.clone(),
                                    *selected_step,
                                    branch_path.clone(),
                                ))
                            } else {
                                None
                            };

                            // Load the branch's cached result from the database
                            match get_cache_by_id(cache_id) {
                                Ok(Some(cache_row)) => {
                                    // Parse the path JSON strings back into CrackResults
                                    let crack_results: Vec<CrackResult> = cache_row
                                        .path
                                        .iter()
                                        .filter_map(|json_str| serde_json::from_str(json_str).ok())
                                        .collect();

                                    // Reconstruct the DecoderResult
                                    let result = DecoderResult {
                                        text: vec![cache_row.decoded_text.clone()],
                                        path: crack_results,
                                    };

                                    // Update input_text for display purposes
                                    app.input_text = cache_row.encoded_text;

                                    // Set the result state with the branch's cache_id
                                    app.set_result_with_cache_id(result, cache_id);

                                    // Now push the previous (cache_id, step) onto branch_path
                                    if let (
                                        Some((
                                            Some(prev_cache_id),
                                            prev_step,
                                            mut prev_branch_path,
                                        )),
                                        super::app::AppState::Results { branch_path, .. },
                                    ) = (current_context, &mut app.state)
                                    {
                                        // Push the previous location onto the branch path
                                        prev_branch_path.push(prev_cache_id, prev_step);
                                        *branch_path = prev_branch_path;
                                    }

                                    app.set_status(format!(
                                        "Switched to branch (cache_id: {})",
                                        cache_id
                                    ));
                                }
                                Ok(None) => {
                                    app.set_status(format!(
                                        "Branch not found (cache_id: {})",
                                        cache_id
                                    ));
                                }
                                Err(e) => {
                                    app.set_status(format!("Error loading branch: {}", e));
                                }
                            }
                        }
                        Action::OpenBranchPrompt => {
                            app.open_branch_prompt();
                        }
                        Action::OpenDecoderSearch => {
                            app.open_decoder_search();
                        }
                        Action::OpenQuickSearch => {
                            app.open_quick_search(config);
                        }
                        Action::LaunchQuickSearch(url) => match open::that(&url) {
                            Ok(()) => {
                                app.set_status("Opened in browser.".to_string());
                            }
                            Err(e) => {
                                app.set_status(format!("Failed to open browser: {}", e));
                            }
                        },
                        Action::ReturnToParent => {
                            // Return to parent branch in the branch hierarchy
                            if let super::app::AppState::Results { branch_path, .. } =
                                &mut app.state
                            {
                                if !branch_path.is_branch() {
                                    app.set_status("Already at main path.".to_string());
                                } else if let Some((parent_cache_id, parent_step)) =
                                    branch_path.pop()
                                {
                                    // Clone the remaining branch path before we replace app.state
                                    let remaining_branch_path = branch_path.clone();

                                    // Load the parent result from the database
                                    match get_cache_by_id(parent_cache_id) {
                                        Ok(Some(cache_row)) => {
                                            // Reconstruct CrackResults from the path JSON
                                            let crack_results: Vec<CrackResult> = cache_row
                                                .path
                                                .iter()
                                                .filter_map(|json_str| {
                                                    serde_json::from_str(json_str).ok()
                                                })
                                                .collect();

                                            let result = DecoderResult {
                                                text: vec![cache_row.decoded_text.clone()],
                                                path: crack_results,
                                            };

                                            // Update the app input_text for display
                                            app.input_text = cache_row.encoded_text.clone();

                                            // Load branches for the parent step
                                            let branches =
                                                crate::storage::database::get_branches_for_step(
                                                    parent_cache_id,
                                                    parent_step,
                                                )
                                                .unwrap_or_default();

                                            // Update the app state with the parent result
                                            app.state = super::app::AppState::Results {
                                                result,
                                                selected_step: parent_step,
                                                scroll_offset: 0,
                                                cache_id: Some(parent_cache_id),
                                                branch_path: remaining_branch_path,
                                                current_branches: branches,
                                                highlighted_branch: None,
                                                branch_scroll_offset: 0,
                                                focus: super::app::ResultsFocus::default(),
                                                tree_branches:
                                                    super::app::App::load_tree_branches_static(
                                                        parent_cache_id,
                                                    ),
                                                level_visible_rows: 10,
                                                ai_explanation: None,
                                                ai_loading: false,
                                            };

                                            app.set_status(
                                                "Returned to parent branch.".to_string(),
                                            );
                                        }
                                        Ok(None) => {
                                            app.set_status(format!(
                                                "Parent branch (ID {}) not found in database.",
                                                parent_cache_id
                                            ));
                                        }
                                        Err(e) => {
                                            app.set_status(format!(
                                                "Error loading parent branch: {}",
                                                e
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        Action::RunBranchFullSearch(text, branch_context) => {
                            // Run a full A* search as a branch
                            // Store branch context for post-hoc linking when the result arrives
                            pending_branch_context = branch_context;

                            // Reset state for a fresh run
                            crate::checkers::reset_human_checker_state();
                            crate::timer::resume();
                            crate::storage::wait_athena_storage::clear_plaintext_results();

                            // Transition to loading state
                            app.state = super::app::AppState::Loading {
                                start_time: Instant::now(),
                                current_quote: random_quote_index(),
                                spinner_frame: 0,
                            };
                            app.clear_status();

                            // Reset timing
                            start_time = Instant::now();
                            tick_count = 0;

                            // Reinitialize confirmation channel
                            confirmation_receiver = reinit_tui_confirmation_channel();

                            // Spawn decode thread with cache_id support for branching
                            let (tx, rx) = mpsc::channel::<CrackingResult>();
                            result_receiver = Some(rx);

                            let config_clone = config.clone();
                            thread::spawn(move || {
                                crate::config::set_global_config(config_clone.clone());
                                let result =
                                    crate::perform_cracking_with_cache_id(&text, config_clone);
                                let _ = tx.send(result);
                            });

                            app.set_status("Running full search on branch...".to_string());
                        }
                        Action::RunBranchSingleLayer(text) => {
                            // Run single layer decoding (all decoders once)
                            // This is a quick exploration that doesn't recurse

                            // Get branch context from the BranchModePrompt state before transitioning
                            let branch_context = if let super::app::AppState::BranchModePrompt {
                                branch_context,
                                ..
                            } = &app.state
                            {
                                Some(branch_context.clone())
                            } else {
                                // Fallback: try to get from Results state
                                app.get_branch_context()
                            };

                            // Restore to Results state by getting context and going back
                            // We need to restore the Results state since we're coming from BranchModePrompt
                            if let Some(context) = &branch_context {
                                if let Some(parent_id) = context.parent_cache_id {
                                    // Restore the Results state from the parent cache entry
                                    if let Ok(Some(cache_row)) = get_cache_by_id(parent_id) {
                                        let crack_results: Vec<CrackResult> = cache_row
                                            .path
                                            .iter()
                                            .filter_map(|json_str| {
                                                serde_json::from_str(json_str).ok()
                                            })
                                            .collect();

                                        let result = DecoderResult {
                                            text: vec![cache_row.decoded_text.clone()],
                                            path: crack_results,
                                        };

                                        app.set_result_with_cache_id(result, parent_id);
                                    }
                                }
                            }

                            let config_clone = config.clone();
                            crate::config::set_global_config(config_clone.clone());

                            // Create checker for single layer run
                            let checker = crate::checkers::CheckerTypes::CheckAthena(
                                crate::checkers::checker_type::Checker::<
                                    crate::checkers::athena::Athena,
                                >::new(),
                            );

                            let results = crate::run_single_layer(&text, &checker);

                            if results.is_empty() {
                                app.set_status(
                                    "No decoders produced output for this text.".to_string(),
                                );
                            } else {
                                // Store results as branches in the database
                                let mut branches_created = 0;

                                if let Some(context) = branch_context {
                                    if let Some(parent_cache_id) = context.parent_cache_id {
                                        for result in &results {
                                            // Only store results that have unencrypted_text
                                            if let Some(outputs) = &result.unencrypted_text {
                                                if let Some(decoded) = outputs.first() {
                                                    let cache_entry = CacheEntry {
                                                        encoded_text: text.clone(),
                                                        decoded_text: decoded.clone(),
                                                        path: vec![result.clone()],
                                                        execution_time_ms: 0,
                                                        input_length: text.len() as i64,
                                                        decoder_count: 1,
                                                        checker_name: None,
                                                        key_used: result.key.clone(),
                                                    };

                                                    if insert_branch(
                                                        &cache_entry,
                                                        parent_cache_id,
                                                        context.branch_step,
                                                        &BranchType::SingleLayer,
                                                    )
                                                    .is_ok()
                                                    {
                                                        branches_created += 1;
                                                    }
                                                }
                                            }
                                        }

                                        // Refresh the branch list for the current step
                                        app.load_branches_for_step();
                                        // Refresh tree data for birds-eye view
                                        app.refresh_tree_branches();

                                        app.set_status(format!(
                                            "Created {} branches from single-layer decoding.",
                                            branches_created
                                        ));
                                    } else {
                                        app.set_status(
                                            "Cannot create branches: no parent cache ID."
                                                .to_string(),
                                        );
                                    }
                                } else {
                                    app.set_status(format!(
                                        "Found {} decoder outputs but no branch context.",
                                        results.len()
                                    ));
                                }
                            }
                        }
                        Action::RunBranchDecoder(text, decoder_name) => {
                            // Run a specific decoder on the text
                            let config_clone = config.clone();
                            crate::config::set_global_config(config_clone.clone());

                            // Get branch context before running the decoder
                            let branch_ctx = app.get_branch_context();

                            // Create checker
                            let checker = crate::checkers::CheckerTypes::CheckAthena(
                                crate::checkers::checker_type::Checker::<
                                    crate::checkers::athena::Athena,
                                >::new(),
                            );

                            if let Some(result) =
                                crate::run_specific_decoder(&text, &decoder_name, &checker)
                            {
                                if let Some(outputs) = &result.unencrypted_text {
                                    if let Some(first_output) = outputs.first() {
                                        // Try to store as a branch if we have context
                                        let mut saved_branch_id: Option<i64> = None;

                                        if let Some(ref ctx) = branch_ctx {
                                            if let Some(parent_id) = ctx.parent_cache_id {
                                                // Create cache entry for the branch
                                                let cache_entry = CacheEntry {
                                                    encoded_text: text.clone(),
                                                    decoded_text: first_output.clone(),
                                                    path: vec![result.clone()],
                                                    execution_time_ms: 0,
                                                    input_length: text.len() as i64,
                                                    decoder_count: 1,
                                                    checker_name: None,
                                                    key_used: result.key.clone(),
                                                };

                                                // Insert branch into database
                                                if let Ok(new_id) = insert_branch(
                                                    &cache_entry,
                                                    parent_id,
                                                    ctx.branch_step,
                                                    &BranchType::Manual,
                                                ) {
                                                    saved_branch_id = Some(new_id);
                                                    // Refresh the branch list for current step
                                                    app.load_branches_for_step();
                                                    // Refresh tree data for birds-eye view
                                                    app.refresh_tree_branches();
                                                }
                                            }
                                        }

                                        // Set status message
                                        let preview = if first_output.len() > 50 {
                                            format!(
                                                "{}...",
                                                first_output.chars().take(50).collect::<String>()
                                            )
                                        } else {
                                            first_output.clone()
                                        };

                                        if let Some(branch_id) = saved_branch_id {
                                            app.set_status(format!(
                                                "{} decoded to: {} (saved as branch #{})",
                                                decoder_name, preview, branch_id
                                            ));
                                        } else {
                                            app.set_status(format!(
                                                "{} decoded to: {}",
                                                decoder_name, preview
                                            ));
                                        }
                                    }
                                }
                            } else {
                                app.set_status(format!(
                                    "{} could not decode this text.",
                                    decoder_name
                                ));
                            }
                        }
                        Action::ExplainStep {
                            decoder_name,
                            input_text,
                            output_text,
                            key,
                        } => {
                            // Spawn background thread for AI explanation
                            let (ai_tx, ai_rx) = mpsc::channel();
                            ai_result_receiver = Some(ai_rx);

                            thread::spawn(move || {
                                let result = crate::ai::explain_step(
                                    &decoder_name,
                                    &input_text,
                                    &output_text,
                                    key.as_deref(),
                                );
                                let _ = ai_tx.send(result);
                            });

                            app.set_status("Loading AI explanation...".to_string());
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

            // Clear status message after configured timeout (0 = never auto-clear)
            if config.status_message_timeout > 0 {
                let status_clear_ticks =
                    (config.status_message_timeout * 1000 / TICK_RATE_MS) as usize;
                if status_clear_ticks > 0 && tick_count % status_clear_ticks == 0 {
                    app.clear_status();
                }
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

        // Check for AI explanation result (non-blocking)
        if let Some(ref ai_rx) = ai_result_receiver {
            if let Ok(ai_result) = ai_rx.try_recv() {
                match ai_result {
                    Ok(explanation) => {
                        app.set_ai_explanation(explanation);
                        app.set_status("AI explanation loaded.".to_string());
                    }
                    Err(e) => {
                        // Clear loading state on error
                        app.clear_ai_explanation();
                        app.set_status(format!("AI explanation failed: {}", e));
                    }
                }
                ai_result_receiver = None;
            }
        }

        // Check for decode result (non-blocking)
        if let Some(ref rx) = result_receiver {
            if let Ok(cracking_result) = rx.try_recv() {
                match cracking_result.result {
                    Some(decoder_result) => {
                        // Use set_result_with_cache_id if we have a cache_id for branching support
                        if let Some(cache_id) = cracking_result.cache_id {
                            // If this result came from a RunBranchFullSearch, link it as a branch
                            if let Some(ref ctx) = pending_branch_context {
                                if let Some(parent_id) = ctx.parent_cache_id {
                                    if let Err(e) = link_as_branch(
                                        cache_id,
                                        parent_id,
                                        ctx.branch_step,
                                        &BranchType::Auto,
                                    ) {
                                        app.set_status(format!(
                                            "Warning: failed to link branch: {}",
                                            e
                                        ));
                                    }
                                }
                            }
                            pending_branch_context = None;

                            app.set_result_with_cache_id(decoder_result, cache_id);

                            // Refresh branches so the new branch appears in the UI
                            app.load_branches_for_step();
                            app.refresh_tree_branches();
                        } else {
                            pending_branch_context = None;
                            app.set_result(decoder_result);
                        }
                    }
                    None => {
                        pending_branch_context = None;
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
}
