//! State definitions for the TUI application.
//!
//! This module defines the state machine types used by the TUI,
//! handling transitions between loading, results, settings, and failure states.
//!
//! Each major state's data is encapsulated in its own struct (e.g. [`ResultsState`],
//! [`HomeState`]) so that methods can be implemented directly on the state rather
//! than requiring `App`-level guards everywhere.

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::checkers::checker_result::CheckResult;
use crate::decoders::crack_results::CrackResult;
use crate::storage::database::BranchSummary;
use crate::tui::widgets::tree_viewer::TreeNode;
use crate::DecoderResult;

use super::super::multiline_text_input::MultilineTextInput;
use super::super::settings::SettingsModel;
use super::super::text_input::TextInput;

// ============================================================================
// Shared helper types
// ============================================================================

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

/// Context for creating a branch from a node.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchContext {
    /// Text to decode (output from the branch point).
    pub text_to_decode: String,
    /// Path prefix up to (and including) the branch point.
    pub prefix_path: Vec<CrackResult>,
    /// Parent cache ID for database linking.
    pub parent_cache_id: Option<i64>,
    /// Step index in parent's path where we're branching.
    pub branch_step: usize,
}

/// Tracks current position in branch hierarchy.
#[derive(Debug, Clone, Default)]
pub struct BranchPath {
    /// Stack of (cache_id, branch_step) representing path from root.
    pub stack: Vec<(i64, usize)>,
}

impl BranchPath {
    /// Creates a new empty branch path (at main/root level).
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Get compact display string: "Main > B2 > B1".
    pub fn display(&self) -> String {
        if self.stack.is_empty() {
            "Main".to_string()
        } else {
            let mut parts = vec!["Main".to_string()];
            for (i, _) in self.stack.iter().enumerate() {
                parts.push(format!("B{}", i + 1));
            }
            parts.join(" > ")
        }
    }

    /// Check if currently viewing a branch (not main).
    pub fn is_branch(&self) -> bool {
        !self.stack.is_empty()
    }

    /// Push a new branch onto the stack.
    pub fn push(&mut self, cache_id: i64, branch_step: usize) {
        self.stack.push((cache_id, branch_step));
    }

    /// Pop the most recent branch from the stack.
    pub fn pop(&mut self) -> Option<(i64, usize)> {
        self.stack.pop()
    }

    /// Get the current cache_id (top of stack) if viewing a branch.
    pub fn current_cache_id(&self) -> Option<i64> {
        self.stack.last().map(|(id, _)| *id)
    }
}

/// Branch mode options (for creating new branches).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchMode {
    /// Run full A* search to find plaintext.
    FullSearch,
    /// Run all decoders once and show results.
    SingleLayer,
}

/// Which panel in the Results screen is currently focused.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResultsFocus {
    /// The birds-eye tree view panel (top right).
    TreeView,
    /// The level detail list panel (bottom right).
    LevelDetail,
    /// The step details panel (left).
    StepDetails,
}

impl Default for ResultsFocus {
    fn default() -> Self {
        Self::TreeView
    }
}

/// Context for showing appropriate help keybindings.
///
/// Different screens have different keybindings, so the help overlay
/// adapts its content based on the current context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HelpContext {
    /// Home screen keybindings.
    Home,
    /// Results screen keybindings.
    Results,
    /// Settings screen keybindings.
    Settings,
    /// Loading screen (minimal keybindings).
    Loading,
}

/// Overlay for asking AI a question about the current step.
///
/// This floats over the Results screen, providing a multiline input
/// for the question, showing step context, and displaying the AI response.
#[derive(Debug)]
pub struct AskAiOverlay {
    /// Multiline text input for the question.
    pub text_input: MultilineTextInput,
    /// Name of the decoder for the step being asked about.
    pub decoder_name: String,
    /// Input text to the decoder step.
    pub step_input: String,
    /// Output text from the decoder step.
    pub step_output: String,
    /// Key used by the decoder (if any).
    pub step_key: Option<String>,
    /// Description of the decoder.
    pub step_description: String,
    /// Reference link for the decoder.
    pub step_link: String,
    /// AI's response (None if not yet answered).
    pub response: Option<String>,
    /// Whether an AI request is currently in-flight.
    pub loading: bool,
    /// Error message if the AI request failed.
    pub error: Option<String>,
    /// Scroll offset for the response area (in lines).
    pub response_scroll: u16,
}

/// Overlay for vim-style decoder search (floats over Results screen).
///
/// This is implemented as an overlay rather than a state variant so that
/// the Results screen remains visible underneath.
#[derive(Debug)]
pub struct DecoderSearchOverlay {
    /// Text input for search query.
    pub text_input: TextInput,
    /// All available decoder names.
    pub all_decoders: Vec<&'static str>,
    /// Filtered decoder names based on search.
    pub filtered_decoders: Vec<&'static str>,
    /// Currently selected index in the filtered list.
    pub selected_index: usize,
    /// Context for creating the branch.
    pub branch_context: BranchContext,
}

/// Overlay for quick search (floats over Results screen).
///
/// Displays a list of configured search providers (e.g., Google, ChatGPT)
/// that open the selected step's output text in an external browser.
#[derive(Debug)]
pub struct QuickSearchOverlay {
    /// Parsed entries as (name, url_template) pairs.
    pub entries: Vec<(String, String)>,
    /// Currently selected index in the entries list.
    pub selected_index: usize,
    /// The output text from the selected step to search for.
    pub output_text: String,
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

// ============================================================================
// Per-state data structs
// ============================================================================

/// Data for the Home state where users can paste ciphertext to decode.
#[derive(Debug, Clone)]
pub struct HomeState {
    /// Multi-line text input for entering ciphertext.
    pub text_input: MultilineTextInput,
    /// History of previous decode attempts.
    pub history: Vec<HistoryEntry>,
    /// Currently selected history entry index (None = input focused).
    pub selected_history: Option<usize>,
    /// Scroll offset for the history panel.
    pub history_scroll_offset: usize,
}

/// Data for the Loading state while processing input.
#[derive(Debug, Clone)]
pub struct LoadingState {
    /// When the loading started, used to calculate elapsed time.
    pub start_time: Instant,
    /// Current index into the quotes array for display rotation.
    pub current_quote: usize,
    /// Current frame of the spinner animation.
    pub spinner_frame: usize,
}

impl LoadingState {
    /// Advances spinner animation and rotates quotes.
    pub fn tick(&mut self) {
        self.spinner_frame = self.spinner_frame.wrapping_add(1);
        if self.spinner_frame % 100 == 0 {
            self.current_quote = self.current_quote.wrapping_add(1);
        }
    }
}

/// Data for the HumanConfirmation state (waiting for user to confirm plaintext).
#[derive(Debug)]
pub struct HumanConfirmationState {
    /// When the loading started (preserved from Loading state).
    pub start_time: Instant,
    /// Current quote index (preserved from Loading state).
    pub current_quote: usize,
    /// Current spinner frame (preserved from Loading state).
    pub spinner_frame: usize,
    /// The confirmation request details.
    pub request: HumanConfirmationRequest,
    /// Channel to send the user's response back to the cracker thread.
    pub response_sender: mpsc::Sender<bool>,
}

impl HumanConfirmationState {
    /// Advances spinner animation and rotates quotes (same as loading).
    pub fn tick(&mut self) {
        self.spinner_frame = self.spinner_frame.wrapping_add(1);
        if self.spinner_frame % 100 == 0 {
            self.current_quote = self.current_quote.wrapping_add(1);
        }
    }
}

/// Data for the Results state (decoding completed successfully).
#[derive(Debug)]
pub struct ResultsState {
    /// The successful decoding result containing the path and plaintext.
    pub result: DecoderResult,
    /// Currently selected step in the decoding path.
    pub selected_step: usize,
    /// Vertical scroll offset for long content.
    pub scroll_offset: usize,
    /// Cache ID for this result (for branch linking).
    pub cache_id: Option<i64>,
    /// Current position in branch hierarchy.
    pub branch_path: BranchPath,
    /// Branches for the currently selected step (loaded on demand).
    pub current_branches: Vec<BranchSummary>,
    /// Index of highlighted branch in list (None = no branch highlighted).
    pub highlighted_branch: Option<usize>,
    /// Scroll offset for branch list.
    pub branch_scroll_offset: usize,
    /// Which panel is currently focused (tree view or level detail).
    pub focus: ResultsFocus,
    /// Cached tree data: branches at each step index, keyed by step index.
    /// This avoids re-querying the database on every render.
    pub tree_branches: HashMap<usize, Vec<TreeNode>>,
    /// Number of visible rows in the level detail panel (updated during rendering).
    /// Used by auto-scroll logic to keep highlighted branch visible.
    pub level_visible_rows: usize,
    /// AI-generated explanation of the currently selected step (if loaded).
    pub ai_explanation: Option<String>,
    /// Whether an AI explanation request is currently in progress.
    pub ai_loading: bool,
    /// Cache of AI explanations for all steps (step_index -> explanation text).
    /// This avoids re-fetching explanations when navigating between steps.
    pub ai_explanation_cache: HashMap<usize, String>,
}

impl ResultsState {
    /// Creates a new `ResultsState` with sensible defaults.
    ///
    /// The selected step defaults to the last step in the path (the plaintext),
    /// so pressing 'c' to copy will copy the final output by default.
    pub fn new(result: DecoderResult) -> Self {
        let last_step = result.path.len().saturating_sub(1);
        Self {
            result,
            selected_step: last_step,
            scroll_offset: 0,
            cache_id: None,
            branch_path: BranchPath::new(),
            current_branches: Vec::new(),
            highlighted_branch: None,
            branch_scroll_offset: 0,
            focus: ResultsFocus::default(),
            tree_branches: HashMap::new(),
            level_visible_rows: 10,
            ai_explanation: None,
            ai_loading: false,
            ai_explanation_cache: HashMap::new(),
        }
    }

    /// Creates a new `ResultsState` with a cache ID and preloaded tree/AI data.
    ///
    /// Used when showing results from the database (e.g., history) where we
    /// have a cache_id for branch linking.
    pub fn new_with_cache_id(result: DecoderResult, cache_id: i64) -> Self {
        let last_step = result.path.len().saturating_sub(1);
        let tree_branches = super::App::load_tree_branches_static(cache_id);
        let ai_explanation_cache =
            crate::storage::database::read_cache_ai_explanations(cache_id).unwrap_or_default();
        Self {
            result,
            selected_step: last_step,
            scroll_offset: 0,
            cache_id: Some(cache_id),
            branch_path: BranchPath::new(),
            current_branches: Vec::new(),
            highlighted_branch: None,
            branch_scroll_offset: 0,
            focus: ResultsFocus::default(),
            tree_branches,
            level_visible_rows: 10,
            ai_explanation: None,
            ai_loading: false,
            ai_explanation_cache,
        }
    }
}

/// Data for the Failure state (decoding failed to produce plaintext).
#[derive(Debug, Clone)]
pub struct FailureState {
    /// The original input text that could not be decoded.
    pub input_text: String,
    /// How long the decoding attempt took before failing.
    pub elapsed: Duration,
}

/// Data for the Settings panel state.
#[derive(Debug)]
pub struct SettingsState {
    /// The settings model being edited.
    pub settings: SettingsModel,
    /// Which section is currently selected (0 = General, 1 = Checkers, etc.).
    pub selected_section: usize,
    /// Which field within the section is selected.
    pub selected_field: usize,
    /// Whether we're currently editing a field value.
    pub editing_mode: bool,
    /// Text input component for editing field values.
    pub text_input: TextInput,
    /// Scroll offset for long settings lists.
    pub scroll_offset: usize,
    /// The state we came from (to return to after save/cancel).
    pub previous_state: PreviousState,
    /// Validation errors (field_id -> error message).
    pub validation_errors: HashMap<String, String>,
}

/// Data for the ListEditor sub-modal (editing string lists).
#[derive(Debug)]
pub struct ListEditorState {
    /// Field being edited.
    pub field_id: String,
    /// Field label for display.
    pub field_label: String,
    /// Current list items.
    pub items: Vec<String>,
    /// Currently selected item index (None if adding new).
    pub selected_item: Option<usize>,
    /// Text input component for adding/editing items.
    pub text_input: TextInput,
    /// Parent settings state snapshot to return to.
    pub parent_settings: Box<SettingsStateSnapshot>,
}

/// Data for the WordlistManager sub-modal.
#[derive(Debug)]
pub struct WordlistManagerState {
    /// List of wordlist files from database.
    pub wordlist_files: Vec<WordlistFileInfo>,
    /// Currently selected row.
    pub selected_row: usize,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Parent settings state snapshot to return to.
    pub parent_settings: Box<SettingsStateSnapshot>,
    /// Current focus (table, add button, done button).
    pub focus: WordlistManagerFocus,
    /// Text input component for new wordlist path.
    pub text_input: TextInput,
    /// Pending changes (file_id -> new_enabled_state).
    pub pending_changes: HashMap<i64, bool>,
}

/// Data for the ThemePicker modal.
#[derive(Debug)]
pub struct ThemePickerState {
    /// Currently selected theme index (0-5 for presets, 6 for custom).
    pub selected_theme: usize,
    /// Whether in custom color input mode.
    pub custom_mode: bool,
    /// Custom colors being edited.
    pub custom_colors: super::super::widgets::theme_picker::ThemePickerCustomColors,
    /// Current field in custom mode (0-4).
    pub custom_field: usize,
    /// Parent settings state snapshot to return to.
    pub parent_settings: Box<SettingsStateSnapshot>,
}

/// Data for the ToggleListEditor modal (selecting items from a fixed set).
#[derive(Debug)]
pub struct ToggleListEditorState {
    /// Field being edited.
    pub field_id: String,
    /// Field label for display.
    pub field_label: String,
    /// All available items that can be toggled.
    pub all_items: Vec<String>,
    /// Currently selected/enabled items.
    pub selected_items: Vec<String>,
    /// Currently highlighted item index in the list.
    pub cursor_index: usize,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Parent settings state snapshot to return to.
    pub parent_settings: Box<SettingsStateSnapshot>,
}

/// Data for the SaveConfirmation modal.
#[derive(Debug)]
pub struct SaveConfirmationState {
    /// The settings state to return to if user cancels.
    pub parent_settings: Box<SettingsStateSnapshot>,
}

/// Data for the BranchModePrompt modal.
#[derive(Debug, Clone)]
pub struct BranchModePromptState {
    /// Currently selected mode.
    pub selected_mode: BranchMode,
    /// Context for creating the branch.
    pub branch_context: BranchContext,
}

// ============================================================================
// AppState enum — thin wrapper around per-state structs
// ============================================================================

/// Represents the current state of the TUI application.
///
/// Each variant wraps a dedicated state struct so that methods can be
/// implemented directly on the data rather than requiring destructuring
/// guards on `App`.
#[derive(Debug)]
pub enum AppState {
    /// Home screen where users can paste ciphertext to decode.
    Home(HomeState),
    /// The application is processing input and waiting for results.
    Loading(LoadingState),
    /// Waiting for human confirmation of a potential plaintext.
    HumanConfirmation(HumanConfirmationState),
    /// Decoding completed successfully with results to display.
    Results(ResultsState),
    /// Decoding failed to produce any valid plaintext.
    Failure(FailureState),
    /// Settings panel for editing configuration.
    Settings(SettingsState),
    /// Sub-modal for editing string lists (lemmeknow tags).
    ListEditor(ListEditorState),
    /// Sub-modal for wordlist management table.
    WordlistManager(WordlistManagerState),
    /// Theme picker modal for selecting color schemes.
    ThemePicker(ThemePickerState),
    /// Toggle list editor for selecting items from a fixed set.
    ToggleListEditor(ToggleListEditorState),
    /// Confirmation modal asking if user wants to save settings.
    SaveConfirmation(SaveConfirmationState),
    /// Modal for selecting branch mode (when creating new branch).
    BranchModePrompt(BranchModePromptState),
}

// ============================================================================
// PreviousState — reuses per-state structs where possible
// ============================================================================

/// Represents the state we came from before entering settings.
#[derive(Debug, Clone)]
pub enum PreviousState {
    /// Was in the home state.
    Home(HomeState),
    /// Was in the loading state.
    Loading(LoadingState),
    /// Was in the results state.
    Results(Box<ResultsStateSaved>),
    /// Was in the failure state.
    Failure(FailureState),
}

/// Saved subset of `ResultsState` for `PreviousState`.
///
/// This is a clone-friendly snapshot that omits non-cloneable and
/// transient fields (`current_branches`, `ai_loading`), which get
/// sensible defaults when restored.
#[derive(Debug, Clone)]
pub struct ResultsStateSaved {
    /// The decoding result.
    pub result: DecoderResult,
    /// Selected step index.
    pub selected_step: usize,
    /// Scroll offset.
    pub scroll_offset: usize,
    /// Cache ID for this result.
    pub cache_id: Option<i64>,
    /// Current position in branch hierarchy.
    pub branch_path: BranchPath,
    /// Highlighted branch index.
    pub highlighted_branch: Option<usize>,
    /// Branch scroll offset.
    pub branch_scroll_offset: usize,
    /// Which panel is focused.
    pub focus: ResultsFocus,
    /// Cached tree branch data.
    pub tree_branches: HashMap<usize, Vec<TreeNode>>,
    /// Number of visible rows in the level detail panel.
    pub level_visible_rows: usize,
    /// Cached AI explanation text.
    pub ai_explanation: Option<String>,
    /// Cache of AI explanations per step.
    pub ai_explanation_cache: HashMap<usize, String>,
}

impl ResultsStateSaved {
    /// Creates a saved snapshot from a live `ResultsState`.
    pub fn from_results(rs: &ResultsState) -> Self {
        Self {
            result: rs.result.clone(),
            selected_step: rs.selected_step,
            scroll_offset: rs.scroll_offset,
            cache_id: rs.cache_id,
            branch_path: rs.branch_path.clone(),
            highlighted_branch: rs.highlighted_branch,
            branch_scroll_offset: rs.branch_scroll_offset,
            focus: rs.focus,
            tree_branches: rs.tree_branches.clone(),
            level_visible_rows: rs.level_visible_rows,
            ai_explanation: rs.ai_explanation.clone(),
            ai_explanation_cache: rs.ai_explanation_cache.clone(),
        }
    }

    /// Restores a full `ResultsState` from this saved snapshot.
    ///
    /// Transient fields (`current_branches`, `ai_loading`) get default values.
    pub fn into_results(self) -> ResultsState {
        ResultsState {
            result: self.result,
            selected_step: self.selected_step,
            scroll_offset: self.scroll_offset,
            cache_id: self.cache_id,
            branch_path: self.branch_path,
            current_branches: Vec::new(),
            highlighted_branch: self.highlighted_branch,
            branch_scroll_offset: self.branch_scroll_offset,
            focus: self.focus,
            tree_branches: self.tree_branches,
            level_visible_rows: self.level_visible_rows,
            ai_explanation: self.ai_explanation,
            ai_loading: false,
            ai_explanation_cache: self.ai_explanation_cache,
        }
    }
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
