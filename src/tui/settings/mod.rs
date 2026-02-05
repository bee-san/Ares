//! Settings model and field definitions for the TUI settings panel.
//!
//! This module provides the data structures and logic for managing
//! application settings in the TUI, including:
//!
//! - Settings model with all configurable fields
//! - Field type definitions (boolean, integer, float, string, etc.)
//! - Validation logic for each field type
//! - Serialization to/from Config struct

mod model;
pub mod validation;

pub use model::{FieldType, SettingField, SettingValue, SettingsModel, SettingsSection};
pub use validation::{parse_input, validate_field, ValidationError};
