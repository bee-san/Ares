//! Unified error types for ciphey.
//!
//! This module provides a central `CipheyError` enum that covers the main
//! error categories across the codebase, enabling consistent `Result`-based
//! error propagation with the `?` operator.

use std::fmt;

/// Central error type for ciphey operations.
#[derive(Debug)]
pub enum CipheyError {
    /// I/O errors (file operations, stdin/stdout).
    Io(std::io::Error),
    /// Configuration errors (parsing, serialization, missing values).
    Config(String),
    /// Database errors (SQLite operations).
    Database(rusqlite::Error),
    /// Serialization/deserialization errors (TOML, JSON).
    Serialization(String),
    /// Home directory not found.
    HomeNotFound,
}

impl fmt::Display for CipheyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipheyError::Io(e) => write!(f, "I/O error: {}", e),
            CipheyError::Config(msg) => write!(f, "Configuration error: {}", msg),
            CipheyError::Database(e) => write!(f, "Database error: {}", e),
            CipheyError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            CipheyError::HomeNotFound => write!(f, "Could not find home directory"),
        }
    }
}

impl std::error::Error for CipheyError {}

impl From<std::io::Error> for CipheyError {
    fn from(e: std::io::Error) -> Self {
        CipheyError::Io(e)
    }
}

impl From<rusqlite::Error> for CipheyError {
    fn from(e: rusqlite::Error) -> Self {
        CipheyError::Database(e)
    }
}

impl From<toml::ser::Error> for CipheyError {
    fn from(e: toml::ser::Error) -> Self {
        CipheyError::Serialization(e.to_string())
    }
}

impl From<toml::de::Error> for CipheyError {
    fn from(e: toml::de::Error) -> Self {
        CipheyError::Config(e.to_string())
    }
}

impl From<serde_json::Error> for CipheyError {
    fn from(e: serde_json::Error) -> Self {
        CipheyError::Serialization(e.to_string())
    }
}