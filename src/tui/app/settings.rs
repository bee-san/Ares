//! Settings panel management methods.

use std::collections::HashMap;

use crate::config::Config;
use crate::storage::database::read_all_wordlist_files;

use super::super::{
    settings::{FieldType, SettingValue, SettingsModel},
    setup_wizard::themes::ColorScheme,
    text_input::TextInput,
};
use super::state::{AppState, PreviousState, SettingsStateSnapshot};
use super::App;

impl App {
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
            AppState::Home {
                text_input,
                history,
                selected_history,
                history_scroll_offset,
            } => PreviousState::Home {
                text_input: text_input.clone(),
                history: history.clone(),
                selected_history: *selected_history,
                history_scroll_offset: *history_scroll_offset,
            },
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
                cache_id,
                branch_path,
                highlighted_branch,
                branch_scroll_offset,
                focus,
                tree_branches,
                level_visible_rows,
                ai_explanation,
                ai_explanation_cache,
                ..
            } => PreviousState::Results {
                result: result.clone(),
                selected_step: *selected_step,
                scroll_offset: *scroll_offset,
                cache_id: *cache_id,
                branch_path: branch_path.clone(),
                highlighted_branch: *highlighted_branch,
                branch_scroll_offset: *branch_scroll_offset,
                focus: *focus,
                tree_branches: tree_branches.clone(),
                level_visible_rows: *level_visible_rows,
                ai_explanation: ai_explanation.clone(),
                ai_explanation_cache: ai_explanation_cache.clone(),
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
            text_input: TextInput::new(),
            scroll_offset: 0,
            previous_state,
            validation_errors: HashMap::new(),
        };
    }

    /// Closes settings and returns to the previous state without saving.
    pub fn close_settings(&mut self) {
        if let AppState::Settings { previous_state, .. } = &self.state {
            self.state = match previous_state.clone() {
                PreviousState::Home {
                    text_input,
                    history,
                    selected_history,
                    history_scroll_offset,
                } => AppState::Home {
                    text_input,
                    history,
                    selected_history,
                    history_scroll_offset,
                },
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
                    cache_id,
                    branch_path,
                    highlighted_branch,
                    branch_scroll_offset,
                    focus,
                    tree_branches,
                    level_visible_rows,
                    ai_explanation,
                    ai_explanation_cache,
                } => AppState::Results {
                    result,
                    selected_step,
                    scroll_offset,
                    cache_id,
                    branch_path,
                    current_branches: Vec::new(),
                    highlighted_branch,
                    branch_scroll_offset,
                    focus,
                    tree_branches,
                    level_visible_rows,
                    ai_explanation,
                    ai_loading: false,
                    ai_explanation_cache,
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
            text_input,
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
                        text_input: TextInput::new(),
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

                    // Load wordlist files from database
                    let wordlist_files = match read_all_wordlist_files() {
                        Ok(rows) => rows
                            .into_iter()
                            .map(|r| super::state::WordlistFileInfo {
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

                    self.state = AppState::WordlistManager {
                        wordlist_files,
                        selected_row: 0,
                        scroll_offset: 0,
                        parent_settings: Box::new(snapshot),
                        focus: super::state::WordlistManagerFocus::Table,
                        text_input: TextInput::new(),
                        pending_changes: HashMap::new(),
                    };
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
                        settings: settings.clone(),
                        selected_section: *selected_section,
                        selected_field: *selected_field,
                        scroll_offset: *scroll_offset,
                        previous_state: previous_state.clone(),
                        validation_errors: validation_errors.clone(),
                    };

                    self.state = AppState::ToggleListEditor {
                        field_id: field.id.to_string(),
                        field_label: field.label.to_string(),
                        all_items: all_items.clone(),
                        selected_items,
                        cursor_index: 0,
                        scroll_offset: 0,
                        parent_settings: Box::new(snapshot),
                    };
                }
                // Other fields enter text editing mode
                _ => {
                    *editing_mode = true;
                    let value = field.display_value();
                    // Handle "(not set)" placeholder
                    if value == "(not set)" {
                        text_input.clear();
                    } else {
                        text_input.set_text(value);
                    }
                }
            }
        }
    }

    /// Cancels editing the current field (exits edit mode without saving).
    pub fn cancel_field_edit(&mut self) {
        if let AppState::Settings {
            editing_mode,
            text_input,
            ..
        } = &mut self.state
        {
            *editing_mode = false;
            text_input.clear();
        }
    }

    /// Confirms the current field edit and updates the value.
    pub fn confirm_field_edit(&mut self) {
        if let AppState::Settings {
            settings,
            selected_section,
            selected_field,
            editing_mode,
            text_input,
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
            let input_text = text_input.get_text();
            match super::super::settings::validation::parse_input(input_text, &field.field_type) {
                Ok(new_value) => {
                    if let Some(field_mut) =
                        settings.get_field_at_mut(*selected_section, *selected_field)
                    {
                        field_mut.value = new_value;
                        validation_errors.remove(field_mut.id);
                    }
                    *editing_mode = false;
                    text_input.clear();
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
            text_input,
            ..
        } = &mut self.state
        {
            if *editing_mode {
                text_input.insert_char(c);
            }
        } else if let AppState::ListEditor { text_input, .. } = &mut self.state {
            text_input.insert_char(c);
        } else if let AppState::WordlistManager {
            focus, text_input, ..
        } = &mut self.state
        {
            if *focus == super::state::WordlistManagerFocus::AddPathInput {
                text_input.insert_char(c);
            }
        }
    }

    /// Handles backspace in input buffer.
    pub fn input_backspace(&mut self) {
        if let AppState::Settings {
            editing_mode,
            text_input,
            ..
        } = &mut self.state
        {
            if *editing_mode {
                text_input.backspace();
            }
        } else if let AppState::ListEditor { text_input, .. } = &mut self.state {
            text_input.backspace();
        } else if let AppState::WordlistManager {
            focus, text_input, ..
        } = &mut self.state
        {
            if *focus == super::state::WordlistManagerFocus::AddPathInput {
                text_input.backspace();
            }
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
                custom_colors:
                    super::super::widgets::theme_picker::ThemePickerCustomColors::default(),
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
                text_input: TextInput::new(),
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
    pub fn handle_save_confirmation(&mut self, save: bool) -> super::super::input::Action {
        if let AppState::SaveConfirmation { parent_settings } = &self.state {
            let snapshot = parent_settings.as_ref().clone();

            if save {
                // Restore settings state and return SaveSettings action
                self.state = AppState::Settings {
                    settings: snapshot.settings,
                    selected_section: snapshot.selected_section,
                    selected_field: snapshot.selected_field,
                    editing_mode: false,
                    text_input: TextInput::new(),
                    scroll_offset: snapshot.scroll_offset,
                    previous_state: snapshot.previous_state,
                    validation_errors: snapshot.validation_errors,
                };
                super::super::input::Action::SaveSettings
            } else {
                // Discard changes and close settings
                self.state = match snapshot.previous_state {
                    PreviousState::Home {
                        text_input,
                        history,
                        selected_history,
                        history_scroll_offset,
                    } => AppState::Home {
                        text_input,
                        history,
                        selected_history,
                        history_scroll_offset,
                    },
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
                        cache_id,
                        branch_path,
                        highlighted_branch,
                        branch_scroll_offset,
                        focus,
                        tree_branches,
                        level_visible_rows,
                        ai_explanation,
                        ai_explanation_cache,
                    } => AppState::Results {
                        result,
                        selected_step,
                        scroll_offset,
                        cache_id,
                        branch_path,
                        current_branches: Vec::new(),
                        highlighted_branch,
                        branch_scroll_offset,
                        focus,
                        tree_branches,
                        level_visible_rows,
                        ai_explanation,
                        ai_loading: false,
                        ai_explanation_cache,
                    },
                    PreviousState::Failure {
                        input_text,
                        elapsed,
                    } => AppState::Failure {
                        input_text,
                        elapsed,
                    },
                };
                super::super::input::Action::None
            }
        } else {
            super::super::input::Action::None
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
                text_input: TextInput::new(),
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }

    // ==================== Toggle List Editor Methods ====================

    /// Moves the cursor up in the toggle list editor.
    pub fn toggle_list_cursor_up(&mut self) {
        if let AppState::ToggleListEditor {
            cursor_index,
            scroll_offset,
            ..
        } = &mut self.state
        {
            if *cursor_index > 0 {
                *cursor_index -= 1;
                // Adjust scroll if needed
                if *cursor_index < *scroll_offset {
                    *scroll_offset = *cursor_index;
                }
            }
        }
    }

    /// Moves the cursor down in the toggle list editor.
    pub fn toggle_list_cursor_down(&mut self) {
        if let AppState::ToggleListEditor {
            all_items,
            cursor_index,
            scroll_offset,
            ..
        } = &mut self.state
        {
            if *cursor_index < all_items.len().saturating_sub(1) {
                *cursor_index += 1;
                // Adjust scroll if needed (assuming visible height of ~15 items)
                const VISIBLE_HEIGHT: usize = 15;
                if *cursor_index >= *scroll_offset + VISIBLE_HEIGHT {
                    *scroll_offset = cursor_index.saturating_sub(VISIBLE_HEIGHT - 1);
                }
            }
        }
    }

    /// Toggles the currently selected item in the toggle list editor.
    pub fn toggle_list_toggle_item(&mut self) {
        if let AppState::ToggleListEditor {
            all_items,
            selected_items,
            cursor_index,
            ..
        } = &mut self.state
        {
            if let Some(item) = all_items.get(*cursor_index) {
                if selected_items.contains(item) {
                    selected_items.retain(|x| x != item);
                } else {
                    selected_items.push(item.clone());
                }
            }
        }
    }

    /// Selects all items in the toggle list editor.
    pub fn toggle_list_select_all(&mut self) {
        if let AppState::ToggleListEditor {
            all_items,
            selected_items,
            ..
        } = &mut self.state
        {
            *selected_items = all_items.clone();
        }
    }

    /// Deselects all items in the toggle list editor.
    pub fn toggle_list_select_none(&mut self) {
        if let AppState::ToggleListEditor { selected_items, .. } = &mut self.state {
            selected_items.clear();
        }
    }

    /// Closes the toggle list editor and applies changes to the settings.
    pub fn close_toggle_list_editor(&mut self) {
        if let AppState::ToggleListEditor {
            field_id,
            selected_items,
            parent_settings,
            ..
        } = &self.state
        {
            let snapshot = parent_settings.as_ref().clone();
            let mut settings = snapshot.settings;

            // Update the field value
            if let Some(field) = settings.get_field_mut(field_id) {
                field.value = SettingValue::List(selected_items.clone());
            }

            // Return to settings state
            self.state = AppState::Settings {
                settings,
                selected_section: snapshot.selected_section,
                selected_field: snapshot.selected_field,
                editing_mode: false,
                text_input: TextInput::new(),
                scroll_offset: snapshot.scroll_offset,
                previous_state: snapshot.previous_state,
                validation_errors: snapshot.validation_errors,
            };
        }
    }
}
