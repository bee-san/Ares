//! List editor modal management methods.

use super::super::{settings::SettingValue, text_input::TextInput};
use super::state::{AppState, SettingsState};
use super::App;

impl App {
    /// Returns from list editor back to settings, updating the field value.
    pub fn finish_list_editor(&mut self) {
        if let AppState::ListEditor(ref le) = self.state {
            let mut snapshot = le.parent_settings.as_ref().clone();

            // Update the field value with the edited items
            if let Some(field) = snapshot.settings.get_field_mut(&le.field_id) {
                field.value = SettingValue::List(le.items.clone());
            }

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

    /// Returns from list editor back to settings without saving changes.
    pub fn cancel_list_editor(&mut self) {
        if let AppState::ListEditor(ref le) = self.state {
            let snapshot = le.parent_settings.as_ref().clone();

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

    /// Adds a new item to the list editor.
    pub fn list_editor_add_item(&mut self) {
        if let AppState::ListEditor(ref mut le) = self.state {
            let trimmed = le.text_input.get_text().trim();
            if !trimmed.is_empty() {
                le.items.push(trimmed.to_string());
                le.selected_item = Some(le.items.len() - 1);
            }
            le.text_input.clear();
        }
    }

    /// Removes the selected item from the list editor.
    pub fn list_editor_remove_item(&mut self) {
        if let AppState::ListEditor(ref mut le) = self.state {
            if let Some(idx) = le.selected_item {
                if idx < le.items.len() {
                    le.items.remove(idx);
                    // Adjust selection
                    if le.items.is_empty() {
                        le.selected_item = None;
                    } else if idx >= le.items.len() {
                        le.selected_item = Some(le.items.len() - 1);
                    }
                }
            }
        }
    }

    /// Selects the next item in the list editor.
    pub fn list_editor_next_item(&mut self) {
        if let AppState::ListEditor(ref mut le) = self.state {
            if le.items.is_empty() {
                le.selected_item = None;
            } else {
                le.selected_item = Some(match le.selected_item {
                    Some(idx) => (idx + 1) % le.items.len(),
                    None => 0,
                });
            }
        }
    }

    /// Selects the previous item in the list editor.
    pub fn list_editor_prev_item(&mut self) {
        if let AppState::ListEditor(ref mut le) = self.state {
            if le.items.is_empty() {
                le.selected_item = None;
            } else {
                le.selected_item = Some(match le.selected_item {
                    Some(0) => le.items.len() - 1,
                    Some(idx) => idx - 1,
                    None => le.items.len() - 1,
                });
            }
        }
    }
}
