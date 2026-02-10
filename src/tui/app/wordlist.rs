//! Wordlist manager modal management methods.

use super::super::text_input::TextInput;
use super::state::{AppState, SettingsState, WordlistManagerFocus};
use super::App;
use crate::storage::bloom::{build_bloom_filter_from_db, save_bloom_filter};
use crate::storage::database::{set_wordlist_file_enabled, set_words_enabled_by_file_id};

impl App {
    /// Returns from wordlist manager back to settings.
    ///
    /// Applies pending changes to the database and rebuilds the bloom filter
    /// if any changes were made.
    pub fn finish_wordlist_manager(&mut self) {
        if let AppState::WordlistManager(ref wm) = self.state {
            let snapshot = wm.parent_settings.as_ref().clone();
            let changes = wm.pending_changes.clone();

            // Apply pending changes to database
            let mut changes_applied = false;
            for (file_id, enabled) in changes.iter() {
                // Update the wordlist file's enabled status
                if set_wordlist_file_enabled(*file_id, *enabled).is_ok() {
                    // Also update all words from that file
                    let _ = set_words_enabled_by_file_id(*file_id, *enabled);
                    changes_applied = true;
                }
            }

            // Rebuild bloom filter if changes were made
            if changes_applied {
                if let Ok(bloom) = build_bloom_filter_from_db() {
                    let _ = save_bloom_filter(&bloom);
                }
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

    /// Cancels wordlist manager and returns to settings.
    pub fn cancel_wordlist_manager(&mut self) {
        if let AppState::WordlistManager(ref wm) = self.state {
            let snapshot = wm.parent_settings.as_ref().clone();

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

    /// Cycles focus in the wordlist manager (Table -> AddPath -> Done -> Table).
    pub fn wordlist_manager_next_focus(&mut self) {
        if let AppState::WordlistManager(ref mut wm) = self.state {
            wm.focus = match wm.focus {
                WordlistManagerFocus::Table => WordlistManagerFocus::AddPathInput,
                WordlistManagerFocus::AddPathInput => WordlistManagerFocus::DoneButton,
                WordlistManagerFocus::DoneButton => WordlistManagerFocus::Table,
            };
        }
    }
}
