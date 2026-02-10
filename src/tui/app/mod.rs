//! Application state machine for the Ciphey TUI.
//!
//! This module defines the core state management for the terminal user interface,
//! handling transitions between loading, results, settings, and failure states.

use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Submodules
pub mod list_editor;
pub mod navigation;
pub mod settings;
pub mod state;
pub mod wordlist;

// Re-export commonly used types
pub use state::{
    AppState, AskAiOverlay, BranchContext, BranchMode, BranchModePromptState, BranchPath,
    DecoderSearchOverlay, FailureState, HelpContext, HistoryEntry, HomeState,
    HumanConfirmationRequest, HumanConfirmationState, ListEditorState, LoadingState, PreviousState,
    QuickSearchOverlay, ResultsFocus, ResultsState, ResultsStateSaved, SaveConfirmationState,
    SettingsState, SettingsStateSnapshot, ThemePickerState, ToggleListEditorState,
    WordlistFileInfo, WordlistManagerFocus, WordlistManagerState,
};

use crate::DecoderResult;

use super::multiline_text_input::MultilineTextInput;
use super::spinner::random_quote_index;

/// Severity level for status messages.
///
/// Controls how long a message persists and how it's styled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusSeverity {
    /// Informational messages  auto-clear on timer.
    Info,
    /// Success messages  auto-clear on timer.
    Success,
    /// Warning messages  persist until any keypress.
    Warning,
    /// Error messages  persist until any keypress.
    Error,
}

/// A status message with severity for display at the bottom of the screen.
#[derive(Debug, Clone)]
pub struct StatusMessage {
    /// The message text.
    pub text: String,
    /// Severity level controlling persistence and styling.
    pub severity: StatusSeverity,
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
    /// Optional status message with severity for user feedback.
    pub status_message: Option<StatusMessage>,
    /// Pending 'g' key for vim-style gg command.
    pub pending_g: bool,
    /// Decoder search overlay (floats over Results screen when Some).
    pub decoder_search: Option<DecoderSearchOverlay>,
    /// Quick search overlay (floats over Results screen when Some).
    pub quick_search: Option<QuickSearchOverlay>,
    /// Ask AI overlay (floats over Results screen when Some).
    pub ask_ai: Option<AskAiOverlay>,
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
            state: AppState::Loading(LoadingState {
                start_time: Instant::now(),
                current_quote: random_quote_index(),
                spinner_frame: 0,
                decoders_tried: Arc::new(AtomicUsize::new(0)),
                cancel_flag: Arc::new(AtomicBool::new(false)),
            }),
            input_text,
            should_quit: false,
            show_help: false,
            status_message: None,
            pending_g: false,
            decoder_search: None,
            quick_search: None,
            ask_ai: None,
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
            state: AppState::Home(HomeState {
                text_input: MultilineTextInput::new(),
                history,
                selected_history: None,
                history_scroll_offset: 0,
            }),
            input_text: String::new(),
            should_quit: false,
            show_help: false,
            status_message: None,
            pending_g: false,
            decoder_search: None,
            quick_search: None,
            ask_ai: None,
        }
    }

    /// Refreshes the history list from the database.
    ///
    /// Call this after returning from a decode attempt to update the history panel.
    pub fn refresh_history(&mut self) {
        if let AppState::Home(ref mut home) = self.state {
            home.history = match crate::storage::database::read_cache_history() {
                Ok(rows) => rows.iter().map(HistoryEntry::from_cache_row).collect(),
                Err(_) => Vec::new(),
            };
            home.selected_history = None;
            home.history_scroll_offset = 0;
        }
    }

    /// Checks if the app is currently in the Home state.
    ///
    /// # Returns
    ///
    /// `true` if in Home state, `false` otherwise.
    pub fn is_home(&self) -> bool {
        matches!(self.state, AppState::Home(_))
    }

    /// Gets the text from the Home state text input.
    ///
    /// # Returns
    ///
    /// The text entered by the user, or an empty string if not in Home state.
    pub fn get_home_input(&self) -> String {
        match &self.state {
            AppState::Home(home) => home.text_input.get_text(),
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
        if let AppState::Home(ref home) = self.state {
            let input = home.text_input.get_text();
            if input.trim().is_empty() {
                return None;
            }

            self.input_text = input.clone();
            self.state = AppState::Loading(LoadingState {
                start_time: Instant::now(),
                current_quote: random_quote_index(),
                spinner_frame: 0,
                decoders_tried: Arc::new(AtomicUsize::new(0)),
                cancel_flag: Arc::new(AtomicBool::new(false)),
            });
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
            AppState::Loading(ref mut loading) => loading.tick(),
            AppState::HumanConfirmation(ref mut hc) => hc.tick(),
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
        self.state = AppState::Results(ResultsState::new(result));
    }

    /// Transitions the application to the Results state with a cache ID.
    ///
    /// This variant is used when showing results from the database (e.g., history)
    /// where we have a cache_id for branch linking.
    ///
    /// # Arguments
    ///
    /// * `result` - The successful decoding result to display
    /// * `cache_id` - The database cache ID for this result
    pub fn set_result_with_cache_id(&mut self, result: DecoderResult, cache_id: i64) {
        self.state = AppState::Results(ResultsState::new_with_cache_id(result, cache_id));
    }

    /// Transitions the application to the Failure state.
    ///
    /// # Arguments
    ///
    /// * `elapsed` - How long the decoding attempt took
    pub fn set_failure(&mut self, elapsed: Duration) {
        self.state = AppState::Failure(FailureState {
            input_text: self.input_text.clone(),
            elapsed,
        });
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
            AppState::Loading(ls) => (ls.start_time, ls.current_quote, ls.spinner_frame),
            AppState::HumanConfirmation(hc) => (hc.start_time, hc.current_quote, hc.spinner_frame),
            _ => (Instant::now(), 0, 0),
        };

        self.state = AppState::HumanConfirmation(HumanConfirmationState {
            start_time,
            current_quote,
            spinner_frame,
            request,
            response_sender,
        });
    }

    /// Sends a response to the human confirmation request and returns to Loading state.
    ///
    /// # Arguments
    ///
    /// * `accepted` - Whether the user accepted the plaintext candidate
    pub fn respond_to_confirmation(&mut self, accepted: bool) {
        if let AppState::HumanConfirmation(ref hc) = self.state {
            // Send the response (ignore error if receiver dropped)
            let _ = hc.response_sender.send(accepted);

            // Return to loading state
            self.state = AppState::Loading(LoadingState {
                start_time: hc.start_time,
                current_quote: hc.current_quote,
                spinner_frame: hc.spinner_frame,
                decoders_tried: Arc::new(AtomicUsize::new(0)),
                cancel_flag: Arc::new(AtomicBool::new(false)),
            });
        }
    }

    /// Toggles the visibility of the help overlay.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Sets a temporary informational status message for user feedback.
    ///
    /// Info messages auto-clear on the status message timer.
    ///
    /// # Arguments
    ///
    /// * `msg` - The status message to display
    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some(StatusMessage {
            text: msg,
            severity: StatusSeverity::Info,
        });
    }

    /// Sets a success status message for user feedback.
    ///
    /// Success messages auto-clear on the status message timer.
    ///
    /// # Arguments
    ///
    /// * `msg` - The status message to display
    pub fn set_status_success(&mut self, msg: String) {
        self.status_message = Some(StatusMessage {
            text: msg,
            severity: StatusSeverity::Success,
        });
    }

    /// Sets an error status message for user feedback.
    ///
    /// Error messages persist until the user presses any key.
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message to display
    pub fn set_status_error(&mut self, msg: String) {
        self.status_message = Some(StatusMessage {
            text: msg,
            severity: StatusSeverity::Error,
        });
    }

    /// Sets a warning status message for user feedback.
    ///
    /// Warning messages persist until the user presses any key.
    ///
    /// # Arguments
    ///
    /// * `msg` - The warning message to display
    pub fn set_status_warning(&mut self, msg: String) {
        self.status_message = Some(StatusMessage {
            text: msg,
            severity: StatusSeverity::Warning,
        });
    }

    /// Clears the current status message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Acknowledges a persistent status message (Error/Warning) so it clears on next tick.
    ///
    /// Called when any key is pressed while an Error/Warning status is shown.
    /// Info/Success messages are not affected (they auto-clear on timer).
    pub fn acknowledge_status(&mut self) {
        if let Some(ref msg) = self.status_message {
            if matches!(
                msg.severity,
                StatusSeverity::Error | StatusSeverity::Warning
            ) {
                self.status_message = None;
            }
        }
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

        self.state = AppState::Home(HomeState {
            text_input: MultilineTextInput::new(),
            history,
            selected_history: None,
            history_scroll_offset: 0,
        });
        self.input_text.clear();
        self.clear_status();
    }

    // ============================================================================
    // Tree View Helper Methods
    // ============================================================================

    /// Loads all branches for the tree view, keyed by step index.
    ///
    /// Queries the database for all branches of the given cache entry and
    /// converts them into `TreeNode` structs grouped by step.
    fn load_tree_branches(
        cache_id: i64,
        _path_len: usize,
    ) -> std::collections::HashMap<usize, Vec<crate::tui::widgets::tree_viewer::TreeNode>> {
        Self::load_tree_branches_static(cache_id)
    }

    /// Static version of `load_tree_branches` that can be called without `&self`.
    ///
    /// This is used by `run.rs` when constructing `AppState::Results` directly.
    pub fn load_tree_branches_static(
        cache_id: i64,
    ) -> std::collections::HashMap<usize, Vec<crate::tui::widgets::tree_viewer::TreeNode>> {
        use crate::storage::database::count_sub_branches;
        use crate::tui::widgets::tree_viewer::TreeNode;

        let mut tree: std::collections::HashMap<usize, Vec<TreeNode>> =
            std::collections::HashMap::new();

        if let Ok(conn) = crate::storage::database::get_db_connection_pub() {
            let mut stmt = conn
                .prepare(
                    "SELECT id, branch_step, path, decoded_text, successful
                     FROM cache
                     WHERE parent_cache_id = ?1
                     ORDER BY branch_step ASC, timestamp DESC",
                )
                .ok();

            if let Some(ref mut stmt) = stmt {
                let rows: Vec<(usize, TreeNode)> = stmt
                    .query_map([cache_id], |row| {
                        let id: i64 = row.get(0)?;
                        let step: i64 = row.get(1)?;
                        let path_json: String = row.get(2)?;
                        let decoded_text: String = row.get(3)?;
                        let successful: bool = row.get(4)?;

                        // Parse the first decoder name
                        let path_vec: Vec<String> =
                            serde_json::from_str(&path_json).unwrap_or_default();
                        let first_decoder = if let Some(first_json) = path_vec.first() {
                            serde_json::from_str::<serde_json::Value>(first_json)
                                .ok()
                                .and_then(|v| {
                                    v.get("decoder")
                                        .and_then(|d| d.as_str().map(|s| s.to_string()))
                                })
                                .unwrap_or_else(|| "Unknown".to_string())
                        } else {
                            "Unknown".to_string()
                        };

                        // Truncate preview
                        let preview = if decoded_text.chars().count() > 20 {
                            format!("{}...", decoded_text.chars().take(17).collect::<String>())
                        } else {
                            decoded_text
                        };

                        Ok((id, step as usize, first_decoder, successful, preview))
                    })
                    .ok()
                    .map(|iter| {
                        iter.filter_map(|r| r.ok())
                            .map(|(id, step, decoder, successful, preview)| {
                                let has_children = count_sub_branches(id).unwrap_or(0) > 0;
                                (
                                    step,
                                    TreeNode {
                                        decoder_name: decoder,
                                        has_children,
                                        successful,
                                        cache_id: Some(id),
                                        text_preview: preview,
                                    },
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                for (step, node) in rows {
                    tree.entry(step).or_default().push(node);
                }
            }
        }

        tree
    }

    /// Refreshes the tree branch data from the database.
    ///
    /// Call this after any branch mutation (create, delete) to keep
    /// the tree view in sync.
    pub fn refresh_tree_branches(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            if let Some(cid) = rs.cache_id {
                rs.tree_branches = Self::load_tree_branches(cid, rs.result.path.len());
            }
        }
    }

    /// Sets the AI explanation text for the current step and caches it.
    ///
    /// # Arguments
    ///
    /// * `explanation` - The AI-generated explanation text
    pub fn set_ai_explanation(&mut self, explanation: String) {
        if let AppState::Results(ref mut rs) = self.state {
            let step = rs.selected_step;
            rs.ai_explanation_cache.insert(step, explanation.clone());
            rs.ai_explanation = Some(explanation.clone());
            rs.ai_loading = false;

            // Persist to database (best-effort)
            if let Some(cid) = rs.cache_id {
                let _ =
                    crate::storage::database::update_cache_ai_explanation(cid, step, &explanation);
            }
        }
    }

    /// Clears the AI explanation (e.g., when navigating to a different step).
    pub fn clear_ai_explanation(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            rs.ai_explanation = None;
            rs.ai_loading = false;
        }
    }

    /// Loads AI explanation from the per-step cache for the current step.
    ///
    /// Called when navigating between steps so cached explanations re-appear.
    pub fn load_cached_ai_explanation(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            rs.ai_explanation = rs.ai_explanation_cache.get(&rs.selected_step).cloned();
        }
    }

    /// Switches focus between the tree view, level detail, and step details panels.
    pub fn switch_focus(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            rs.focus = match rs.focus {
                ResultsFocus::TreeView => ResultsFocus::LevelDetail,
                ResultsFocus::LevelDetail => ResultsFocus::StepDetails,
                ResultsFocus::StepDetails => ResultsFocus::TreeView,
            };
        }
    }

    // ============================================================================
    // Branch Modal Methods
    // ============================================================================

    /// Opens the branch mode prompt modal.
    ///
    /// Called when the user presses Enter on a step that has no branches.
    /// Allows choosing between full A* search or single-layer decoding.
    pub fn open_branch_prompt(&mut self) {
        if let Some(context) = self.get_branch_context() {
            self.state = AppState::BranchModePrompt(BranchModePromptState {
                selected_mode: BranchMode::FullSearch,
                branch_context: context,
            });
        }
    }

    /// Closes the branch mode prompt and returns to Results state.
    ///
    /// Restores the Results state by loading from the database using
    /// the parent_cache_id stored in the branch context.
    pub fn close_branch_mode_prompt(&mut self) {
        use crate::decoders::crack_results::CrackResult;
        use crate::storage::database::get_cache_by_id;

        if let AppState::BranchModePrompt(ref bmp) = self.state {
            if let Some(parent_id) = bmp.branch_context.parent_cache_id {
                let branch_step = bmp.branch_context.branch_step;

                // Load the parent result from the database
                if let Ok(Some(cache_row)) = get_cache_by_id(parent_id) {
                    let crack_results: Vec<CrackResult> = cache_row
                        .path
                        .iter()
                        .filter_map(|json_str| serde_json::from_str(json_str).ok())
                        .collect();

                    let result = DecoderResult {
                        text: vec![cache_row.decoded_text.clone()],
                        path: crack_results,
                    };

                    self.input_text = cache_row.encoded_text;

                    // Restore to Results state
                    let mut rs = ResultsState::new_with_cache_id(result, parent_id);
                    rs.selected_step = branch_step;
                    self.state = AppState::Results(rs);

                    // Load branches for this step
                    self.load_branches_for_step();
                }
            }
        }
    }

    /// Opens the decoder search overlay.
    ///
    /// Called when the user presses '/' in Results state.
    /// The overlay floats on top of the Results screen without replacing it.
    pub fn open_decoder_search(&mut self) {
        use super::text_input::TextInput;
        use crate::decoders::get_all_decoder_names;

        // Only open if we're in Results state and have a branch context
        if let Some(context) = self.get_branch_context() {
            let all_decoders = get_all_decoder_names();
            let filtered_decoders = all_decoders.clone();

            self.decoder_search = Some(DecoderSearchOverlay {
                text_input: TextInput::new(),
                all_decoders,
                filtered_decoders,
                selected_index: 0,
                branch_context: context,
            });
        }
    }

    /// Closes the decoder search overlay.
    ///
    /// Simply clears the overlay, leaving the Results state unchanged.
    pub fn close_decoder_search(&mut self) {
        self.decoder_search = None;
    }

    /// Checks if the decoder search overlay is active.
    pub fn is_decoder_search_active(&self) -> bool {
        self.decoder_search.is_some()
    }

    /// Opens the quick search overlay.
    ///
    /// Called when the user presses 'o' in Results state.
    /// Parses the config's quick_searches entries into (name, url_template) pairs.
    pub fn open_quick_search(&mut self, config: &crate::config::Config) {
        // Only open if we're in Results state and have a selected step with output
        if let AppState::Results(ref rs) = self.state {
            let output_text = rs
                .result
                .path
                .get(rs.selected_step)
                .and_then(|step| step.unencrypted_text.as_ref())
                .and_then(|texts| texts.first().cloned())
                .unwrap_or_default();

            if output_text.is_empty() {
                return;
            }

            // Parse "Name=URL" entries
            let entries: Vec<(String, String)> = config
                .quick_searches
                .iter()
                .filter_map(|entry| {
                    let parts: Vec<&str> = entry.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();

            if entries.is_empty() {
                self.set_status(
                    "No quick searches configured. Add them in Settings (Ctrl+S).".to_string(),
                );
                return;
            }

            self.quick_search = Some(QuickSearchOverlay {
                entries,
                selected_index: 0,
                output_text,
            });
        }
    }

    /// Closes the quick search overlay.
    pub fn close_quick_search(&mut self) {
        self.quick_search = None;
    }

    /// Checks if the quick search overlay is active.
    pub fn is_quick_search_active(&self) -> bool {
        self.quick_search.is_some()
    }

    /// Opens the Ask AI overlay for the currently selected step.
    ///
    /// Extracts step context from the Results state and initializes
    /// the overlay with an empty question input.
    pub fn open_ask_ai(&mut self) {
        if let AppState::Results(ref rs) = self.state {
            if let Some(step) = rs.result.path.get(rs.selected_step) {
                let output_text = step
                    .unencrypted_text
                    .as_ref()
                    .and_then(|t| t.first().cloned())
                    .unwrap_or_default();

                self.ask_ai = Some(AskAiOverlay {
                    text_input: MultilineTextInput::new(),
                    decoder_name: step.decoder.to_string(),
                    step_input: step.encrypted_text.chars().take(500).collect(),
                    step_output: output_text.chars().take(500).collect(),
                    step_key: step.key.clone(),
                    step_description: step.description.to_string(),
                    step_link: step.link.to_string(),
                    response: None,
                    loading: false,
                    error: None,
                    response_scroll: 0,
                });
            }
        }
    }

    /// Closes the Ask AI overlay.
    pub fn close_ask_ai(&mut self) {
        self.ask_ai = None;
    }

    /// Checks if the Ask AI overlay is active.
    pub fn is_ask_ai_active(&self) -> bool {
        self.ask_ai.is_some()
    }

    /// Sets the Ask AI overlay to loading state.
    pub fn set_ask_ai_loading(&mut self) {
        if let Some(ref mut overlay) = self.ask_ai {
            overlay.loading = true;
            overlay.error = None;
        }
    }

    /// Sets the Ask AI response text.
    pub fn set_ask_ai_response(&mut self, response: String) {
        if let Some(ref mut overlay) = self.ask_ai {
            overlay.response = Some(response);
            overlay.loading = false;
            overlay.error = None;
            overlay.response_scroll = 0;
        }
    }

    /// Sets the Ask AI error message.
    pub fn set_ask_ai_error(&mut self, error: String) {
        if let Some(ref mut overlay) = self.ask_ai {
            overlay.error = Some(error);
            overlay.loading = false;
        }
    }

    /// Gets the appropriate help context based on current state.
    ///
    /// This determines which set of keybindings should be shown in the help overlay.
    pub fn help_context(&self) -> HelpContext {
        match &self.state {
            AppState::Home(_) => HelpContext::Home,
            AppState::Results(_) => HelpContext::Results,
            AppState::Settings(_)
            | AppState::ListEditor(_)
            | AppState::WordlistManager(_)
            | AppState::ThemePicker(_)
            | AppState::ToggleListEditor(_)
            | AppState::SaveConfirmation(_) => HelpContext::Settings,
            AppState::Loading(_)
            | AppState::HumanConfirmation(_)
            | AppState::Failure(_)
            | AppState::BranchModePrompt(_) => HelpContext::Loading,
        }
    }
}
