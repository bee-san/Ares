//! Settings panel management methods.

use std::collections::HashMap;

use crate::config::Config;
use crate::storage::database::read_all_wordlist_files;

use super::super::{
    settings::{FieldType, SettingValue, SettingsModel},
    setup_wizard::themes::ColorScheme,
    text_input::TextInput,
};
use super::state::{
    AppState, ListEditorState, PreviousState, ResultsStateSaved, SettingsState,
    SettingsStateSnapshot, ThemePickerState, ToggleListEditorState, WordlistFileInfo,
    WordlistManagerFocus, WordlistManagerState,
};
use super::App;

impl App {
    /// Opens the settings panel from the current state.
    ///
    /// # Arguments
    ///
    /// * `config` - The current configuration to populate the settings model
    pub fn open_settings(&mut self, config: &Config) {
        // Don't allow opening settings from HumanConfirmation state
        if matches!(self.state, AppState::HumanConfirmation(_)) {
            return;
        }

        // Capture previous state
        let previous_state = match &self.state {
            AppState::Home(home) => PreviousState::Home(home.clone()),
            AppState::Loading(ls) => PreviousState::Loading(ls.clone()),
            AppState::Results(rs) => {
                PreviousState::Results(Box::new(ResultsStateSaved::from_results(rs)))
            }
            AppState::Failure(fs) => PreviousState::Failure(fs.clone()),
            // Already in settings or sub-modals, do nothing
            _ => return,
        };

        // Create settings model from config
        let settings = SettingsModel::from_config(config);

        self.state = AppState::Settings(SettingsState {
            settings,
            selected_section: 0,
            selected_field: 0,
            editing_mode: false,
            text_input: TextInput::new(),
            scroll_offset: 0,
            previous_state,
            validation_errors: HashMap::new(),
        });
    }

    /// Closes settings and returns to the previous state without saving.
    pub fn close_settings(&mut self) {
        if let AppState::Settings(ref ss) = self.state {
            self.state = match ss.previous_state.clone() {
                PreviousState::Home(home) => AppState::Home(home),
                PreviousState::Loading(ls) => AppState::Loading(ls),
                PreviousState::Results(saved) => AppState::Results(saved.into_results()),
                PreviousState::Failure(fs) => AppState::Failure(fs),
            };
        }
    }

    /// Returns true if the app is in settings or a settings sub-modal.
    pub fn is_in_settings(&self) -> bool {
        matches!(
            self.state,
            AppState::Settings(_) | AppState::ListEditor(_) | AppState::WordlistManager(_)
        )
    }

    /// Navigates to the next section in settings.
    pub fn next_settings_section(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            let section_count = ss.settings.section_count();
            if section_count > 0 {
                ss.selected_section = (ss.selected_section + 1) % section_count;
                ss.selected_field = 0;
                ss.scroll_offset = 0;
            }
        }
    }

    /// Navigates to the previous section in settings.
    pub fn prev_settings_section(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            let section_count = ss.settings.section_count();
            if section_count > 0 {
                ss.selected_section = if ss.selected_section == 0 {
                    section_count - 1
                } else {
                    ss.selected_section - 1
                };
                ss.selected_field = 0;
                ss.scroll_offset = 0;
            }
        }
    }

    /// Navigates to the next field in the current settings section.
    pub fn next_settings_field(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            let field_count = ss.settings.field_count(ss.selected_section);
            if field_count > 0 {
                ss.selected_field = (ss.selected_field + 1) % field_count;
            }
        }
    }

    /// Navigates to the previous field in the current settings section.
    pub fn prev_settings_field(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            let field_count = ss.settings.field_count(ss.selected_section);
            if field_count > 0 {
                ss.selected_field = if ss.selected_field == 0 {
                    field_count - 1
                } else {
                    ss.selected_field - 1
                };
            }
        }
    }

    /// Enters edit mode for the currently selected field, or activates it if it's a special type.
    pub fn edit_current_field(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            let field = match ss.settings.get_field_at(ss.selected_section, ss.selected_field) {
                Some(f) => f,
                None => return,
            };

            match &field.field_type {
                // Boolean fields toggle immediately
                FieldType::Boolean => {
                    if let Some(field_mut) =
                        ss.settings
                            .get_field_at_mut(ss.selected_section, ss.selected_field)
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
                        settings: ss.settings.clone(),
                        selected_section: ss.selected_section,
                        selected_field: ss.selected_field,
                        scroll_offset: ss.scroll_offset,
                        previous_state: ss.previous_state.clone(),
                        validation_errors: ss.validation_errors.clone(),
                    };

                    self.state = AppState::ListEditor(ListEditorState {
                        field_id: field.id.to_string(),
                        field_label: field.label.to_string(),
                        items,
                        selected_item: None,
                        text_input: TextInput::new(),
                        parent_settings: Box::new(snapshot),
                    });
                }
                // Wordlist manager opens its own modal
                FieldType::WordlistManager => {
                    let snapshot = SettingsStateSnapshot {
                        settings: ss.settings.clone(),
                        selected_section: ss.selected_section,
                        selected_field: ss.selected_field,
                        scroll_offset: ss.scroll_offset,
                        previous_state: ss.previous_state.clone(),
                        validation_errors: ss.validation_errors.clone(),
                    };

                    // Load wordlist files from database
                    let wordlist_files = match read_all_wordlist_files() {
                        Ok(rows) => rows
                            .into_iter()
                            .map(|r| WordlistFileInfo {
                                id: r.id,
                                filename: r.filename,
                                file_path: r.file_path,
                                source: r.source,
                                word_count: r.word_count,
                                enabled: r.enabled,
                                added_date: r.added_date,
                            })
                            .collect(),
                        Err(_) => vec![],
                    };

                    self.state = AppState::WordlistManager(WordlistManagerState {
                        wordlist_files,
                        selected_row: 0,
                        scroll_offset: 0,
                        parent_settings: Box::new(snapshot),
                        focus: WordlistManagerFocus::Table,
                        text_input: TextInput::new(),
                        pending_changes: HashMap::new(),
                    });
                }
                // Theme picker opens its own modal
                FieldType::ThemePicker => {
                    self.open_theme_picker();
                }
                // Toggle list opens its own modal
                FieldType::ToggleList { all_items } => {
                    let selected_items = if let SettingValue::List(list) = &field.value {
                        list.clone()
                    } else {
                        vec![]
                    };

                    let snapshot = SettingsStateSnapshot {
                        settings: ss.settings.clone(),
                        selected_section: ss.selected_section,
                        selected_field: ss.selected_field,
                        scroll_offset: ss.scroll_offset,
                        previous_state: ss.previous_state.clone(),
                        validation_errors: ss.validation_errors.clone(),
                    };

                    self.state = AppState::ToggleListEditor(ToggleListEditorState {
                        field_id: field.id.to_string(),
                        field_label: field.label.to_string(),
                        all_items: all_items.clone(),
                        selected_items,
                        cursor_index: 0,
                        scroll_offset: 0,
                        parent_settings: Box::new(snapshot),
                    });
                }
                // Other fields enter text editing mode
                _ => {
                    ss.editing_mode = true;
                    let value = field.display_value();
                    // Handle "(not set)" placeholder
                    if value == "(not set)" {
                        ss.text_input.clear();
                    } else {
                        ss.text_input.set_text(value);
                    }
                }
            }
        }
    }

    /// Cancels editing the current field (exits edit mode without saving).
    pub fn cancel_field_edit(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            ss.editing_mode = false;
            ss.text_input.clear();
        }
    }

    /// Confirms the current field edit and updates the value.
    pub fn confirm_field_edit(&mut self) {
        if let AppState::Settings(ref mut ss) = self.state {
            if !ss.editing_mode {
                return;
            }

            let field =
                match ss.settings.get_field_at(ss.selected_section, ss.selected_field) {
                    Some(f) => f,
                    None => return,
                };

            // Parse and validate the input
            let input_text = ss.text_input.get_text();
            match super::super::settings::validation::parse_input(input_text, &field.field_type) {
                Ok(new_value) => {
                    if let Some(field_mut) =
                        ss.settings
                            .get_field_at_mut(ss.selected_section, ss.selected_field)
                    {
                        field_mut.value = new_value;
                        ss.validation_errors.remove(field_mut.id);
                    }
                    ss.editing_mode = false;
                    ss.text_input.clear();
                }
                Err(e) => {
                    // Store validation error but stay in edit mode
                    ss.validation_errors
                        .insert(field.id.to_string(), e.to_string());
                }
            }
        }
    }

    /// Appends a character to the input buffer when editing.
    pub fn input_char(&mut self, c: char) {
        match &mut self.state {
            AppState::Settings(ref mut ss) if ss.editing_mode => {
                ss.text_input.insert_char(c);
            }
            AppState::ListEditor(ref mut le) => {
                le.text_input.insert_char(c);
            }
            AppState::WordlistManager(ref mut wm)
                if wm.focus == WordlistManagerFocus::AddPathInput =>
            {
                wm.text_input.insert_char(c);
            }
            _ => {}
        }
    }

    /// Handles backspace in input buffer.
    pub fn input_backspace(&mut self) {
        match &mut self.state {
            AppState::Settings(ref mut ss) if ss.editing_mode => {
                ss.text_input.backspace();
            }
            AppState::ListEditor(ref mut le) => {
                le.text_input.backspace();
            }
            AppState::WordlistManager(ref mut wm)
                if wm.focus == WordlistManagerFocus::AddPathInput =>
            {
                wm.text_input.backspace();
            }
            _ => {}
        }
    }

    /// Checks if settings have any unsaved changes.
    pub fn settings_have_changes(&self) -> bool {
        if let AppState::Settings(ref ss) = self.state {
            ss.settings.has_changes()
        } else {
            false
        }
    }

    /// Opens the theme picker modal from settings.
    ///
    /// This transitions from the Settings state to ThemePicker state,
    /// preserving the settings state for when the user returns.
    pub fn open_theme_picker(&mut self) {
        if let AppState::Settings(ref ss) = self.state {
            let snapshot = SettingsStateSnapshot {
                settings: ss.settings.clone(),
                selected_section: ss.selected_section,
                selected_field: ss.selected_field,
                scroll_offset: ss.scroll_offset,
                validation_errors: ss.validation_errors.clone(),
                previous_state: ss.previous_state.clone(),
            };

            self.state = AppState::ThemePicker(ThemePickerState {
                selected_theme: 0,
                custom_mode: false,
                custom_colors:
                    super::super::widgets::theme_picker::ThemePickerCustomColors::default(),
                custom_field: 0,
                parent_settings: Box::new(snapshot),
            });
        }
    }

    /// Closes the theme picker and returns to settings.
    ///
    /// # Arguments
    ///
    /// * `apply_theme` - Whether to apply the selected theme to settings color fields
    /// * `scheme` - Optional color scheme to apply
    pub fn close_theme_picker(&mut self, apply_theme: bool, scheme: Option<ColorScheme>) {
        if let AppState::ThemePicker(ref tp) = self.state {
            let snapshot = tp.parent_settings.as_ref().clone();

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
            self.state = AppState::Settings(SettingsState {
                settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                text_input: TextInput::new(),
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            });
        }
    }

    /// Shows the save confirmation modal from settings.
    pub fn show_save_confirmation(&mut self) {
        if let AppState::Settings(ref ss) = self.state {
            let snapshot = SettingsStateSnapshot {
                settings: ss.settings.clone(),
                selected_section: ss.selected_section,
                selected_field: ss.selected_field,
                scroll_offset: ss.scroll_offset,
                validation_errors: ss.validation_errors.clone(),
                previous_state: ss.previous_state.clone(),
            };

            self.state = AppState::SaveConfirmation(super::state::SaveConfirmationState {
                parent_settings: Box::new(snapshot),
            });
        }
    }

    /// Handles the user's response to the save confirmation modal.
    ///
    /// # Returns
    ///
    /// An Action indicating what needs to happen (Save or None).
    pub fn handle_save_confirmation(&mut self, save: bool) -> super::super::input::Action {
        if let AppState::SaveConfirmation(ref sc) = self.state {
            let snapshot = sc.parent_settings.as_ref().clone();

            if save {
                // Restore settings state and return SaveSettings action
                self.state = AppState::Settings(SettingsState {
                    settings: snapshot.settings,
                    selected_section: snapshot.selected_section,
                    selected_field: snapshot.selected_field,
                    editing_mode: false,
                    text_input: TextInput::new(),
                    scroll_offset: snapshot.scroll_offset,
                    previous_state: snapshot.previous_state,
                    validation_errors: snapshot.validation_errors,
                });
                super::super::input::Action::SaveSettings
            } else {
                // Discard changes and close settings
                self.state = match snapshot.previous_state {
                    PreviousState::Home(home) => AppState::Home(home),
                    PreviousState::Loading(ls) => AppState::Loading(ls),
                    PreviousState::Results(saved) => AppState::Results(saved.into_results()),
                    PreviousState::Failure(fs) => AppState::Failure(fs),
                };
                super::super::input::Action::None
            }
        } else {
            super::super::input::Action::None
        }
    }

    /// Cancels the save confirmation and returns to settings.
    pub fn cancel_save_confirmation(&mut self) {
        if let AppState::SaveConfirmation(ref sc) = self.state {
            let snapshot = sc.parent_settings.as_ref().clone();

            self.state = AppState::Settings(SettingsState {
                settings: snapshot.settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                text_input: TextInput::new(),
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            });
        }
    }

    // ==================== Toggle List Editor Methods ====================

    /// Moves the cursor up in the toggle list editor.
    pub fn toggle_list_cursor_up(&mut self) {
        if let AppState::ToggleListEditor(ref mut tle) = self.state {
            if tle.cursor_index > 0 {
                tle.cursor_index -= 1;
                // Adjust scroll if needed
                if tle.cursor_index < tle.scroll_offset {
                    tle.scroll_offset = tle.cursor_index;
                }
            }
        }
    }

    /// Moves the cursor down in the toggle list editor.
    pub fn toggle_list_cursor_down(&mut self) {
        if let AppState::ToggleListEditor(ref mut tle) = self.state {
            if tle.cursor_index < tle.all_items.len().saturating_sub(1) {
                tle.cursor_index += 1;
                // Adjust scroll if needed (assuming visible height of ~15 items)
                const VISIBLE_HEIGHT: usize = 15;
                if tle.cursor_index >= tle.scroll_offset + VISIBLE_HEIGHT {
                    tle.scroll_offset = tle.cursor_index.saturating_sub(VISIBLE_HEIGHT - 1);
                }
            }
        }
    }

    /// Toggles the currently selected item in the toggle list editor.
    pub fn toggle_list_toggle_item(&mut self) {
        if let AppState::ToggleListEditor(ref mut tle) = self.state {
            if let Some(item) = tle.all_items.get(tle.cursor_index) {
                if tle.selected_items.contains(item) {
                    tle.selected_items.retain(|x| x != item);
                } else {
                    tle.selected_items.push(item.clone());
                }
            }
        }
    }

    /// Selects all items in the toggle list editor.
    pub fn toggle_list_select_all(&mut self) {
        if let AppState::ToggleListEditor(ref mut tle) = self.state {
            tle.selected_items = tle.all_items.clone();
        }
    }

    /// Deselects all items in the toggle list editor.
    pub fn toggle_list_select_none(&mut self) {
        if let AppState::ToggleListEditor(ref mut tle) = self.state {
            tle.selected_items.clear();
        }
    }

    /// Closes the toggle list editor and applies changes to the settings.
    pub fn close_toggle_list_editor(&mut self) {
        if let AppState::ToggleListEditor(ref tle) = self.state {
            let snapshot = tle.parent_settings.as_ref().clone();
            let mut settings = snapshot.settings;

            // Update the field value
            if let Some(field) = settings.get_field_mut(&tle.field_id) {
                field.value = SettingValue::List(tle.selected_items.clone());
            }

            // Return to settings state
            self.state = AppState::Settings(SettingsState {
                settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                text_input: TextInput::new(),
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            });
        }
    }
}
