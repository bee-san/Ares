//! AI-powered language detection checker.
//!
//! This checker uses the AI module to detect whether decoded text is in a
//! foreign (non-English) language. If foreign text is detected with sufficient
//! confidence, it attempts to translate it to English.
//!
//! This checker is standalone and not wired into Athena's pipeline by default.
//! It can be invoked manually or added to the pipeline in a future update.

use crate::checkers::checker_result::CheckResult;
use gibberish_or_not::Sensitivity;
use lemmeknow::Identifier;

use crate::checkers::checker_type::{Check, Checker};

/// Detects foreign language text and translates it to English using AI.
pub struct LanguageChecker;

/// Implementation of the Check trait for LanguageChecker.
///
/// This checker:
/// 1. Verifies AI features are configured and enabled
/// 2. Calls `ai::detect_language()` on the input text
/// 3. If foreign language is detected with high or medium confidence,
///    calls `ai::translate()` to get the English translation
/// 4. Returns the translated text as the identified plaintext
impl Check for Checker<LanguageChecker> {
    fn new() -> Self {
        Checker {
            name: "Language Detection",
            description: "Detects foreign language text and translates to English using AI",
            link: "https://github.com/bee-san/ciphey",
            tags: vec!["ai", "language", "translation"],
            expected_runtime: 2.0,
            popularity: 0.5,
            lemmeknow_config: Identifier::default(),
            sensitivity: Sensitivity::Medium,
            enhanced_detector: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        let mut result = CheckResult::new(self);

        // AI must be configured and enabled
        if !crate::ai::is_ai_configured() {
            return result;
        }

        // Skip very short text (unlikely to be meaningful foreign language).
        // Use char count for Unicode safety, and require at least 20 chars
        // since language detection on very short strings is unreliable.
        if text.chars().count() < 20 {
            return result;
        }

        // Detect language
        let detection = match crate::ai::detect_language(text) {
            Ok(det) => det,
            Err(_) => return result,
        };

        // Only proceed if foreign language detected with sufficient confidence
        if !detection.is_foreign_language {
            return result;
        }

        let confidence = detection.confidence.as_deref().unwrap_or("low");
        if confidence == "low" {
            return result;
        }

        let language = detection.detected_language.as_deref().unwrap_or("Unknown");

        // Attempt translation with language description
        match crate::ai::translate_with_description(text, language) {
            Ok(trans_result) => {
                result.is_identified = true;
                result.text = trans_result.translation;
                result.description = format!(
                    "Foreign language detected: {} (confidence: {}).\n\n{}\n\nTranslated to English.",
                    language, confidence, trans_result.language_description
                );
            }
            Err(_) => {
                // Translation failed but we still detected the language
                // Mark as identified with original text
                result.is_identified = true;
                result.text = text.to_string();
                result.description = if language == "Unknown" {
                    format!(
                        "Foreign language detected (confidence: {}). Translation unavailable.",
                        confidence
                    )
                } else {
                    format!(
                        "Foreign language detected: {} (confidence: {}). Translation failed.",
                        language, confidence
                    )
                };
            }
        }

        result
    }

    fn with_sensitivity(mut self, sensitivity: Sensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    fn get_sensitivity(&self) -> Sensitivity {
        self.sensitivity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_valid_checker() {
        let checker = Checker::<LanguageChecker>::new();
        assert_eq!(checker.name, "Language Detection");
        assert!(checker.tags.contains(&"ai"));
        assert!(checker.tags.contains(&"language"));
    }

    #[test]
    fn test_check_returns_not_identified_when_ai_not_configured() {
        // AI is not configured by default in tests
        let checker = Checker::<LanguageChecker>::new();
        let result = checker.check("bonjour le monde");
        // Without AI configured, should return not identified
        assert!(!result.is_identified);
    }

    #[test]
    fn test_check_short_text_returns_not_identified() {
        let checker = Checker::<LanguageChecker>::new();
        let result = checker.check("ab");
        assert!(!result.is_identified);
    }

    #[test]
    fn test_default_sensitivity_is_medium() {
        let checker = Checker::<LanguageChecker>::new();
        assert!(matches!(checker.get_sensitivity(), Sensitivity::Medium));
    }

    #[test]
    fn test_with_sensitivity_changes_sensitivity() {
        let checker = Checker::<LanguageChecker>::new().with_sensitivity(Sensitivity::Low);
        assert!(matches!(checker.get_sensitivity(), Sensitivity::Low));
    }
}
