//! TUI components for Ciphey.
//!
//! This module provides terminal user interface components built with Ratatui.

pub mod app;
pub mod colors;
pub mod human_checker_bridge;
pub mod input;
pub mod multiline_text_input;
pub mod settings;
pub mod setup_wizard;
pub mod spinner;
pub mod text_input;
pub mod ui;
pub mod widgets;

mod run;

pub use run::run_tui;
pub use setup_wizard::run_setup_wizard;
