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
    AppState, BranchContext, BranchMode, BranchPath, DecoderSearchOverlay, HelpContext,
    HistoryEntry, HumanConfirmationRequest, PreviousState, ResultsFocus, SettingsStateSnapshot,
    WordlistFileInfo, WordlistManagerFocus,
};

use crate::DecoderResult;

use super::multiline_text_input::MultilineTextInput;
use super::spinner::random_quote_index;

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
    /// Pending 'g' key for vim-style gg command.
    pub pending_g: bool,
    /// Decoder search overlay (floats over Results screen when Some).
    pub decoder_search: Option<DecoderSearchOverlay>,
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
                current_quote: random_quote_index(),
                spinner_frame: 0,
            },
            input_text,
            should_quit: false,
            show_help: false,
            status_message: None,
            pending_g: false,
            decoder_search: None,
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
            pending_g: false,
            decoder_search: None,
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
                current_quote: random_quote_index(),
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
                // Rotate quotes every ~100 ticks (assuming ~10 ticks/sec, change every 10 seconds)
                if *spinner_frame % 100 == 0 {
                    *current_quote = current_quote.wrapping_add(1);
                }
            }
            AppState::HumanConfirmation {
                spinner_frame,
                current_quote,
                ..
            } => {
                *spinner_frame = spinner_frame.wrapping_add(1);
                // Rotate quotes every ~100 ticks (assuming ~10 ticks/sec, change every 10 seconds)
                if *spinner_frame % 100 == 0 {
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
            cache_id: None,
            branch_path: state::BranchPath::new(),
            current_branches: Vec::new(),
            highlighted_branch: None,
            branch_scroll_offset: 0,
            focus: state::ResultsFocus::default(),
            tree_branches: std::collections::HashMap::new(),
            level_visible_rows: 10,
            ai_explanation: None,
            ai_loading: false,
        };
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
        let last_step = result.path.len().saturating_sub(1);
        // Load all branches for the tree view
        let tree_branches = Self::load_tree_branches(cache_id, result.path.len());
        self.state = AppState::Results {
            result,
            selected_step: last_step,
            scroll_offset: 0,
            cache_id: Some(cache_id),
            branch_path: state::BranchPath::new(),
            current_branches: Vec::new(),
            highlighted_branch: None,
            branch_scroll_offset: 0,
            focus: state::ResultsFocus::default(),
            tree_branches,
            level_visible_rows: 10,
            ai_explanation: None,
            ai_loading: false,
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
        if let AppState::Results {
            cache_id,
            result,
            tree_branches,
            ..
        } = &mut self.state
        {
            if let Some(cid) = cache_id {
                *tree_branches = Self::load_tree_branches(*cid, result.path.len());
            }
        }
    }

    /// Sets the AI explanation text for the current step.
    ///
    /// # Arguments
    ///
    /// * `explanation` - The AI-generated explanation text
    pub fn set_ai_explanation(&mut self, explanation: String) {
        if let AppState::Results {
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            *ai_explanation = Some(explanation);
            *ai_loading = false;
        }
    }

    /// Clears the AI explanation (e.g., when navigating to a different step).
    pub fn clear_ai_explanation(&mut self) {
        if let AppState::Results {
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            *ai_explanation = None;
            *ai_loading = false;
        }
    }

    /// Switches focus between the tree view, level detail, and step details panels.
    pub fn switch_focus(&mut self) {
        if let AppState::Results { focus, .. } = &mut self.state {
            *focus = match focus {
                state::ResultsFocus::TreeView => state::ResultsFocus::LevelDetail,
                state::ResultsFocus::LevelDetail => state::ResultsFocus::StepDetails,
                state::ResultsFocus::StepDetails => state::ResultsFocus::TreeView,
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
            self.state = AppState::BranchModePrompt {
                selected_mode: BranchMode::FullSearch,
                branch_context: context,
            };
        }
    }

    /// Closes the branch mode prompt and returns to Results state.
    ///
    /// Restores the Results state by loading from the database using
    /// the parent_cache_id stored in the branch context.
    pub fn close_branch_mode_prompt(&mut self) {
        use crate::decoders::crack_results::CrackResult;
        use crate::storage::database::get_cache_by_id;

        if let AppState::BranchModePrompt { branch_context, .. } = &self.state {
            if let Some(parent_id) = branch_context.parent_cache_id {
                let branch_step = branch_context.branch_step;

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
                    self.state = AppState::Results {
                        result,
                        selected_step: branch_step,
                        scroll_offset: 0,
                        cache_id: Some(parent_id),
                        branch_path: state::BranchPath::new(),
                        current_branches: Vec::new(),
                        highlighted_branch: None,
                        branch_scroll_offset: 0,
                        focus: state::ResultsFocus::default(),
                        tree_branches: Self::load_tree_branches(parent_id, 0),
                        level_visible_rows: 10,
                        ai_explanation: None,
                        ai_loading: false,
                    };

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

    /// Gets the appropriate help context based on current state.
    ///
    /// This determines which set of keybindings should be shown in the help overlay.
    pub fn help_context(&self) -> HelpContext {
        match &self.state {
            AppState::Home { .. } => HelpContext::Home,
            AppState::Results { .. } => HelpContext::Results,
            AppState::Settings { .. }
            | AppState::ListEditor { .. }
            | AppState::WordlistManager { .. }
            | AppState::ThemePicker { .. }
            | AppState::ToggleListEditor { .. }
            | AppState::SaveConfirmation { .. } => HelpContext::Settings,
            AppState::Loading { .. }
            | AppState::HumanConfirmation { .. }
            | AppState::Failure { .. }
            | AppState::BranchModePrompt { .. } => HelpContext::Loading,
        }
    }
}
