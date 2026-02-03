//! Widget components for the Ciphey TUI.
//!
//! This module provides reusable widget components for building the terminal
//! user interface, including:
//!
//! - [`path_viewer`]: Visualizes the decoder chain as connected boxes
//! - [`step_details`]: Shows detailed information about a selected decoding step
//! - [`text_panel`]: Scrollable text panels for input/output display

pub mod path_viewer;
pub mod step_details;
pub mod text_panel;

pub use path_viewer::PathViewer;
pub use step_details::render_step_details;
pub use text_panel::{render_text_panel, TextPanel};
