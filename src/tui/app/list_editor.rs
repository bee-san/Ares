//! List editor modal management methods.

use super::super::{settings::SettingValue, text_input::TextInput};
use super::state::AppState;
use super::App;

impl App {
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
                text_input: TextInput::new(),
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
                text_input: TextInput::new(),
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
            text_input,
            selected_item,
            ..
        } = &mut self.state
        {
            let trimmed = text_input.get_text().trim();
            if !trimmed.is_empty() {
                items.push(trimmed.to_string());
                *selected_item = Some(items.len() - 1);
            }
            text_input.clear();
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
}
