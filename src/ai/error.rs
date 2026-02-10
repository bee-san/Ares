//! Error types for the AI module.
//!
//! Provides a unified error type for all AI-related operations including
//! HTTP communication, API responses, and configuration issues.

use std::fmt;

/// Errors that can occur when using AI features.
#[derive(Debug)]
pub enum AiError {
    /// AI features are not configured (missing API URL, key, or model).
    NotConfigured,
    /// AI features are disabled in the config.
    Disabled,
    /// HTTP request failed (network error, DNS resolution, etc.).
    HttpError(String),
    /// The HTTP request timed out (connect or read timeout).
    Timeout,
    /// The API returned HTTP 429 (rate limited).
    RateLimited {
        /// Optional `Retry-After` value in seconds from the response header.
        retry_after: Option<u64>,
    },
    /// The API returned a non-success status code.
    ApiError {
        /// HTTP status code returned by the API.
        status: u16,
        /// Error message from the API response body.
        message: String,
    },
    /// Failed to parse the API response.
    ParseError(String),
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiError::NotConfigured => {
                write!(f, "AI is not configured: missing API URL, key, or model")
            }
            AiError::Disabled => write!(f, "AI features are disabled in the configuration"),
            AiError::HttpError(msg) => write!(f, "HTTP request failed: {}", msg),
            AiError::Timeout => write!(f, "AI request timed out"),
            AiError::RateLimited { retry_after } => {
                if let Some(secs) = retry_after {
                    write!(f, "Rate limited by API (retry after {}s)", secs)
                } else {
                    write!(f, "Rate limited by API")
                }
            }
            AiError::ApiError { status, message } => {
                write!(f, "API error (status {}): {}", status, message)
            }
            AiError::ParseError(msg) => write!(f, "Failed to parse API response: {}", msg),
        }
    }
}

impl std::error::Error for AiError {}

impl From<reqwest::Error> for AiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AiError::Timeout
        } else {
            AiError::HttpError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AiError {
    fn from(err: serde_json::Error) -> Self {
        AiError::ParseError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_not_configured() {
        let err = AiError::NotConfigured;
        assert!(err.to_string().contains("not configured"));
    }

    #[test]
    fn test_display_disabled() {
        let err = AiError::Disabled;
        assert!(err.to_string().contains("disabled"));
    }

    #[test]
    fn test_display_http_error() {
        let err = AiError::HttpError("connection refused".to_string());
        assert!(err.to_string().contains("connection refused"));
    }

    #[test]
    fn test_display_timeout() {
        let err = AiError::Timeout;
        assert!(err.to_string().contains("timed out"));
    }

    #[test]
    fn test_display_rate_limited_with_retry() {
        let err = AiError::RateLimited {
            retry_after: Some(30),
        };
        let display = err.to_string();
        assert!(display.contains("Rate limited"));
        assert!(display.contains("30s"));
    }

    #[test]
    fn test_display_rate_limited_without_retry() {
        let err = AiError::RateLimited { retry_after: None };
        let display = err.to_string();
        assert!(display.contains("Rate limited"));
        assert!(!display.contains("retry after"));
    }

    #[test]
    fn test_display_api_error() {
        let err = AiError::ApiError {
            status: 401,
            message: "Invalid API key".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("401"));
        assert!(display.contains("Invalid API key"));
    }

    #[test]
    fn test_display_parse_error() {
        let err = AiError::ParseError("unexpected token".to_string());
        assert!(err.to_string().contains("unexpected token"));
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<String>("not valid json").unwrap_err();
        let ai_err = AiError::from(json_err);
        assert!(matches!(ai_err, AiError::ParseError(_)));
    }
}
