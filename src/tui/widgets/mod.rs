//! Widget components for the Ciphey TUI.
//!
//! This module provides reusable widget components for building the terminal
//! user interface, including:
//!
//! - [`path_viewer`]: Visualizes the decoder chain as connected boxes
//! - [`step_details`]: Shows detailed information about a selected decoding step
//! - [`text_panel`]: Scrollable text panels for input/output display
//! - [`settings_panel`]: Settings form for editing configuration
//! - [`list_editor`]: Modal for editing string lists
//! - [`wordlist_manager`]: Modal for managing wordlist files
//! - [`theme_picker`]: Modal for selecting color themes

pub mod list_editor;
pub mod path_viewer;
pub mod settings_panel;
pub mod step_details;
pub mod text_panel;
pub mod theme_picker;
pub mod wordlist_manager;

pub use list_editor::{render_list_editor, ListEditor};
pub use path_viewer::PathViewer;
pub use settings_panel::{render_settings_screen, SettingsPanel};
pub use step_details::render_step_details;
pub use text_panel::{render_text_panel, TextPanel};
pub use theme_picker::ThemePicker;
pub use wordlist_manager::{render_wordlist_manager, WordlistFocus, WordlistManagerWidget};
