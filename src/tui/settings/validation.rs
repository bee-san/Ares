//! Validation logic for settings fields.

use std::fmt;
use std::path::Path;

use super::model::{FieldType, SettingField, SettingValue};

/// Error type for field validation failures
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error message
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

impl ValidationError {
    /// Creates a new validation error with the given message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Validates a setting field based on its type and value
///
/// # Arguments
///
/// * `field` - The field to validate
///
/// # Returns
///
/// Ok(()) if valid, Err(ValidationError) if invalid
pub fn validate_field(field: &SettingField) -> Result<(), ValidationError> {
    match (&field.field_type, &field.value) {
        // Boolean validation - always valid
        (FieldType::Boolean, SettingValue::Bool(_)) => Ok(()),

        // Integer validation - check bounds
        (FieldType::Integer { min, max }, SettingValue::Int(v)) => {
            if *v < *min {
                Err(ValidationError::new(format!(
                    "Value must be at least {}",
                    min
                )))
            } else if *v > *max {
                Err(ValidationError::new(format!(
                    "Value must be at most {}",
                    max
                )))
            } else {
                Ok(())
            }
        }

        // Float validation - check bounds
        (FieldType::Float { min, max }, SettingValue::Float(v)) => {
            if *v < *min {
                Err(ValidationError::new(format!(
                    "Value must be at least {:.2}",
                    min
                )))
            } else if *v > *max {
                Err(ValidationError::new(format!(
                    "Value must be at most {:.2}",
                    max
                )))
            } else {
                Ok(())
            }
        }

        // String validation - check max length
        (FieldType::String { max_length }, SettingValue::Text(v)) => {
            if let Some(max) = max_length {
                if v.len() > *max {
                    return Err(ValidationError::new(format!(
                        "Maximum {} characters allowed",
                        max
                    )));
                }
            }
            Ok(())
        }

        // Optional string validation
        (FieldType::String { max_length }, SettingValue::OptionalText(v)) => {
            if let Some(text) = v {
                if let Some(max) = max_length {
                    if text.len() > *max {
                        return Err(ValidationError::new(format!(
                            "Maximum {} characters allowed",
                            max
                        )));
                    }
                }
            }
            Ok(())
        }

        // Path validation - optionally check existence
        (FieldType::Path { must_exist }, SettingValue::OptionalText(v)) => {
            if let Some(path_str) = v {
                if !path_str.is_empty() && *must_exist {
                    let path = Path::new(path_str);
                    if !path.exists() {
                        return Err(ValidationError::new("File does not exist"));
                    }
                }
            }
            Ok(())
        }

        (FieldType::Path { must_exist }, SettingValue::Text(v)) => {
            if !v.is_empty() && *must_exist {
                let path = Path::new(v);
                if !path.exists() {
                    return Err(ValidationError::new("File does not exist"));
                }
            }
            Ok(())
        }

        // RGB color validation - check format "r,g,b"
        (FieldType::RgbColor, SettingValue::Text(v)) => validate_rgb_color(v),

        // String list validation - always valid (individual items could be validated)
        (FieldType::StringList, SettingValue::List(_)) => Ok(()),

        // Toggle list validation - always valid (items are from a fixed set)
        (FieldType::ToggleList { .. }, SettingValue::List(_)) => Ok(()),

        // Wordlist manager placeholder - always valid
        (FieldType::WordlistManager, SettingValue::WordlistPlaceholder) => Ok(()),

        // Type mismatch
        _ => Err(ValidationError::new("Invalid value type for field")),
    }
}

/// Validates an RGB color string in "r,g,b" format
fn validate_rgb_color(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new("Color cannot be empty"));
    }

    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 3 {
        return Err(ValidationError::new(
            "Format must be r,g,b (e.g., 255,128,0)",
        ));
    }

    for (i, part) in parts.iter().enumerate() {
        let component_name = match i {
            0 => "Red",
            1 => "Green",
            2 => "Blue",
            _ => "Component",
        };

        match part.trim().parse::<u8>() {
            Ok(_) => {}
            Err(_) => {
                return Err(ValidationError::new(format!(
                    "{} must be 0-255",
                    component_name
                )));
            }
        }
    }

    Ok(())
}

/// Parses a string into the appropriate SettingValue based on field type
///
/// # Arguments
///
/// * `input` - The string to parse
/// * `field_type` - The expected field type
///
/// # Returns
///
/// Ok(SettingValue) if parsing succeeds, Err(ValidationError) if it fails
pub fn parse_input(input: &str, field_type: &FieldType) -> Result<SettingValue, ValidationError> {
    match field_type {
        FieldType::Boolean => {
            let lower = input.to_lowercase();
            match lower.as_str() {
                "true" | "yes" | "1" | "on" | "enabled" => Ok(SettingValue::Bool(true)),
                "false" | "no" | "0" | "off" | "disabled" => Ok(SettingValue::Bool(false)),
                _ => Err(ValidationError::new("Enter 'true' or 'false'")),
            }
        }

        FieldType::Integer { min, max } => match input.trim().parse::<i64>() {
            Ok(v) => {
                if v < *min || v > *max {
                    Err(ValidationError::new(format!(
                        "Value must be between {} and {}",
                        min, max
                    )))
                } else {
                    Ok(SettingValue::Int(v))
                }
            }
            Err(_) => Err(ValidationError::new("Enter a valid integer")),
        },

        FieldType::Float { min, max } => match input.trim().parse::<f64>() {
            Ok(v) => {
                if v < *min || v > *max {
                    Err(ValidationError::new(format!(
                        "Value must be between {:.2} and {:.2}",
                        min, max
                    )))
                } else {
                    Ok(SettingValue::Float(v))
                }
            }
            Err(_) => Err(ValidationError::new("Enter a valid number")),
        },

        FieldType::String { .. } => {
            if input.is_empty() {
                Ok(SettingValue::OptionalText(None))
            } else {
                Ok(SettingValue::OptionalText(Some(input.to_string())))
            }
        }

        FieldType::Path { .. } => {
            if input.is_empty() {
                Ok(SettingValue::OptionalText(None))
            } else {
                Ok(SettingValue::OptionalText(Some(input.to_string())))
            }
        }

        FieldType::RgbColor => {
            validate_rgb_color(input)?;
            Ok(SettingValue::Text(input.to_string()))
        }

        FieldType::StringList => {
            // For now, parse as comma-separated
            let items: Vec<String> = input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok(SettingValue::List(items))
        }

        FieldType::WordlistManager => {
            // This shouldn't be called for WordlistManager
            Ok(SettingValue::WordlistPlaceholder)
        }

        FieldType::ThemePicker => {
            // This shouldn't be called for ThemePicker
            Ok(SettingValue::Text("Custom".to_string()))
        }

        FieldType::ToggleList { .. } => {
            // Toggle list is handled by the modal, this shouldn't be called
            // But return the input parsed as a comma-separated list for safety
            let items: Vec<String> = input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok(SettingValue::List(items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_in_range() {
        let field = SettingField {
            id: "test",
            label: "Test",
            description: "Test field",
            field_type: FieldType::Integer { min: 0, max: 10 },
            value: SettingValue::Int(5),
            original_value: SettingValue::Int(5),
        };

        assert!(validate_field(&field).is_ok());
    }

    #[test]
    fn test_validate_integer_out_of_range() {
        let field = SettingField {
            id: "test",
            label: "Test",
            description: "Test field",
            field_type: FieldType::Integer { min: 0, max: 10 },
            value: SettingValue::Int(15),
            original_value: SettingValue::Int(5),
        };

        assert!(validate_field(&field).is_err());
    }

    #[test]
    fn test_validate_rgb_color_valid() {
        assert!(validate_rgb_color("255,128,0").is_ok());
        assert!(validate_rgb_color("0,0,0").is_ok());
        assert!(validate_rgb_color("255,255,255").is_ok());
    }

    #[test]
    fn test_validate_rgb_color_invalid() {
        assert!(validate_rgb_color("").is_err());
        assert!(validate_rgb_color("256,0,0").is_err());
        assert!(validate_rgb_color("255").is_err());
        assert!(validate_rgb_color("a,b,c").is_err());
    }

    #[test]
    fn test_parse_integer() {
        let result = parse_input("42", &FieldType::Integer { min: 0, max: 100 });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SettingValue::Int(42));
    }

    #[test]
    fn test_parse_float() {
        let result = parse_input(
            "3.14",
            &FieldType::Float {
                min: 0.0,
                max: 10.0,
            },
        );
        assert!(result.is_ok());
        if let SettingValue::Float(v) = result.unwrap() {
            assert!((v - 3.14).abs() < 0.001);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            parse_input("true", &FieldType::Boolean).unwrap(),
            SettingValue::Bool(true)
        );
        assert_eq!(
            parse_input("false", &FieldType::Boolean).unwrap(),
            SettingValue::Bool(false)
        );
        assert_eq!(
            parse_input("yes", &FieldType::Boolean).unwrap(),
            SettingValue::Bool(true)
        );
        assert_eq!(
            parse_input("no", &FieldType::Boolean).unwrap(),
            SettingValue::Bool(false)
        );
    }
}
