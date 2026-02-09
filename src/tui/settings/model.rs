//! Settings data model with serialization/deserialization from Config.

use std::collections::HashMap;

use crate::checkers::get_all_checker_names;
use crate::config::Config;
use crate::decoders::get_all_decoder_names;

/// Complete settings model containing all editable fields
#[derive(Debug, Clone)]
pub struct SettingsModel {
    /// All settings organized by section
    pub sections: Vec<SettingsSection>,
}

/// A section grouping related settings
#[derive(Debug, Clone)]
pub struct SettingsSection {
    /// Section name for display
    pub name: &'static str,
    /// Brief description of the section
    pub description: &'static str,
    /// Fields in this section
    pub fields: Vec<SettingField>,
}

/// A single configurable setting field
#[derive(Debug, Clone)]
pub struct SettingField {
    /// Unique identifier matching config field name
    pub id: &'static str,
    /// Display name for the UI
    pub label: &'static str,
    /// Help text describing the setting
    pub description: &'static str,
    /// Type of value this field holds
    pub field_type: FieldType,
    /// Current value
    pub value: SettingValue,
    /// Original value (for detecting changes)
    pub original_value: SettingValue,
}

/// Types of settings fields
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    /// Boolean toggle (on/off)
    Boolean,
    /// Integer with min/max bounds
    Integer {
        /// Minimum allowed value
        min: i64,
        /// Maximum allowed value
        max: i64,
    },
    /// Floating point with min/max bounds
    Float {
        /// Minimum allowed value
        min: f64,
        /// Maximum allowed value
        max: f64,
    },
    /// Free-form string
    String {
        /// Maximum length (None = unlimited)
        max_length: Option<usize>,
    },
    /// File path
    Path {
        /// Whether the path must exist
        must_exist: bool,
    },
    /// RGB color in "r,g,b" format
    RgbColor,
    /// List of strings (opens sub-modal to edit)
    StringList,
    /// Special type for wordlist management (opens sub-modal)
    WordlistManager,
    /// Special type for theme picker (opens sub-modal)
    ThemePicker,
    /// Toggle list for selecting items from a fixed set (opens sub-modal)
    /// Used for decoder/checker selection
    ToggleList {
        /// All available items that can be toggled
        all_items: Vec<String>,
    },
}

/// Value types for settings
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// Floating point value
    Float(f64),
    /// Text string
    Text(String),
    /// Optional text string
    OptionalText(Option<String>),
    /// List of strings
    List(Vec<String>),
    /// Placeholder for wordlist manager (value stored in database)
    WordlistPlaceholder,
}

impl SettingsModel {
    /// Creates a SettingsModel from the current Config
    pub fn from_config(config: &Config) -> Self {
        Self {
            sections: vec![
                // Section 1: General
                SettingsSection {
                    name: "General",
                    description: "Basic application settings",
                    fields: vec![
                        SettingField {
                            id: "verbose",
                            label: "Verbose Level",
                            description: "Logging verbosity (0-3)",
                            field_type: FieldType::Integer { min: 0, max: 3 },
                            value: SettingValue::Int(i64::from(config.verbose)),
                            original_value: SettingValue::Int(i64::from(config.verbose)),
                        },
                        SettingField {
                            id: "timeout",
                            label: "Timeout",
                            description: "Seconds before Ciphey stops searching",
                            field_type: FieldType::Integer { min: 1, max: 500 },
                            value: SettingValue::Int(i64::from(config.timeout)),
                            original_value: SettingValue::Int(i64::from(config.timeout)),
                        },
                        SettingField {
                            id: "top_results",
                            label: "Collect All Results",
                            description: "Gather all plaintexts instead of stopping at first",
                            field_type: FieldType::Boolean,
                            value: SettingValue::Bool(config.top_results),
                            original_value: SettingValue::Bool(config.top_results),
                        },
                        SettingField {
                            id: "api_mode",
                            label: "API Mode",
                            description: "Run in non-interactive API mode",
                            field_type: FieldType::Boolean,
                            value: SettingValue::Bool(config.api_mode),
                            original_value: SettingValue::Bool(config.api_mode),
                        },
                        SettingField {
                            id: "regex",
                            label: "Regex Pattern",
                            description: "Match plaintext against this pattern (crib)",
                            field_type: FieldType::String {
                                max_length: Some(500),
                            },
                            value: SettingValue::OptionalText(config.regex.clone()),
                            original_value: SettingValue::OptionalText(config.regex.clone()),
                        },
                        SettingField {
                            id: "status_message_timeout",
                            label: "Status Timeout",
                            description: "Seconds before status messages auto-clear (0 = never)",
                            field_type: FieldType::Integer { min: 0, max: 300 },
                            value: SettingValue::Int(config.status_message_timeout as i64),
                            original_value: SettingValue::Int(config.status_message_timeout as i64),
                        },
                    ],
                },
                // Section 2: Checkers
                SettingsSection {
                    name: "Checkers",
                    description: "Plaintext detection settings",
                    fields: vec![
                        SettingField {
                            id: "human_checker_on",
                            label: "Human Checker",
                            description: "Ask for confirmation on potential plaintexts",
                            field_type: FieldType::Boolean,
                            value: SettingValue::Bool(config.human_checker_on),
                            original_value: SettingValue::Bool(config.human_checker_on),
                        },
                        SettingField {
                            id: "enhanced_detection",
                            label: "Enhanced Detection",
                            description: "Use AI model for improved accuracy (~40% better)",
                            field_type: FieldType::Boolean,
                            value: SettingValue::Bool(config.enhanced_detection),
                            original_value: SettingValue::Bool(config.enhanced_detection),
                        },
                        SettingField {
                            id: "model_path",
                            label: "Model Path",
                            description: "Path to enhanced detection model file",
                            field_type: FieldType::Path { must_exist: false },
                            value: SettingValue::OptionalText(config.model_path.clone()),
                            original_value: SettingValue::OptionalText(config.model_path.clone()),
                        },
                        // Wordlist management opens a sub-modal
                        SettingField {
                            id: "wordlist_manager",
                            label: "Wordlist Manager",
                            description: "Manage wordlists for plaintext detection",
                            field_type: FieldType::WordlistManager,
                            value: SettingValue::WordlistPlaceholder,
                            original_value: SettingValue::WordlistPlaceholder,
                        },
                    ],
                },
                // Section 3: LemmeKnow Detection
                SettingsSection {
                    name: "LemmeKnow",
                    description: "Encoding/format detection settings",
                    fields: vec![
                        SettingField {
                            id: "lemmeknow_min_rarity",
                            label: "Min Rarity",
                            description: "Minimum rarity threshold (0.0-1.0)",
                            field_type: FieldType::Float { min: 0.0, max: 1.0 },
                            value: SettingValue::Float(f64::from(config.lemmeknow_min_rarity)),
                            original_value: SettingValue::Float(f64::from(
                                config.lemmeknow_min_rarity,
                            )),
                        },
                        SettingField {
                            id: "lemmeknow_max_rarity",
                            label: "Max Rarity",
                            description: "Maximum rarity threshold (0.0-1.0)",
                            field_type: FieldType::Float { min: 0.0, max: 1.0 },
                            value: SettingValue::Float(f64::from(config.lemmeknow_max_rarity)),
                            original_value: SettingValue::Float(f64::from(
                                config.lemmeknow_max_rarity,
                            )),
                        },
                        SettingField {
                            id: "lemmeknow_boundaryless",
                            label: "Boundaryless Mode",
                            description: "Match patterns without word boundaries",
                            field_type: FieldType::Boolean,
                            value: SettingValue::Bool(config.lemmeknow_boundaryless),
                            original_value: SettingValue::Bool(config.lemmeknow_boundaryless),
                        },
                        SettingField {
                            id: "lemmeknow_tags",
                            label: "Include Tags",
                            description: "Tags to include in detection",
                            field_type: FieldType::StringList,
                            value: SettingValue::List(config.lemmeknow_tags.clone()),
                            original_value: SettingValue::List(config.lemmeknow_tags.clone()),
                        },
                        SettingField {
                            id: "lemmeknow_exclude_tags",
                            label: "Exclude Tags",
                            description: "Tags to exclude from detection",
                            field_type: FieldType::StringList,
                            value: SettingValue::List(config.lemmeknow_exclude_tags.clone()),
                            original_value: SettingValue::List(
                                config.lemmeknow_exclude_tags.clone(),
                            ),
                        },
                    ],
                },
                // Section 4: A* Search Tuning
                SettingsSection {
                    name: "Search Tuning",
                    description: "A* algorithm parameters",
                    fields: vec![
                        SettingField {
                            id: "depth_penalty",
                            label: "Depth Penalty",
                            description: "Cost added per depth level (0.0-5.0)",
                            field_type: FieldType::Float { min: 0.0, max: 5.0 },
                            value: SettingValue::Float(f64::from(config.depth_penalty)),
                            original_value: SettingValue::Float(f64::from(config.depth_penalty)),
                        },
                        SettingField {
                            id: "decoder_batch_size",
                            label: "Decoder Batch Size",
                            description: "Decoders per node expansion (1-20)",
                            field_type: FieldType::Integer { min: 1, max: 20 },
                            value: SettingValue::Int(config.decoder_batch_size as i64),
                            original_value: SettingValue::Int(config.decoder_batch_size as i64),
                        },
                    ],
                },
                // Section 5: Color Scheme
                SettingsSection {
                    name: "Themes",
                    description: "Color scheme and appearance",
                    fields: vec![
                        // Theme picker at the top
                        SettingField {
                            id: "theme_preset",
                            label: "Theme Preset",
                            description:
                                "Choose a preset theme (colors below can be customized after)",
                            field_type: FieldType::ThemePicker,
                            value: SettingValue::Text("Custom".to_string()),
                            original_value: SettingValue::Text("Custom".to_string()),
                        },
                        SettingField {
                            id: "color_informational",
                            label: "Informational",
                            description: "Color for status messages",
                            field_type: FieldType::RgbColor,
                            value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("informational")
                                    .cloned()
                                    .unwrap_or_else(|| "255,215,0".to_string()),
                            ),
                            original_value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("informational")
                                    .cloned()
                                    .unwrap_or_else(|| "255,215,0".to_string()),
                            ),
                        },
                        SettingField {
                            id: "color_warning",
                            label: "Warning",
                            description: "Color for warnings",
                            field_type: FieldType::RgbColor,
                            value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("warning")
                                    .cloned()
                                    .unwrap_or_else(|| "255,0,0".to_string()),
                            ),
                            original_value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("warning")
                                    .cloned()
                                    .unwrap_or_else(|| "255,0,0".to_string()),
                            ),
                        },
                        SettingField {
                            id: "color_success",
                            label: "Success",
                            description: "Color for success messages",
                            field_type: FieldType::RgbColor,
                            value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("success")
                                    .cloned()
                                    .unwrap_or_else(|| "0,255,0".to_string()),
                            ),
                            original_value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("success")
                                    .cloned()
                                    .unwrap_or_else(|| "0,255,0".to_string()),
                            ),
                        },
                        SettingField {
                            id: "color_error",
                            label: "Error",
                            description: "Color for errors",
                            field_type: FieldType::RgbColor,
                            value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("error")
                                    .cloned()
                                    .unwrap_or_else(|| "255,0,0".to_string()),
                            ),
                            original_value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("error")
                                    .cloned()
                                    .unwrap_or_else(|| "255,0,0".to_string()),
                            ),
                        },
                        SettingField {
                            id: "color_question",
                            label: "Question",
                            description: "Color for prompts",
                            field_type: FieldType::RgbColor,
                            value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("question")
                                    .cloned()
                                    .unwrap_or_else(|| "255,215,0".to_string()),
                            ),
                            original_value: SettingValue::Text(
                                config
                                    .colourscheme
                                    .get("question")
                                    .cloned()
                                    .unwrap_or_else(|| "255,215,0".to_string()),
                            ),
                        },
                    ],
                },
                // Section 6: Decoders to Run
                SettingsSection {
                    name: "Decoders to Run",
                    description: "Select which decoders are enabled",
                    fields: vec![SettingField {
                        id: "decoders_to_run",
                        label: "Decoders",
                        description: "Select which decoders to use",
                        field_type: FieldType::ToggleList {
                            all_items: get_all_decoder_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        },
                        // If config has empty list, default to all enabled for backwards compat
                        value: SettingValue::List(if config.decoders_to_run.is_empty() {
                            get_all_decoder_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            config.decoders_to_run.clone()
                        }),
                        original_value: SettingValue::List(if config.decoders_to_run.is_empty() {
                            get_all_decoder_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            config.decoders_to_run.clone()
                        }),
                    }],
                },
                // Section 7: Checkers to Run
                SettingsSection {
                    name: "Checkers to Run",
                    description: "Select which checkers are enabled",
                    fields: vec![SettingField {
                        id: "checkers_to_run",
                        label: "Checkers",
                        description: "Select which checkers to use",
                        field_type: FieldType::ToggleList {
                            all_items: get_all_checker_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        },
                        // If config has empty list, default to all enabled for backwards compat
                        value: SettingValue::List(if config.checkers_to_run.is_empty() {
                            get_all_checker_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            config.checkers_to_run.clone()
                        }),
                        original_value: SettingValue::List(if config.checkers_to_run.is_empty() {
                            get_all_checker_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            config.checkers_to_run.clone()
                        }),
                    }],
                },
            ],
        }
    }

    /// Applies the settings model back to a Config struct
    #[allow(clippy::cognitive_complexity)]
    pub fn apply_to_config(&self, config: &mut Config) {
        for section in &self.sections {
            for field in &section.fields {
                match field.id {
                    "verbose" => {
                        if let SettingValue::Int(v) = &field.value {
                            config.verbose = *v as u8;
                        }
                    }
                    "timeout" => {
                        if let SettingValue::Int(v) = &field.value {
                            config.timeout = *v as u32;
                        }
                    }
                    "top_results" => {
                        if let SettingValue::Bool(v) = &field.value {
                            config.top_results = *v;
                        }
                    }
                    "api_mode" => {
                        if let SettingValue::Bool(v) = &field.value {
                            config.api_mode = *v;
                        }
                    }
                    "regex" => {
                        if let SettingValue::OptionalText(v) = &field.value {
                            config.regex = v.clone();
                        }
                    }
                    "human_checker_on" => {
                        if let SettingValue::Bool(v) = &field.value {
                            config.human_checker_on = *v;
                        }
                    }
                    "enhanced_detection" => {
                        if let SettingValue::Bool(v) = &field.value {
                            config.enhanced_detection = *v;
                        }
                    }
                    "model_path" => {
                        if let SettingValue::OptionalText(v) = &field.value {
                            config.model_path = v.clone();
                        }
                    }
                    "lemmeknow_min_rarity" => {
                        if let SettingValue::Float(v) = &field.value {
                            config.lemmeknow_min_rarity = *v as f32;
                        }
                    }
                    "lemmeknow_max_rarity" => {
                        if let SettingValue::Float(v) = &field.value {
                            config.lemmeknow_max_rarity = *v as f32;
                        }
                    }
                    "lemmeknow_boundaryless" => {
                        if let SettingValue::Bool(v) = &field.value {
                            config.lemmeknow_boundaryless = *v;
                        }
                    }
                    "lemmeknow_tags" => {
                        if let SettingValue::List(v) = &field.value {
                            config.lemmeknow_tags = v.clone();
                        }
                    }
                    "lemmeknow_exclude_tags" => {
                        if let SettingValue::List(v) = &field.value {
                            config.lemmeknow_exclude_tags = v.clone();
                        }
                    }
                    "depth_penalty" => {
                        if let SettingValue::Float(v) = &field.value {
                            config.depth_penalty = *v as f32;
                        }
                    }
                    "decoder_batch_size" => {
                        if let SettingValue::Int(v) = &field.value {
                            config.decoder_batch_size = *v as usize;
                        }
                    }
                    "color_informational" => {
                        if let SettingValue::Text(v) = &field.value {
                            config
                                .colourscheme
                                .insert("informational".to_string(), v.clone());
                        }
                    }
                    "color_warning" => {
                        if let SettingValue::Text(v) = &field.value {
                            config.colourscheme.insert("warning".to_string(), v.clone());
                        }
                    }
                    "color_success" => {
                        if let SettingValue::Text(v) = &field.value {
                            config.colourscheme.insert("success".to_string(), v.clone());
                        }
                    }
                    "color_error" => {
                        if let SettingValue::Text(v) = &field.value {
                            config.colourscheme.insert("error".to_string(), v.clone());
                        }
                    }
                    "color_question" => {
                        if let SettingValue::Text(v) = &field.value {
                            config
                                .colourscheme
                                .insert("question".to_string(), v.clone());
                        }
                    }
                    "decoders_to_run" => {
                        if let SettingValue::List(v) = &field.value {
                            config.decoders_to_run = v.clone();
                        }
                    }
                    "checkers_to_run" => {
                        if let SettingValue::List(v) = &field.value {
                            config.checkers_to_run = v.clone();
                        }
                    }
                    "status_message_timeout" => {
                        if let SettingValue::Int(v) = &field.value {
                            config.status_message_timeout = *v as u64;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Checks if any fields have been modified from their original values
    pub fn has_changes(&self) -> bool {
        self.sections
            .iter()
            .any(|s| s.fields.iter().any(|f| f.value != f.original_value))
    }

    /// Gets a field by its ID
    pub fn get_field(&self, id: &str) -> Option<&SettingField> {
        self.sections
            .iter()
            .flat_map(|s| s.fields.iter())
            .find(|f| f.id == id)
    }

    /// Gets a mutable field by its ID
    pub fn get_field_mut(&mut self, id: &str) -> Option<&mut SettingField> {
        self.sections
            .iter_mut()
            .flat_map(|s| s.fields.iter_mut())
            .find(|f| f.id == id)
    }

    /// Gets the field at a specific section and field index
    pub fn get_field_at(&self, section_idx: usize, field_idx: usize) -> Option<&SettingField> {
        self.sections
            .get(section_idx)
            .and_then(|s| s.fields.get(field_idx))
    }

    /// Gets a mutable field at a specific section and field index
    pub fn get_field_at_mut(
        &mut self,
        section_idx: usize,
        field_idx: usize,
    ) -> Option<&mut SettingField> {
        self.sections
            .get_mut(section_idx)
            .and_then(|s| s.fields.get_mut(field_idx))
    }

    /// Gets the number of fields in a section
    pub fn field_count(&self, section_idx: usize) -> usize {
        self.sections
            .get(section_idx)
            .map(|s| s.fields.len())
            .unwrap_or(0)
    }

    /// Gets the number of sections
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Validates all fields and returns a map of field_id -> error message
    pub fn validate_all(&self) -> HashMap<String, String> {
        let mut errors = HashMap::new();
        for section in &self.sections {
            for field in &section.fields {
                if let Err(e) = super::validation::validate_field(field) {
                    errors.insert(field.id.to_string(), e.to_string());
                }
            }
        }
        errors
    }
}

impl SettingField {
    /// Returns true if this field is a boolean type
    pub fn is_boolean(&self) -> bool {
        matches!(self.field_type, FieldType::Boolean)
    }

    /// Returns true if this field opens a sub-modal
    pub fn opens_modal(&self) -> bool {
        matches!(
            self.field_type,
            FieldType::StringList
                | FieldType::WordlistManager
                | FieldType::ThemePicker
                | FieldType::ToggleList { .. }
        )
    }

    /// Gets the current value as a string for display
    pub fn display_value(&self) -> String {
        match &self.value {
            SettingValue::Bool(v) => {
                if *v {
                    "Enabled".to_string()
                } else {
                    "Disabled".to_string()
                }
            }
            SettingValue::Int(v) => v.to_string(),
            SettingValue::Float(v) => format!("{:.2}", v),
            SettingValue::Text(v) => {
                if v.is_empty() {
                    "(not set)".to_string()
                } else {
                    v.clone()
                }
            }
            SettingValue::OptionalText(v) => v.clone().unwrap_or_else(|| "(not set)".to_string()),
            SettingValue::List(v) => {
                // Check if this is a ToggleList field
                if let FieldType::ToggleList { all_items } = &self.field_type {
                    if v.is_empty() {
                        "0 enabled (nothing will run)".to_string()
                    } else if v.len() == all_items.len() {
                        format!("All {} enabled", all_items.len())
                    } else {
                        format!("{} of {} enabled", v.len(), all_items.len())
                    }
                } else if v.is_empty() {
                    "(empty)".to_string()
                } else if v.len() <= 3 {
                    v.join(", ")
                } else {
                    format!("{} items", v.len())
                }
            }
            SettingValue::WordlistPlaceholder => "[Manage Wordlists]".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_model_from_default_config() {
        let config = Config::default();
        let model = SettingsModel::from_config(&config);

        assert_eq!(model.section_count(), 7);
        assert!(!model.has_changes());
    }

    #[test]
    fn test_settings_model_detects_changes() {
        let config = Config::default();
        let mut model = SettingsModel::from_config(&config);

        // Initially no changes
        assert!(!model.has_changes());

        // Modify a field
        if let Some(field) = model.get_field_mut("timeout") {
            field.value = SettingValue::Int(10);
        }

        // Now has changes
        assert!(model.has_changes());
    }

    #[test]
    fn test_apply_to_config() {
        let mut config = Config::default();
        let mut model = SettingsModel::from_config(&config);

        // Modify timeout
        if let Some(field) = model.get_field_mut("timeout") {
            field.value = SettingValue::Int(100);
        }

        // Apply changes
        model.apply_to_config(&mut config);

        assert_eq!(config.timeout, 100);
    }

    #[test]
    fn test_get_field_by_id() {
        let config = Config::default();
        let model = SettingsModel::from_config(&config);

        let field = model.get_field("timeout");
        assert!(field.is_some());
        assert_eq!(field.unwrap().label, "Timeout");

        let missing = model.get_field("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_display_value() {
        let field = SettingField {
            id: "test",
            label: "Test",
            description: "Test field",
            field_type: FieldType::Boolean,
            value: SettingValue::Bool(true),
            original_value: SettingValue::Bool(true),
        };

        assert_eq!(field.display_value(), "Enabled");
    }
}
