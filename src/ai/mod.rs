//! AI module for Ciphey.
//!
//! This module provides AI-powered features using OpenAI-compatible API endpoints.
//! It supports any provider that implements the OpenAI chat completions API
//! (OpenAI, Ollama, LM Studio, Azure OpenAI, etc.).
//!
//! # Features
//!
//! - **Step explanation**: Explain how a decoder transforms input to output
//! - **Language detection**: Detect if text is in a foreign language
//! - **Translation**: Translate foreign language text to English
//!
//! # Configuration
//!
//! AI features require the following config fields to be set:
//! - `ai_enabled`: Must be `true`
//! - `ai_api_url`: The API base URL (e.g., `https://api.openai.com/v1`)
//! - `ai_api_key`: The API key for authentication
//! - `ai_model`: The model name (e.g., `gpt-4o-mini`)
//!
//! # Caching
//!
//! AI responses are cached in SQLite (`ai_cache` table) to avoid redundant API
//! calls. The cache key includes the function type, input parameters, and model
//! name, so switching models automatically invalidates the cache.

/// HTTP client for OpenAI-compatible API endpoints.
pub mod client;
/// Error types for AI operations.
pub mod error;
/// Prompt templates for each AI use case.
pub mod prompts;

use client::AiClient;
use error::AiError;

use crate::config::get_config;
use crate::storage::database::{insert_ai_cache, read_ai_cache};

/// Result of a language detection request.
#[derive(Debug, Clone)]
pub struct LanguageDetectionResult {
    /// Whether the text appears to be in a non-English language.
    pub is_foreign_language: bool,
    /// The detected language name (e.g., "French", "Japanese"), if applicable.
    pub detected_language: Option<String>,
    /// Confidence level of the detection: "high", "medium", or "low".
    pub confidence: Option<String>,
}

/// Computes a deterministic cache key from the function type, model, and input parameters.
///
/// The key format is `function_type|model|param1|param2|...` which provides
/// uniqueness across different functions and model configurations.
fn compute_cache_key(function_type: &str, model: &str, params: &[&str]) -> String {
    let mut key = format!("{}|{}", function_type, model);
    for param in params {
        key.push('|');
        key.push_str(param);
    }
    key
}

/// Checks whether AI features are properly configured and enabled.
///
/// Returns `true` if `ai_enabled` is `true` and all required fields
/// (API URL, API key, model) are set and non-empty.
pub fn is_ai_configured() -> bool {
    let config = get_config();
    if !config.ai_enabled {
        return false;
    }
    AiClient::from_config(config).is_some()
}

/// Explains how a decoder step transforms its input into output.
///
/// Uses the AI model to generate a human-readable explanation of the
/// encoding/cipher algorithm and how it was applied to the specific input.
///
/// # Arguments
///
/// * `decoder_name` - The name of the decoder (e.g., "Base64", "Caesar")
/// * `input` - The input text to the decoder step
/// * `output` - The output text from the decoder step
/// * `key` - Optional key used by the decoder (e.g., shift value for Caesar)
///
/// # Errors
///
/// Returns `AiError::Disabled` if AI is not enabled, `AiError::NotConfigured`
/// if configuration is incomplete, or other variants for HTTP/API errors.
pub fn explain_step(
    decoder_name: &str,
    input: &str,
    output: &str,
    key: Option<&str>,
) -> Result<String, AiError> {
    let config = get_config();
    if !config.ai_enabled {
        return Err(AiError::Disabled);
    }
    let client = AiClient::from_config(config).ok_or(AiError::NotConfigured)?;

    // Check cache first
    let key_str = key.unwrap_or("");
    let cache_key = compute_cache_key(
        "explain_step",
        client.model(),
        &[decoder_name, input, output, key_str],
    );
    if let Ok(Some(cached)) = read_ai_cache(&cache_key) {
        return Ok(cached.response);
    }

    let messages = prompts::build_explain_step_prompt(decoder_name, input, output, key);
    let response = client.chat_completion(messages, Some(0.3))?;

    // Store in cache (best-effort, ignore errors)
    let _ = insert_ai_cache(&cache_key, "explain_step", &response, client.model());

    Ok(response)
}

/// Detects whether text is in a foreign (non-English) language.
///
/// Returns a `LanguageDetectionResult` indicating whether the text is
/// foreign, what language it appears to be, and the confidence level.
///
/// # Arguments
///
/// * `text` - The text to analyze
///
/// # Errors
///
/// Returns `AiError::Disabled` if AI is not enabled, `AiError::NotConfigured`
/// if configuration is incomplete, or other variants for HTTP/API errors.
pub fn detect_language(text: &str) -> Result<LanguageDetectionResult, AiError> {
    let config = get_config();
    if !config.ai_enabled {
        return Err(AiError::Disabled);
    }
    let client = AiClient::from_config(config).ok_or(AiError::NotConfigured)?;

    // Check cache first
    let cache_key = compute_cache_key("detect_language", client.model(), &[text]);
    if let Ok(Some(cached)) = read_ai_cache(&cache_key) {
        return parse_language_detection_response(&cached.response);
    }

    let messages = prompts::build_detect_language_prompt(text);
    let response = client.chat_completion(messages, Some(0.1))?;

    // Store in cache (best-effort, ignore errors)
    let _ = insert_ai_cache(&cache_key, "detect_language", &response, client.model());

    // Parse the JSON response from the model
    parse_language_detection_response(&response)
}

/// Translates text from a detected source language into English.
///
/// This should be called after `detect_language` confirms the text is
/// in a foreign language.
///
/// # Arguments
///
/// * `text` - The text to translate
/// * `source_language` - The detected source language (e.g., "French", "Japanese")
///
/// # Errors
///
/// Returns `AiError::Disabled` if AI is not enabled, `AiError::NotConfigured`
/// if configuration is incomplete, or other variants for HTTP/API errors.
pub fn translate(text: &str, source_language: &str) -> Result<String, AiError> {
    let config = get_config();
    if !config.ai_enabled {
        return Err(AiError::Disabled);
    }
    let client = AiClient::from_config(config).ok_or(AiError::NotConfigured)?;

    // Check cache first
    let cache_key = compute_cache_key("translate", client.model(), &[text, source_language]);
    if let Ok(Some(cached)) = read_ai_cache(&cache_key) {
        return Ok(cached.response);
    }

    let messages = prompts::build_translate_prompt(text, source_language);
    let response = client.chat_completion(messages, Some(0.3))?;

    // Store in cache (best-effort, ignore errors)
    let _ = insert_ai_cache(&cache_key, "translate", &response, client.model());

    Ok(response)
}

/// Answers a user's question about a specific decoder step.
///
/// Uses the AI model to answer the question with full step context.
/// Responses are NOT cached (always fresh) since questions are freeform.
///
/// # Arguments
///
/// * `question` - The user's freeform question
/// * `decoder_name` - The name of the decoder
/// * `input` - The input text to the decoder step
/// * `output` - The output text from the decoder step
/// * `key` - Optional key used by the decoder
/// * `description` - Short description of the decoder
/// * `link` - Reference link for the decoder
///
/// # Errors
///
/// Returns `AiError::Disabled` if AI is not enabled, `AiError::NotConfigured`
/// if configuration is incomplete, or other variants for HTTP/API errors.
pub fn ask_about_step(
    question: &str,
    decoder_name: &str,
    input: &str,
    output: &str,
    key: Option<&str>,
    description: &str,
    link: &str,
) -> Result<String, AiError> {
    let config = get_config();
    if !config.ai_enabled {
        return Err(AiError::Disabled);
    }
    let client = AiClient::from_config(config).ok_or(AiError::NotConfigured)?;

    let messages = prompts::build_ask_about_step_prompt(
        question,
        decoder_name,
        input,
        output,
        key,
        description,
        link,
    );
    // Higher temperature for conversational responses
    client.chat_completion(messages, Some(0.7))
}

/// Parses the JSON response from the language detection prompt.
fn parse_language_detection_response(response: &str) -> Result<LanguageDetectionResult, AiError> {
    // The model should return a JSON object, but it might include markdown code fences
    let json_str = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: serde_json::Value = serde_json::from_str(json_str)?;

    let is_foreign = parsed
        .get("is_foreign_language")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let detected_language = parsed
        .get("detected_language")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let confidence = parsed
        .get("confidence")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(LanguageDetectionResult {
        is_foreign_language: is_foreign,
        detected_language,
        confidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_cache_key_basic() {
        let key = compute_cache_key(
            "explain_step",
            "gpt-4o-mini",
            &["Base64", "input", "output", ""],
        );
        assert_eq!(key, "explain_step|gpt-4o-mini|Base64|input|output|");
    }

    #[test]
    fn test_compute_cache_key_different_models() {
        let key1 = compute_cache_key("explain_step", "gpt-4o-mini", &["Base64", "in", "out"]);
        let key2 = compute_cache_key("explain_step", "gpt-4o", &["Base64", "in", "out"]);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_compute_cache_key_different_functions() {
        let key1 = compute_cache_key("explain_step", "gpt-4o-mini", &["text"]);
        let key2 = compute_cache_key("detect_language", "gpt-4o-mini", &["text"]);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_compute_cache_key_different_params() {
        let key1 = compute_cache_key("translate", "model", &["hello", "French"]);
        let key2 = compute_cache_key("translate", "model", &["world", "French"]);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_parse_language_detection_foreign() {
        let response =
            r#"{"is_foreign_language": true, "detected_language": "French", "confidence": "high"}"#;
        let result = parse_language_detection_response(response).unwrap();
        assert!(result.is_foreign_language);
        assert_eq!(result.detected_language.as_deref(), Some("French"));
        assert_eq!(result.confidence.as_deref(), Some("high"));
    }

    #[test]
    fn test_parse_language_detection_english() {
        let response =
            r#"{"is_foreign_language": false, "detected_language": null, "confidence": "high"}"#;
        let result = parse_language_detection_response(response).unwrap();
        assert!(!result.is_foreign_language);
        assert!(result.detected_language.is_none());
    }

    #[test]
    fn test_parse_language_detection_with_code_fences() {
        let response = "```json\n{\"is_foreign_language\": true, \"detected_language\": \"Spanish\", \"confidence\": \"medium\"}\n```";
        let result = parse_language_detection_response(response).unwrap();
        assert!(result.is_foreign_language);
        assert_eq!(result.detected_language.as_deref(), Some("Spanish"));
    }

    #[test]
    fn test_parse_language_detection_invalid_json() {
        let response = "This is not JSON";
        let result = parse_language_detection_response(response);
        assert!(result.is_err());
    }
}
