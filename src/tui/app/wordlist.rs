//! Wordlist manager modal management methods.

use super::super::text_input::TextInput;
use super::state::{AppState, WordlistManagerFocus};
use super::App;

impl App {
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
                text_input: TextInput::new(),
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
                text_input: TextInput::new(),
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
}
