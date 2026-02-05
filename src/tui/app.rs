//! Application state machine for the Ciphey TUI.
//!
//! This module defines the core state management for the terminal user interface,
//! handling transitions between loading, results, settings, and failure states.

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::checkers::checker_result::CheckResult;
use crate::config::Config;
use crate::decoders::crack_results::CrackResult;
use crate::DecoderResult;

use super::settings::{FieldType, SettingValue, SettingsModel};
use super::setup_wizard::themes::ColorScheme;

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
        /// Input buffer for the field being edited.
        input_buffer: String,
        /// Cursor position in input buffer.
        cursor_pos: usize,
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
        /// Input buffer for new/editing item.
        input_buffer: String,
        /// Cursor position in input buffer.
        cursor_pos: usize,
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
        /// Input buffer for new wordlist path.
        new_path_input: String,
        /// Cursor position in path input.
        cursor_pos: usize,
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
        custom_colors: super::widgets::theme_picker::ThemePickerCustomColors,
        /// Current field in custom mode (0-4).
        custom_field: usize,
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

    // =========================================================================
    // Settings-related methods
    // =========================================================================

    /// Opens the settings panel from the current state.
    ///
    /// # Arguments
    ///
    /// * `config` - The current configuration to populate the settings model
    pub fn open_settings(&mut self, config: &Config) {
        // Don't allow opening settings from HumanConfirmation state
        if matches!(self.state, AppState::HumanConfirmation { .. }) {
            return;
        }

        // Capture previous state
        let previous_state = match &self.state {
            AppState::Loading {
                start_time,
                current_quote,
                spinner_frame,
            } => PreviousState::Loading {
                start_time: *start_time,
                current_quote: *current_quote,
                spinner_frame: *spinner_frame,
            },
            AppState::Results {
                result,
                selected_step,
                scroll_offset,
            } => PreviousState::Results {
                result: result.clone(),
                selected_step: *selected_step,
                scroll_offset: *scroll_offset,
            },
            AppState::Failure {
                input_text,
                elapsed,
            } => PreviousState::Failure {
                input_text: input_text.clone(),
                elapsed: *elapsed,
            },
            // Already in settings or sub-modals, do nothing
            _ => return,
        };

        // Create settings model from config
        let settings = SettingsModel::from_config(config);

        self.state = AppState::Settings {
            settings,
            selected_section: 0,
            selected_field: 0,
            editing_mode: false,
            input_buffer: String::new(),
            cursor_pos: 0,
            scroll_offset: 0,
            previous_state,
            validation_errors: HashMap::new(),
        };
    }

    /// Closes settings and returns to the previous state without saving.
    pub fn close_settings(&mut self) {
        if let AppState::Settings { previous_state, .. } = &self.state {
            self.state = match previous_state.clone() {
                PreviousState::Loading {
                    start_time,
                    current_quote,
                    spinner_frame,
                } => AppState::Loading {
                    start_time,
                    current_quote,
                    spinner_frame,
                },
                PreviousState::Results {
                    result,
                    selected_step,
                    scroll_offset,
                } => AppState::Results {
                    result,
                    selected_step,
                    scroll_offset,
                },
                PreviousState::Failure {
                    input_text,
                    elapsed,
                } => AppState::Failure {
                    input_text,
                    elapsed,
                },
            };
        }
    }

    /// Returns true if the app is in settings or a settings sub-modal.
    pub fn is_in_settings(&self) -> bool {
        matches!(
            self.state,
            AppState::Settings { .. }
                | AppState::ListEditor { .. }
                | AppState::WordlistManager { .. }
        )
    }

    /// Navigates to the next section in settings.
    pub fn next_settings_section(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            scroll_offset,
            ..
        } = &mut self.state
        {
            let section_count = settings.section_count();
            if section_count > 0 {
                *selected_section = (*selected_section + 1) % section_count;
                *selected_field = 0;
                *scroll_offset = 0;
            }
        }
    }

    /// Navigates to the previous section in settings.
    pub fn prev_settings_section(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            scroll_offset,
            ..
        } = &mut self.state
        {
            let section_count = settings.section_count();
            if section_count > 0 {
                *selected_section = if *selected_section == 0 {
                    section_count - 1
                } else {
                    *selected_section - 1
                };
                *selected_field = 0;
                *scroll_offset = 0;
            }
        }
    }

    /// Navigates to the next field in the current settings section.
    pub fn next_settings_field(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            ..
        } = &mut self.state
        {
            let field_count = settings.field_count(*selected_section);
            if field_count > 0 {
                *selected_field = (*selected_field + 1) % field_count;
            }
        }
    }

    /// Navigates to the previous field in the current settings section.
    pub fn prev_settings_field(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            ..
        } = &mut self.state
        {
            let field_count = settings.field_count(*selected_section);
            if field_count > 0 {
                *selected_field = if *selected_field == 0 {
                    field_count - 1
                } else {
                    *selected_field - 1
                };
            }
        }
    }

    /// Enters edit mode for the currently selected field, or activates it if it's a special type.
    pub fn edit_current_field(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            editing_mode,
            input_buffer,
            cursor_pos,
            scroll_offset,
            previous_state,
            validation_errors,
        } = &mut self.state
        {
            let field = match settings.get_field_at(*selected_section, *selected_field) {
                Some(f) => f,
                None => return,
            };

            match &field.field_type {
                // Boolean fields toggle immediately
                FieldType::Boolean => {
                    if let Some(field_mut) =
                        settings.get_field_at_mut(*selected_section, *selected_field)
                    {
                        if let SettingValue::Bool(v) = &field_mut.value {
                            field_mut.value = SettingValue::Bool(!*v);
                        }
                    }
                }
                // String lists open the list editor
                FieldType::StringList => {
                    let items = if let SettingValue::List(list) = &field.value {
                        list.clone()
                    } else {
                        vec![]
                    };

                    let snapshot = SettingsStateSnapshot {
                        settings: settings.clone(),
                        selected_section: *selected_section,
                        selected_field: *selected_field,
                        scroll_offset: *scroll_offset,
                        previous_state: previous_state.clone(),
                        validation_errors: validation_errors.clone(),
                    };

                    self.state = AppState::ListEditor {
                        field_id: field.id.to_string(),
                        field_label: field.label.to_string(),
                        items,
                        selected_item: None,
                        input_buffer: String::new(),
                        cursor_pos: 0,
                        parent_settings: Box::new(snapshot),
                    };
                }
                // Wordlist manager opens its own modal
                FieldType::WordlistManager => {
                    let snapshot = SettingsStateSnapshot {
                        settings: settings.clone(),
                        selected_section: *selected_section,
                        selected_field: *selected_field,
                        scroll_offset: *scroll_offset,
                        previous_state: previous_state.clone(),
                        validation_errors: validation_errors.clone(),
                    };

                    // TODO: Load wordlist files from database
                    self.state = AppState::WordlistManager {
                        wordlist_files: vec![],
                        selected_row: 0,
                        scroll_offset: 0,
                        parent_settings: Box::new(snapshot),
                        focus: WordlistManagerFocus::Table,
                        new_path_input: String::new(),
                        cursor_pos: 0,
                        pending_changes: HashMap::new(),
                    };
                }
                // Theme picker opens its own modal
                FieldType::ThemePicker => {
                    self.open_theme_picker();
                }
                // Other fields enter text editing mode
                _ => {
                    *editing_mode = true;
                    *input_buffer = field.display_value();
                    // Handle "(not set)" placeholder
                    if *input_buffer == "(not set)" {
                        *input_buffer = String::new();
                    }
                    *cursor_pos = input_buffer.len();
                }
            }
        }
    }

    /// Cancels editing the current field (exits edit mode without saving).
    pub fn cancel_field_edit(&mut self) {
        if let AppState::Settings {
            editing_mode,
            input_buffer,
            cursor_pos,
            ..
        } = &mut self.state
        {
            *editing_mode = false;
            *input_buffer = String::new();
            *cursor_pos = 0;
        }
    }

    /// Confirms the current field edit and updates the value.
    pub fn confirm_field_edit(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            editing_mode,
            input_buffer,
            cursor_pos,
            validation_errors,
            ..
        } = &mut self.state
        {
            if !*editing_mode {
                return;
            }

            let field = match settings.get_field_at(*selected_section, *selected_field) {
                Some(f) => f,
                None => return,
            };

            // Parse and validate the input
            match super::settings::validation::parse_input(input_buffer, &field.field_type) {
                Ok(new_value) => {
                    if let Some(field_mut) =
                        settings.get_field_at_mut(*selected_section, *selected_field)
                    {
                        field_mut.value = new_value;
                        validation_errors.remove(field_mut.id);
                    }
                    *editing_mode = false;
                    *input_buffer = String::new();
                    *cursor_pos = 0;
                }
                Err(e) => {
                    // Store validation error but stay in edit mode
                    validation_errors.insert(field.id.to_string(), e.to_string());
                }
            }
        }
    }

    /// Appends a character to the input buffer when editing.
    pub fn input_char(&mut self, c: char) {
        if let AppState::Settings {
            editing_mode,
            input_buffer,
            cursor_pos,
            ..
        } = &mut self.state
        {
            if *editing_mode {
                input_buffer.insert(*cursor_pos, c);
                *cursor_pos += 1;
            }
        } else if let AppState::ListEditor {
            input_buffer,
            cursor_pos,
            ..
        } = &mut self.state
        {
            input_buffer.insert(*cursor_pos, c);
            *cursor_pos += 1;
        } else if let AppState::WordlistManager {
            focus,
            new_path_input,
            cursor_pos,
            ..
        } = &mut self.state
        {
            if *focus == WordlistManagerFocus::AddPathInput {
                new_path_input.insert(*cursor_pos, c);
                *cursor_pos += 1;
            }
        }
    }

    /// Handles backspace in input buffer.
    pub fn input_backspace(&mut self) {
        if let AppState::Settings {
            editing_mode,
            input_buffer,
            cursor_pos,
            ..
        } = &mut self.state
        {
            if *editing_mode && *cursor_pos > 0 {
                *cursor_pos -= 1;
                input_buffer.remove(*cursor_pos);
            }
        } else if let AppState::ListEditor {
            input_buffer,
            cursor_pos,
            ..
        } = &mut self.state
        {
            if *cursor_pos > 0 {
                *cursor_pos -= 1;
                input_buffer.remove(*cursor_pos);
            }
        } else if let AppState::WordlistManager {
            focus,
            new_path_input,
            cursor_pos,
            ..
        } = &mut self.state
        {
            if *focus == WordlistManagerFocus::AddPathInput && *cursor_pos > 0 {
                *cursor_pos -= 1;
                new_path_input.remove(*cursor_pos);
            }
        }
    }

    /// Returns from list editor back to settings, updating the field value.
    pub fn finish_list_editor(&mut self) {
        if let AppState::ListEditor {
            field_id,
            items,
            parent_settings,
            ..
        } = &self.state
        {
            let mut snapshot = parent_settings.as_ref().clone();

            // Update the field value with the edited items
            if let Some(field) = snapshot.settings.get_field_mut(field_id) {
                field.value = SettingValue::List(items.clone());
            }

            self.state = AppState::Settings {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    /// Returns from list editor back to settings without saving changes.
    pub fn cancel_list_editor(&mut self) {
        if let AppState::ListEditor {
            parent_settings, ..
        } = &self.state
        {
            let snapshot = parent_settings.as_ref().clone();

            self.state = AppState::Settings {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    /// Adds a new item to the list editor.
    pub fn list_editor_add_item(&mut self) {
        if let AppState::ListEditor {
            items,
            input_buffer,
            cursor_pos,
            selected_item,
            ..
        } = &mut self.state
        {
            let trimmed = input_buffer.trim();
            if !trimmed.is_empty() {
                items.push(trimmed.to_string());
                *selected_item = Some(items.len() - 1);
            }
            *input_buffer = String::new();
            *cursor_pos = 0;
        }
    }

    /// Removes the selected item from the list editor.
    pub fn list_editor_remove_item(&mut self) {
        if let AppState::ListEditor {
            items,
            selected_item,
            ..
        } = &mut self.state
        {
            if let Some(idx) = *selected_item {
                if idx < items.len() {
                    items.remove(idx);
                    // Adjust selection
                    if items.is_empty() {
                        *selected_item = None;
                    } else if idx >= items.len() {
                        *selected_item = Some(items.len() - 1);
                    }
                }
            }
        }
    }

    /// Selects the next item in the list editor.
    pub fn list_editor_next_item(&mut self) {
        if let AppState::ListEditor {
            items,
            selected_item,
            ..
        } = &mut self.state
        {
            if items.is_empty() {
                *selected_item = None;
            } else {
                *selected_item = Some(match *selected_item {
                    Some(idx) => (idx + 1) % items.len(),
                    None => 0,
                });
            }
        }
    }

    /// Selects the previous item in the list editor.
    pub fn list_editor_prev_item(&mut self) {
        if let AppState::ListEditor {
            items,
            selected_item,
            ..
        } = &mut self.state
        {
            if items.is_empty() {
                *selected_item = None;
            } else {
                *selected_item = Some(match *selected_item {
                    Some(0) => items.len() - 1,
                    Some(idx) => idx - 1,
                    None => items.len() - 1,
                });
            }
        }
    }

    /// Returns from wordlist manager back to settings.
    pub fn finish_wordlist_manager(&mut self) {
        if let AppState::WordlistManager {
            parent_settings, ..
        } = &self.state
        {
            let snapshot = parent_settings.as_ref().clone();

            // TODO: Apply pending changes to database and rebuild bloom filter

            self.state = AppState::Settings {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    /// Cancels wordlist manager and returns to settings.
    pub fn cancel_wordlist_manager(&mut self) {
        if let AppState::WordlistManager {
            parent_settings, ..
        } = &self.state
        {
            let snapshot = parent_settings.as_ref().clone();

            self.state = AppState::Settings {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    /// Cycles focus in the wordlist manager (Table -> AddPath -> Done -> Table).
    pub fn wordlist_manager_next_focus(&mut self) {
        if let AppState::WordlistManager { focus, .. } = &mut self.state {
            *focus = match focus {
                WordlistManagerFocus::Table => WordlistManagerFocus::AddPathInput,
                WordlistManagerFocus::AddPathInput => WordlistManagerFocus::DoneButton,
                WordlistManagerFocus::DoneButton => WordlistManagerFocus::Table,
            };
        }
    }

    /// Checks if settings have any unsaved changes.
    pub fn settings_have_changes(&self) -> bool {
        if let AppState::Settings { settings, .. } = &self.state {
            settings.has_changes()
        } else {
            false
        }
    }

    /// Opens the theme picker modal from settings.
    ///
    /// This transitions from the Settings state to ThemePicker state,
    /// preserving the settings state for when the user returns.
    pub fn open_theme_picker(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            scroll_offset,
            validation_errors,
            previous_state,
            ..
        } = &self.state
        {
            let snapshot = SettingsStateSnapshot {
                settings: settings.clone(),
                selected_section: *selected_section,
                selected_field: *selected_field,
                scroll_offset: *scroll_offset,
                validation_errors: validation_errors.clone(),
                previous_state: previous_state.clone(),
            };

            self.state = AppState::ThemePicker {
                selected_theme: 0,
                custom_mode: false,
                custom_colors: super::widgets::theme_picker::ThemePickerCustomColors::default(),
                custom_field: 0,
                parent_settings: Box::new(snapshot),
            };
        }
    }

    /// Closes the theme picker and returns to settings.
    ///
    /// # Arguments
    ///
    /// * `apply_theme` - Whether to apply the selected theme to settings color fields
    /// * `scheme` - Optional color scheme to apply
    pub fn close_theme_picker(&mut self, apply_theme: bool, scheme: Option<ColorScheme>) {
        if let AppState::ThemePicker {
            parent_settings, ..
        } = &self.state
        {
            let snapshot = parent_settings.as_ref().clone();

            let mut settings = snapshot.settings;

            // Apply theme if requested
            if apply_theme {
                if let Some(s) = scheme {
                    // Update all 5 color fields
                    if let Some(field) = settings.get_field_mut("color_informational") {
                        field.value = SettingValue::Text(format!(
                            "{},{},{}",
                            s.informational.0, s.informational.1, s.informational.2
                        ));
                    }
                    if let Some(field) = settings.get_field_mut("color_warning") {
                        field.value = SettingValue::Text(format!(
                            "{},{},{}",
                            s.warning.0, s.warning.1, s.warning.2
                        ));
                    }
                    if let Some(field) = settings.get_field_mut("color_success") {
                        field.value = SettingValue::Text(format!(
                            "{},{},{}",
                            s.success.0, s.success.1, s.success.2
                        ));
                    }
                    if let Some(field) = settings.get_field_mut("color_error") {
                        field.value = SettingValue::Text(format!(
                            "{},{},{}",
                            s.error.0, s.error.1, s.error.2
                        ));
                    }
                    if let Some(field) = settings.get_field_mut("color_question") {
                        field.value = SettingValue::Text(format!(
                            "{},{},{}",
                            s.question.0, s.question.1, s.question.2
                        ));
                    }
                }
            }

            // Return to settings state
            self.state = AppState::Settings {
                settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    /// Shows the save confirmation modal from settings.
    pub fn show_save_confirmation(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            scroll_offset,
            validation_errors,
            previous_state,
            ..
        } = &self.state
        {
            let snapshot = SettingsStateSnapshot {
                settings: settings.clone(),
                selected_section: *selected_section,
                selected_field: *selected_field,
                scroll_offset: *scroll_offset,
                validation_errors: validation_errors.clone(),
                previous_state: previous_state.clone(),
            };

            self.state = AppState::SaveConfirmation {
                parent_settings: Box::new(snapshot),
            };
        }
    }

    /// Handles the user's response to the save confirmation modal.
    ///
    /// # Returns
    ///
    /// An Action indicating what needs to happen (Save or None).
    pub fn handle_save_confirmation(&mut self, save: bool) -> super::input::Action {
        if let AppState::SaveConfirmation { parent_settings } = &self.state {
            let snapshot = parent_settings.as_ref().clone();

            if save {
                // Restore settings state and return SaveSettings action
                self.state = AppState::Settings {
                    settings: snapshot.settings,
                    selected_section: snapshot.selected_section,
                    selected_field: snapshot.selected_field,
                    editing_mode: false,
                    input_buffer: String::new(),
                    cursor_pos: 0,
                    scroll_offset: snapshot.scroll_offset,
                    previous_state: snapshot.previous_state,
                    validation_errors: snapshot.validation_errors,
                };
                super::input::Action::SaveSettings
            } else {
                // Discard changes and close settings
                self.state = match snapshot.previous_state {
                    PreviousState::Loading {
                        start_time,
                        current_quote,
                        spinner_frame,
                    } => AppState::Loading {
                        start_time,
                        current_quote,
                        spinner_frame,
                    },
                    PreviousState::Results {
                        result,
                        selected_step,
                        scroll_offset,
                    } => AppState::Results {
                        result,
                        selected_step,
                        scroll_offset,
                    },
                    PreviousState::Failure {
                        input_text,
                        elapsed,
                    } => AppState::Failure {
                        input_text,
                        elapsed,
                    },
                };
                super::input::Action::None
            }
        } else {
            super::input::Action::None
        }
    }

    /// Cancels the save confirmation and returns to settings.
    pub fn cancel_save_confirmation(&mut self) {
        if let AppState::SaveConfirmation { parent_settings } = &self.state {
            let snapshot = parent_settings.as_ref().clone();

            self.state = AppState::Settings {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                input_buffer: String::new(),
                cursor_pos: 0,
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }
}
