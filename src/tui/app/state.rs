//! State definitions for the TUI application.
//!
//! This module defines the state machine types used by the TUI,
//! handling transitions between loading, results, settings, and failure states.

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::checkers::checker_result::CheckResult;
use crate::DecoderResult;

use super::super::multiline_text_input::MultilineTextInput;
use super::super::settings::SettingsModel;
use super::super::text_input::TextInput;

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

/// A simplified history entry for display in the history panel.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Database row ID.
    pub id: i64,
    /// Preview of the encoded text (~20 chars).
    pub encoded_text_preview: String,
    /// Full encoded text (for populating input on failed entries).
    pub encoded_text_full: String,
    /// Full decoded text (for successful entries).
    pub decoded_text: String,
    /// Full decoder path as JSON strings (for reconstructing DecoderResult).
    pub path: Vec<String>,
    /// Whether the decode was successful.
    pub successful: bool,
    /// Human-readable timestamp.
    pub timestamp: String,
}

impl HistoryEntry {
    /// Creates a HistoryEntry from a CacheRow.
    ///
    /// # Arguments
    ///
    /// * `cache_row` - The database cache row to convert
    pub fn from_cache_row(cache_row: &crate::storage::database::CacheRow) -> Self {
        // Create preview: first ~20 chars with ellipsis if needed
        let preview = if cache_row.encoded_text.chars().count() > 20 {
            format!(
                "{}...",
                cache_row.encoded_text.chars().take(17).collect::<String>()
            )
        } else {
            cache_row.encoded_text.clone()
        };

        Self {
            id: cache_row.id,
            encoded_text_preview: preview,
            encoded_text_full: cache_row.encoded_text.clone(),
            decoded_text: cache_row.decoded_text.clone(),
            path: cache_row.path.clone(),
            successful: cache_row.successful,
            timestamp: cache_row.timestamp.clone(),
        }
    }
}

/// Represents the current state of the TUI application.
#[derive(Debug)]
pub enum AppState {
    /// Home screen where users can paste ciphertext to decode.
    Home {
        /// Multi-line text input for entering ciphertext.
        text_input: MultilineTextInput,
        /// History of previous decode attempts.
        history: Vec<HistoryEntry>,
        /// Currently selected history entry index (None = input focused).
        selected_history: Option<usize>,
        /// Scroll offset for the history panel.
        history_scroll_offset: usize,
    },
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
    /// Settings panel for editing configuration.
    Settings {
        /// The settings model being edited.
        settings: SettingsModel,
        /// Which section is currently selected (0 = General, 1 = Checkers, etc.).
        selected_section: usize,
        /// Which field within the section is selected.
        selected_field: usize,
        /// Whether we're currently editing a field value.
        editing_mode: bool,
        /// Text input component for editing field values.
        text_input: TextInput,
        /// Scroll offset for long settings lists.
        scroll_offset: usize,
        /// The state we came from (to return to after save/cancel).
        previous_state: PreviousState,
        /// Validation errors (field_id -> error message).
        validation_errors: HashMap<String, String>,
    },
    /// Sub-modal for editing string lists (lemmeknow tags).
    ListEditor {
        /// Field being edited.
        field_id: String,
        /// Field label for display.
        field_label: String,
        /// Current list items.
        items: Vec<String>,
        /// Currently selected item index (None if adding new).
        selected_item: Option<usize>,
        /// Text input component for adding/editing items.
        text_input: TextInput,
        /// Parent settings state snapshot to return to.
        parent_settings: Box<SettingsStateSnapshot>,
    },
    /// Sub-modal for wordlist management table.
    WordlistManager {
        /// List of wordlist files from database.
        wordlist_files: Vec<WordlistFileInfo>,
        /// Currently selected row.
        selected_row: usize,
        /// Scroll offset for long lists.
        scroll_offset: usize,
        /// Parent settings state snapshot to return to.
        parent_settings: Box<SettingsStateSnapshot>,
        /// Current focus (table, add button, done button).
        focus: WordlistManagerFocus,
        /// Text input component for new wordlist path.
        text_input: TextInput,
        /// Pending changes (file_id -> new_enabled_state).
        pending_changes: HashMap<i64, bool>,
    },
    /// Theme picker modal for selecting color schemes.
    ThemePicker {
        /// Currently selected theme index (0-5 for presets, 6 for custom).
        selected_theme: usize,
        /// Whether in custom color input mode.
        custom_mode: bool,
        /// Custom colors being edited.
        custom_colors: super::super::widgets::theme_picker::ThemePickerCustomColors,
        /// Current field in custom mode (0-4).
        custom_field: usize,
        /// Parent settings state snapshot to return to.
        parent_settings: Box<SettingsStateSnapshot>,
    },
    /// Toggle list editor for selecting items from a fixed set.
    /// Used for decoder/checker selection.
    ToggleListEditor {
        /// Field being edited.
        field_id: String,
        /// Field label for display.
        field_label: String,
        /// All available items that can be toggled.
        all_items: Vec<String>,
        /// Currently selected/enabled items.
        selected_items: Vec<String>,
        /// Currently highlighted item index in the list.
        cursor_index: usize,
        /// Scroll offset for long lists.
        scroll_offset: usize,
        /// Parent settings state snapshot to return to.
        parent_settings: Box<SettingsStateSnapshot>,
    },
    /// Confirmation modal asking if user wants to save settings.
    SaveConfirmation {
        /// The settings state to return to if user cancels.
        parent_settings: Box<SettingsStateSnapshot>,
    },
}

/// Represents the state we came from before entering settings.
#[derive(Debug, Clone)]
pub enum PreviousState {
    /// Was in the home state.
    Home {
        /// Multi-line text input for entering ciphertext.
        text_input: super::super::multiline_text_input::MultilineTextInput,
        /// History of previous decode attempts.
        history: Vec<HistoryEntry>,
        /// Currently selected history entry index (None = input focused).
        selected_history: Option<usize>,
        /// Scroll offset for the history panel.
        history_scroll_offset: usize,
    },
    /// Was in the loading state.
    Loading {
        /// When loading started.
        start_time: Instant,
        /// Current quote index.
        current_quote: usize,
        /// Current spinner frame.
        spinner_frame: usize,
    },
    /// Was in the results state.
    Results {
        /// The decoding result.
        result: DecoderResult,
        /// Selected step index.
        selected_step: usize,
        /// Scroll offset.
        scroll_offset: usize,
    },
    /// Was in the failure state.
    Failure {
        /// The input text.
        input_text: String,
        /// Time elapsed during decoding.
        elapsed: Duration,
    },
}

/// Snapshot of settings state for returning from sub-modals.
#[derive(Debug, Clone)]
pub struct SettingsStateSnapshot {
    /// The settings model being edited.
    pub settings: SettingsModel,
    /// Which section is selected.
    pub selected_section: usize,
    /// Which field is selected.
    pub selected_field: usize,
    /// Scroll offset.
    pub scroll_offset: usize,
    /// The previous state before entering settings.
    pub previous_state: PreviousState,
    /// Validation errors.
    pub validation_errors: HashMap<String, String>,
}

/// Focus state for wordlist manager.
#[derive(Debug, Clone, PartialEq)]
pub enum WordlistManagerFocus {
    /// Navigating the wordlist table.
    Table,
    /// Typing a file path to add.
    AddPathInput,
    /// Focused on the Done button.
    DoneButton,
}

/// Information about a wordlist file stored in the database.
#[derive(Debug, Clone)]
pub struct WordlistFileInfo {
    /// Database ID.
    pub id: i64,
    /// Display filename.
    pub filename: String,
    /// Full file path.
    pub file_path: String,
    /// Source identifier.
    pub source: String,
    /// Number of words in this wordlist.
    pub word_count: i64,
    /// Whether this wordlist is enabled.
    pub enabled: bool,
    /// When the wordlist was added.
    pub added_date: String,
}
