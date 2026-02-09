//! Widget components for the Ciphey TUI.
//!
//! This module provides reusable widget components for building the terminal
//! user interface, including:
//!
//! - [`step_details`]: Shows detailed information about a selected decoding step
//! - [`text_panel`]: Scrollable text panels for input/output display
//! - [`settings_panel`]: Settings form for editing configuration
//! - [`list_editor`]: Modal for editing string lists
//! - [`wordlist_manager`]: Modal for managing wordlist files
//! - [`theme_picker`]: Modal for selecting color themes
//! - [`toggle_list_editor`]: Modal for selecting items from a fixed set (decoders/checkers)
//! - [`tree_viewer`]: Birds-eye tree view of the decoder path and all branches

pub mod list_editor;
pub mod settings_panel;
pub mod step_details;
pub mod text_panel;
pub mod theme_picker;
pub mod toggle_list_editor;
pub mod tree_viewer;
pub mod wordlist_manager;

pub use list_editor::{render_list_editor, ListEditor};
pub use settings_panel::{render_settings_screen, SettingsPanel};
pub use step_details::render_step_details;
pub use text_panel::{render_text_panel, TextPanel};
pub use theme_picker::ThemePicker;
pub use toggle_list_editor::{render_toggle_list_editor, ToggleListEditor};
pub use tree_viewer::{TreeNode, TreeViewer};
pub use wordlist_manager::{render_wordlist_manager, WordlistFocus, WordlistManagerWidget};
