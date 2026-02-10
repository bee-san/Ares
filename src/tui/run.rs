//! Main TUI entry point and event loop for Ciphey.
//!
//! This module provides the main `run_tui` function that initializes the terminal,
//! runs the decoding in a background thread, and handles the event loop.
//!
//! ## Architecture
//!
//! Background work (decoding, AI calls, single-layer exploration) communicates
//! with the event loop through a single unified [`BackgroundMessage`] channel,
//! eliminating the hand-rolled multiplexer that previously polled 5 separate
//! receivers in sequence. The only exception is the human-checker confirmation
//! channel, which originates in [`super::human_checker_bridge`] and is polled
//! separately.
//!
//! Duplicated logic has been extracted into helpers:
//! - [`reset_for_fresh_run`] — clears global state before spawning a decode thread
//! - [`spawn_decode_thread`] — spawns a background A* decode
//! - [`reconstruct_result_from_cache`] — loads a `DecoderResult` from the DB
//! - [`start_fresh_decode`] — combines the above for `SubmitHomeInput` / `Rerun` / `RunBranchFullSearch`

use std::io::{self, Stdout};
use std::sync::mpsc::{self, Receiver, Sender};
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
use super::input::{copy_to_clipboard, handle_key_event, Action, ExplainStepData, ShowHistoryData};
use super::spinner::random_quote_index;
use super::ui::draw;

/// Result type for TUI operations.
type TuiResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Tick rate for UI updates (in milliseconds).
const TICK_RATE_MS: u64 = 100;

// =============================================================================
// BackgroundMessage — unified channel for all background thread results
// =============================================================================

/// Messages sent from background threads to the TUI event loop.
///
/// All background work (decoding, AI, single-layer) sends its results through
/// a single `mpsc::Sender<BackgroundMessage>` instead of separate channels,
/// eliminating the hand-rolled multiplexer that previously polled 4+ receivers.
enum BackgroundMessage {
    /// Full A* decode result from `perform_cracking_with_cache_id`.
    DecodeResult(CrackingResult),
    /// AI step-explanation result.
    AiExplanation(Result<String, crate::ai::error::AiError>),
    /// Ask-AI response.
    AskAiResponse(Result<String, crate::ai::error::AiError>),
    /// Single-layer decode results with text and branch context.
    SingleLayerResult(
        Vec<CrackResult>,
        String,
        Option<super::app::BranchContext>,
    ),
}

// =============================================================================
// Extracted helper functions
// =============================================================================

/// Resets global state for a fresh decode run.
///
/// Clears human checker state, resumes timer, and clears cached plaintext
/// results. This must be called before spawning any new decode thread.
fn reset_for_fresh_run() {
    crate::checkers::reset_human_checker_state();
    crate::timer::resume();
    crate::storage::wait_athena_storage::clear_plaintext_results();
}

/// Spawns a background decode thread that sends its result as a
/// [`BackgroundMessage::DecodeResult`] on `bg_tx`.
///
/// # Arguments
///
/// * `text` - The text to decode
/// * `config` - The Ciphey configuration (cloned into the thread)
/// * `bg_tx` - Sender for the unified background message channel
fn spawn_decode_thread(text: String, config: &Config, bg_tx: &Sender<BackgroundMessage>) {
    let config_clone = config.clone();
    let tx = bg_tx.clone();
    thread::spawn(move || {
        crate::config::set_global_config(config_clone.clone());
        let result = crate::perform_cracking_with_cache_id(&text, config_clone);
        let _ = tx.send(BackgroundMessage::DecodeResult(result));
    });
}

/// Reconstructs a [`DecoderResult`] from the database cache by `cache_id`.
///
/// Returns `(DecoderResult, encoded_text)` on success, or an error string.
fn reconstruct_result_from_cache(cache_id: i64) -> Result<(DecoderResult, String), String> {
    match get_cache_by_id(cache_id) {
        Ok(Some(cache_row)) => {
            let crack_results: Vec<CrackResult> = cache_row
                .path
                .iter()
                .filter_map(|json_str| serde_json::from_str(json_str).ok())
                .collect();
            let result = DecoderResult {
                text: vec![cache_row.decoded_text.clone()],
                path: crack_results,
            };
            Ok((result, cache_row.encoded_text))
        }
        Ok(None) => Err(format!("Cache entry not found (id: {})", cache_id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

/// Reconstructs a [`DecoderResult`] from a pre-parsed JSON path and decoded text.
///
/// Used when displaying history entries that already have their path available.
fn reconstruct_result_from_path(path_json: &[String], decoded_text: String) -> DecoderResult {
    let crack_results: Vec<CrackResult> = path_json
        .iter()
        .filter_map(|json_str| serde_json::from_str(json_str).ok())
        .collect();
    DecoderResult {
        text: vec![decoded_text],
        path: crack_results,
    }
}

/// Prepares the app for a new loading state and resets timing counters.
///
/// Returns a fresh `Instant` for `start_time` tracking.
fn transition_to_loading(app: &mut App) -> Instant {
    app.state = super::app::AppState::Loading(super::app::LoadingState {
        start_time: Instant::now(),
        current_quote: random_quote_index(),
        spinner_frame: 0,
    });
    app.clear_status();
    Instant::now()
}

// =============================================================================
// Public entry point
// =============================================================================

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

    // Unified background message channel
    let (bg_tx, bg_rx) = mpsc::channel::<BackgroundMessage>();

    // Only spawn decode thread if we have input text
    if let Some(text) = input_text {
        spawn_decode_thread(text.to_string(), &config, &bg_tx);
    }

    // Run the main loop
    let result = run_event_loop(
        &mut terminal,
        &mut app,
        &colors,
        &config,
        bg_tx,
        bg_rx,
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

// =============================================================================
// Event loop
// =============================================================================

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
    bg_tx: Sender<BackgroundMessage>,
    bg_rx: Receiver<BackgroundMessage>,
    initial_confirmation_receiver: Option<Receiver<TuiConfirmationRequest>>,
) -> TuiResult<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();
    let mut start_time = Instant::now();
    let mut tick_count: usize = 0;

    // The confirmation receiver comes from the human checker bridge and must be
    // polled separately because it is created in a different module.
    let mut confirmation_receiver = initial_confirmation_receiver;

    // Branch context for linking RunBranchFullSearch results after the background
    // thread returns. Set when RunBranchFullSearch fires, consumed on result arrival.
    let mut pending_branch_context: Option<super::app::BranchContext> = None;

    loop {
        // Draw the UI
        let completed_frame = terminal.draw(|frame| draw(frame, app, colors))?;

        // Update level_visible_rows based on actual terminal size
        if let super::app::AppState::Results(ref mut rs) = app.state {
            let level_visible_rows = &mut rs.level_visible_rows;
            let term_height = completed_frame.area.height;
            let main_height = term_height.saturating_sub(1);
            let right_panel_height = main_height;
            let level_panel_height = (right_panel_height as f32 * 0.45) as u16;
            let level_inner = level_panel_height.saturating_sub(2);
            let branch_rows = level_inner.saturating_sub(1);
            *level_visible_rows = (branch_rows as usize).max(1);
        }

        // Calculate timeout for event polling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        // Poll for keyboard / terminal events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = handle_key_event(app, key);
                    handle_action(
                        app,
                        action,
                        config,
                        &bg_tx,
                        &mut confirmation_receiver,
                        &mut pending_branch_context,
                        &mut start_time,
                        &mut tick_count,
                    );
                }
            }
        }

        // Tick — animations, status message timeout
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            tick_count += 1;

            if config.status_message_timeout > 0 {
                let status_clear_ticks =
                    (config.status_message_timeout * 1000 / TICK_RATE_MS) as usize;
                if status_clear_ticks > 0 && tick_count.is_multiple_of(status_clear_ticks) {
                    app.clear_status();
                }
            }

            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }

        // Poll human checker confirmation requests (separate bridge channel)
        if let Some(ref conf_rx) = confirmation_receiver {
            if let Ok(conf_request) = conf_rx.try_recv() {
                app.set_human_confirmation(conf_request.request, conf_request.response_tx);
            }
        }

        // Poll unified background message channel (non-blocking, drain all ready)
        while let Ok(msg) = bg_rx.try_recv() {
            handle_background_message(app, msg, &mut pending_branch_context, start_time);
        }
    }

    Ok(())
}

// =============================================================================
// Action handling (extracted from the giant match)
// =============================================================================

/// Dispatches a single [`Action`] returned by the input handler.
///
/// This function replaces the enormous inline `match action { … }` that
/// previously lived inside the event loop body.
#[allow(clippy::too_many_arguments)]
fn handle_action(
    app: &mut App,
    action: Action,
    config: &Config,
    bg_tx: &Sender<BackgroundMessage>,
    confirmation_receiver: &mut Option<Receiver<TuiConfirmationRequest>>,
    pending_branch_context: &mut Option<super::app::BranchContext>,
    start_time: &mut Instant,
    tick_count: &mut usize,
) {
    match action {
        Action::None => {}

        // ----- Clipboard -----
        Action::CopyToClipboard(text) => match copy_to_clipboard(&text) {
            Ok(()) => app.set_status("Copied to clipboard!".to_string()),
            Err(e) => app.set_status(format!("Copy failed: {}", e)),
        },

        // ----- Start a fresh decode from the home screen -----
        Action::SubmitHomeInput(new_input) => {
            start_fresh_decode(
                app,
                &new_input,
                config,
                bg_tx,
                confirmation_receiver,
                start_time,
                tick_count,
            );
            app.set_status("Decoding...".to_string());
        }

        // ----- Rerun from a selected intermediate step -----
        Action::RerunFromSelected(new_input) => {
            start_fresh_decode(
                app,
                &new_input,
                config,
                bg_tx,
                confirmation_receiver,
                start_time,
                tick_count,
            );
            app.set_status("Rerunning Ciphey...".to_string());
        }

        // ----- Settings -----
        Action::OpenSettings => {
            app.open_settings(config);
            app.set_status("Settings opened. Press Esc to close.".to_string());
        }
        Action::SaveSettings => {
            if let super::app::AppState::Settings(ref ss) = app.state {
                let mut new_config = config.clone();
                ss.settings.apply_to_config(&mut new_config);
                crate::config::set_global_config(new_config.clone());
                if let Err(e) = crate::config::save_config(&new_config) {
                    app.set_status(format!("Error saving settings: {}", e));
                } else {
                    app.set_status("Settings saved!".to_string());
                }
            }
            app.close_settings();
        }

        // ----- Show a cached history entry -----
        Action::ShowHistoryResult(ShowHistoryData {
            cache_id,
            encoded_text,
            decoded_text,
            path,
        }) => {
            let result = reconstruct_result_from_path(&path, decoded_text);
            app.input_text = encoded_text;
            app.set_result_with_cache_id(result, cache_id);
            app.set_status("Showing saved result from history.".to_string());
        }

        // ----- Branch: switch to an existing branch -----
        Action::SwitchToBranch(cache_id) => {
            handle_switch_to_branch(app, cache_id);
        }

        // ----- Branch: prompts & search -----
        Action::OpenBranchPrompt => app.open_branch_prompt(),
        Action::OpenDecoderSearch => app.open_decoder_search(),
        Action::OpenQuickSearch => app.open_quick_search(config),
        Action::LaunchQuickSearch(url) => match open::that(&url) {
            Ok(()) => app.set_status("Opened in browser.".to_string()),
            Err(e) => app.set_status(format!("Failed to open browser: {}", e)),
        },

        // ----- Branch: return to parent -----
        Action::ReturnToParent => {
            handle_return_to_parent(app);
        }

        // ----- Branch: full A* search -----
        Action::RunBranchFullSearch(text, branch_context) => {
            *pending_branch_context = branch_context;
            start_fresh_decode(
                app,
                &text,
                config,
                bg_tx,
                confirmation_receiver,
                start_time,
                tick_count,
            );
            app.set_status("Running full search on branch...".to_string());
        }

        // ----- Branch: single-layer exploration -----
        Action::RunBranchSingleLayer(text) => {
            handle_run_branch_single_layer(app, text, config, bg_tx);
        }

        // ----- Branch: run a specific decoder -----
        Action::RunBranchDecoder(text, decoder_name) => {
            handle_run_branch_decoder(app, text, decoder_name, config);
        }

        // ----- AI: explain step -----
        Action::ExplainStep(ExplainStepData {
            decoder_name,
            input_text,
            output_text,
            key,
        }) => {
            let tx = bg_tx.clone();
            thread::spawn(move || {
                let result = crate::ai::explain_step(
                    &decoder_name,
                    &input_text,
                    &output_text,
                    key.as_deref(),
                );
                let _ = tx.send(BackgroundMessage::AiExplanation(result));
            });
            app.set_status("Loading AI explanation...".to_string());
        }

        // ----- AI: ask a question -----
        Action::OpenAskAi => app.open_ask_ai(),
        Action::SubmitAskAi(question) => {
            app.set_ask_ai_loading();
            if let Some(ref overlay) = app.ask_ai {
                let decoder_name = overlay.decoder_name.clone();
                let step_input = overlay.step_input.clone();
                let step_output = overlay.step_output.clone();
                let step_key = overlay.step_key.clone();
                let step_description = overlay.step_description.clone();
                let step_link = overlay.step_link.clone();

                let tx = bg_tx.clone();
                thread::spawn(move || {
                    let result = crate::ai::ask_about_step(
                        &question,
                        &decoder_name,
                        &step_input,
                        &step_output,
                        step_key.as_deref(),
                        &step_description,
                        &step_link,
                    );
                    let _ = tx.send(BackgroundMessage::AskAiResponse(result));
                });
            }
            app.set_status("AI is thinking...".to_string());
        }
        Action::CloseAskAi => app.close_ask_ai(),

        // ----- Wordlist I/O (moved out of input handler) -----
        Action::ImportWordlist(path) => {
            handle_import_wordlist(app, &path);
        }
        Action::DeleteWordlist(file_id) => {
            handle_delete_wordlist(app, file_id);
        }
    }
}

// =============================================================================
// Action sub-handlers (keep handle_action lean)
// =============================================================================

/// Shared logic for `SubmitHomeInput`, `RerunFromSelected`, and `RunBranchFullSearch`.
///
/// Resets global state, transitions to Loading, reinitialises the human-checker
/// channel, and spawns a new decode thread.
fn start_fresh_decode(
    app: &mut App,
    text: &str,
    config: &Config,
    bg_tx: &Sender<BackgroundMessage>,
    confirmation_receiver: &mut Option<Receiver<TuiConfirmationRequest>>,
    start_time: &mut Instant,
    tick_count: &mut usize,
) {
    reset_for_fresh_run();
    app.input_text = text.to_string();
    *start_time = transition_to_loading(app);
    *tick_count = 0;
    *confirmation_receiver = reinit_tui_confirmation_channel();
    spawn_decode_thread(text.to_string(), config, bg_tx);
}

/// Handles `Action::SwitchToBranch` — loads a branch from the DB and updates
/// the branch path stack.
fn handle_switch_to_branch(app: &mut App, cache_id: i64) {
    // Capture current context before mutation
    let current_context = if let super::app::AppState::Results(ref rs) = app.state {
        Some((rs.cache_id, rs.selected_step, rs.branch_path.clone()))
    } else {
        None
    };

    match reconstruct_result_from_cache(cache_id) {
        Ok((result, encoded_text)) => {
            app.input_text = encoded_text;
            app.set_result_with_cache_id(result, cache_id);

            // Push previous (cache_id, step) onto branch_path
            if let (
                Some((Some(prev_cache_id), prev_step, mut prev_branch_path)),
                super::app::AppState::Results(ref mut rs),
            ) = (current_context, &mut app.state)
            {
                prev_branch_path.push(prev_cache_id, prev_step);
                rs.branch_path = prev_branch_path;
            }

            app.set_status(format!("Switched to branch (cache_id: {})", cache_id));
        }
        Err(msg) => app.set_status(msg),
    }
}

/// Handles `Action::ReturnToParent` — pops the branch path stack and restores
/// the parent result.
fn handle_return_to_parent(app: &mut App) {
    if let super::app::AppState::Results(ref mut rs) = app.state {
        if !rs.branch_path.is_branch() {
            app.set_status("Already at main path.".to_string());
            return;
        }

        if let Some((parent_cache_id, parent_step)) = rs.branch_path.pop() {
            let remaining_branch_path = rs.branch_path.clone();

            match reconstruct_result_from_cache(parent_cache_id) {
                Ok((result, encoded_text)) => {
                    app.input_text = encoded_text;

                    let branches = crate::storage::database::get_branches_for_step(
                        parent_cache_id,
                        parent_step,
                    )
                    .unwrap_or_default();

                    let mut new_rs =
                        super::app::ResultsState::new_with_cache_id(result, parent_cache_id);
                    new_rs.selected_step = parent_step;
                    new_rs.branch_path = remaining_branch_path;
                    new_rs.current_branches = branches;
                    app.state = super::app::AppState::Results(new_rs);

                    app.set_status("Returned to parent branch.".to_string());
                }
                Err(msg) => app.set_status(msg),
            }
        }
    }
}

/// Handles `Action::RunBranchSingleLayer` — spawns a background thread to run
/// all decoders once on the given text.
fn handle_run_branch_single_layer(
    app: &mut App,
    text: String,
    config: &Config,
    bg_tx: &Sender<BackgroundMessage>,
) {
    // Get branch context from BranchModePrompt or Results state
    let branch_context = if let super::app::AppState::BranchModePrompt(ref bmp) = app.state {
        Some(bmp.branch_context.clone())
    } else {
        app.get_branch_context()
    };

    // Restore to Results state so user sees the parent while waiting
    if let Some(ref context) = branch_context {
        if let Some(parent_id) = context.parent_cache_id {
            if let Ok((result, _encoded_text)) = reconstruct_result_from_cache(parent_id) {
                app.set_result_with_cache_id(result, parent_id);
            }
        }
    }

    app.set_status("Running single-layer decoders...".to_string());

    let tx = bg_tx.clone();
    let config_clone = config.clone();
    let text_clone = text.clone();
    let branch_context_clone = branch_context;
    thread::spawn(move || {
        crate::config::set_global_config(config_clone);

        let checker = crate::checkers::CheckerTypes::CheckAthena(
            crate::checkers::checker_type::Checker::<crate::checkers::athena::Athena>::new(),
        );

        let results = crate::run_single_layer(&text_clone, &checker);
        let _ = tx.send(BackgroundMessage::SingleLayerResult(
            results,
            text_clone,
            branch_context_clone,
        ));
    });
}

/// Handles `Action::RunBranchDecoder` — runs a single named decoder inline
/// and stores the result as a branch.
fn handle_run_branch_decoder(app: &mut App, text: String, decoder_name: String, config: &Config) {
    let config_clone = config.clone();
    crate::config::set_global_config(config_clone);

    let branch_ctx = app.get_branch_context();

    let checker = crate::checkers::CheckerTypes::CheckAthena(
        crate::checkers::checker_type::Checker::<crate::checkers::athena::Athena>::new(),
    );

    if let Some(result) = crate::run_specific_decoder(&text, &decoder_name, &checker) {
        if let Some(outputs) = &result.unencrypted_text {
            if let Some(first_output) = outputs.first() {
                let mut saved_branch_id: Option<i64> = None;

                if let Some(ref ctx) = branch_ctx {
                    if let Some(parent_id) = ctx.parent_cache_id {
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

                        if let Ok(new_id) = insert_branch(
                            &cache_entry,
                            parent_id,
                            ctx.branch_step,
                            &BranchType::Manual,
                        ) {
                            saved_branch_id = Some(new_id);
                            app.load_branches_for_step();
                            app.refresh_tree_branches();
                        }
                    }
                }

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
                    app.set_status(format!("{} decoded to: {}", decoder_name, preview));
                }
            }
        }
    } else {
        app.set_status(format!("{} could not decode this text.", decoder_name));
    }
}

/// Handles `Action::ImportWordlist` — imports a wordlist file from disk,
/// updates the in-memory state, and rebuilds the bloom filter.
fn handle_import_wordlist(app: &mut App, path: &str) {
    use crate::storage::bloom::{build_bloom_filter_from_db, save_bloom_filter};
    use crate::storage::database::import_wordlist_from_file;

    match import_wordlist_from_file(path, "user_import", |_, _| {}) {
        Ok(file_row) => {
            // Add to in-memory display list
            if let super::app::AppState::WordlistManager(ref mut wm) = app.state {
                wm.wordlist_files.push(super::app::state::WordlistFileInfo {
                    id: file_row.id,
                    filename: file_row.filename,
                    file_path: file_row.file_path,
                    source: file_row.source,
                    word_count: file_row.word_count,
                    enabled: file_row.enabled,
                    added_date: file_row.added_date,
                });
            }
            // Rebuild bloom filter after import
            if let Ok(bloom) = build_bloom_filter_from_db() {
                let _ = save_bloom_filter(&bloom);
            }
            app.set_status("Wordlist imported successfully.".to_string());
        }
        Err(e) => {
            log::warn!("Failed to import wordlist from '{}': {}", path, e);
            app.set_status(format!("Import failed: {}", e));
        }
    }
}

/// Handles `Action::DeleteWordlist` — deletes a wordlist file from the database
/// and rebuilds the bloom filter.
fn handle_delete_wordlist(app: &mut App, file_id: i64) {
    use crate::storage::bloom::{build_bloom_filter_from_db, save_bloom_filter};
    use crate::storage::database::delete_wordlist_file;

    if let Err(e) = delete_wordlist_file(file_id) {
        log::warn!("Failed to delete wordlist file {}: {}", file_id, e);
        app.set_status(format!("Delete failed: {}", e));
        return;
    }
    // Rebuild bloom filter after deletion
    if let Ok(bloom) = build_bloom_filter_from_db() {
        let _ = save_bloom_filter(&bloom);
    }
    app.set_status("Wordlist deleted.".to_string());
}

// =============================================================================
// Background message handling
// =============================================================================

/// Processes a single [`BackgroundMessage`] received on the unified channel.
fn handle_background_message(
    app: &mut App,
    msg: BackgroundMessage,
    pending_branch_context: &mut Option<super::app::BranchContext>,
    start_time: Instant,
) {
    match msg {
        BackgroundMessage::DecodeResult(cracking_result) => {
            handle_decode_result(app, cracking_result, pending_branch_context, start_time);
        }
        BackgroundMessage::AiExplanation(ai_result) => match ai_result {
            Ok(explanation) => {
                app.set_ai_explanation(explanation);
                app.set_status("AI explanation loaded.".to_string());
            }
            Err(e) => {
                app.clear_ai_explanation();
                app.set_status(format!("AI explanation failed: {}", e));
            }
        },
        BackgroundMessage::AskAiResponse(ask_result) => match ask_result {
            Ok(response) => {
                app.set_ask_ai_response(response);
                app.clear_status();
            }
            Err(e) => {
                app.set_ask_ai_error(format!("{}", e));
                app.set_status(format!("AI question failed: {}", e));
            }
        },
        BackgroundMessage::SingleLayerResult(results, text, branch_context) => {
            handle_single_layer_result(app, results, text, branch_context);
        }
    }
}

/// Processes the result of a full A* decode thread.
fn handle_decode_result(
    app: &mut App,
    cracking_result: CrackingResult,
    pending_branch_context: &mut Option<super::app::BranchContext>,
    start_time: Instant,
) {
    match cracking_result.result {
        Some(decoder_result) => {
            if let Some(cache_id) = cracking_result.cache_id {
                // If this result came from a RunBranchFullSearch, link it as a branch
                if let Some(ref ctx) = *pending_branch_context {
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
                *pending_branch_context = None;

                app.set_result_with_cache_id(decoder_result, cache_id);

                // Refresh branches so the new branch appears in the UI
                app.load_branches_for_step();
                app.refresh_tree_branches();
            } else {
                *pending_branch_context = None;
                app.set_result(decoder_result);
            }
        }
        None => {
            *pending_branch_context = None;
            let elapsed = start_time.elapsed();
            app.set_failure(elapsed);
        }
    }
}

/// Processes results from a single-layer decode run.
fn handle_single_layer_result(
    app: &mut App,
    results: Vec<CrackResult>,
    text: String,
    branch_context: Option<super::app::BranchContext>,
) {
    if results.is_empty() {
        app.set_status("No decoders produced output for this text.".to_string());
        return;
    }

    let mut branches_created = 0;

    if let Some(context) = branch_context {
        if let Some(parent_cache_id) = context.parent_cache_id {
            for result in &results {
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
            app.set_status("Cannot create branches: no parent cache ID.".to_string());
        }
    } else {
        app.set_status(format!(
            "Found {} decoder outputs but no branch context.",
            results.len()
        ));
    }
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
