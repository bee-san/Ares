//! Prompt templates for AI-powered features.
//!
//! This module contains the prompt builders for each AI use case:
//! - Explaining decoder steps (how a specific decoder transforms input to output)
//! - Detecting the language of a piece of text
//! - Translating foreign language text into English

use super::client::ChatMessage;

/// Builds the prompt messages for explaining how a decoder step works.
///
/// The AI will explain the algorithm/encoding used by the decoder, how the
/// specific input is transformed into the output, and (if applicable) how
/// the key was used.
///
/// # Arguments
///
/// * `decoder_name` - The name of the decoder (e.g., "Base64", "Caesar")
/// * `input` - The input text to the decoder step
/// * `output` - The output text from the decoder step
/// * `key` - Optional key used by the decoder (e.g., shift value for Caesar)
pub fn build_explain_step_prompt(
    decoder_name: &str,
    input: &str,
    output: &str,
    key: Option<&str>,
) -> Vec<ChatMessage> {
    let system_prompt = "\
You are a concise cryptography expert. Explain ONLY the specific transformation shown.

Your answer must be:
1. One sentence about what this decoder does
2. Exactly how THIS input became THIS output (be concrete, reference the actual data)
3. If a key was used, briefly mention its role

Keep it under 75 words total. Be direct. Focus on the actual data transformation, not general theory.";

    let key_section = if let Some(key_val) = key {
        format!("\nKey: {}", key_val)
    } else {
        String::new()
    };

    let user_content = format!(
        "Decoder: {}\nInput: {}\nOutput: {}{}",
        decoder_name, input, output, key_section
    );

    vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(&user_content),
    ]
}

/// Truncates text for display in prompts, ensuring we don't send huge payloads.
fn truncate_for_prompt(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max_len).collect::<String>())
    }
}

/// Builds the prompt messages for answering a user question about a decoder step.
///
/// Provides full step context (decoder name, input, output, key, description, link)
/// so the AI can give a specific, accurate answer.
///
/// # Arguments
///
/// * `question` - The user's question
/// * `decoder_name` - The name of the decoder
/// * `input` - The input text to the decoder step
/// * `output` - The output text from the decoder step
/// * `key` - Optional key used by the decoder
/// * `description` - Short description of the decoder
/// * `link` - Reference link for the decoder
pub fn build_ask_about_step_prompt(
    question: &str,
    decoder_name: &str,
    input: &str,
    output: &str,
    key: Option<&str>,
    description: &str,
    link: &str,
) -> Vec<ChatMessage> {
    let system_prompt = "\
You are an expert in cryptography, encoding, and decoding. \
Answer the user's question about this decoding step accurately and clearly. \
Use the provided context to give specific answers. \
Keep responses concise but thorough (under 300 words).";

    let key_str = key.unwrap_or("N/A");
    let context = format!(
        "Context:\n\
         Decoder: {}\n\
         Description: {}\n\
         Input: {} ({} chars)\n\
         Output: {} ({} chars)\n\
         Key: {}\n\
         Reference: {}",
        decoder_name,
        description,
        truncate_for_prompt(input, 200),
        input.chars().count(),
        truncate_for_prompt(output, 200),
        output.chars().count(),
        key_str,
        link
    );

    let user_content = format!("{}\n\nQuestion: {}", context, question);

    vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(&user_content),
    ]
}

/// Builds the prompt messages for detecting whether text is a foreign language.
///
/// The AI will determine if the text is in a non-English language and identify
/// which language it is. Returns a structured JSON response for easy parsing.
///
/// # Arguments
///
/// * `text` - The text to analyze for language detection
pub fn build_detect_language_prompt(text: &str) -> Vec<ChatMessage> {
    let system_prompt = "\
You are a language detection expert. Your job is to determine if a given text \
is written in a foreign (non-English) language. \
\n\n\
Respond ONLY with a JSON object in this exact format, with no other text:\n\
{\n\
  \"is_foreign_language\": true/false,\n\
  \"detected_language\": \"language name or null\",\n\
  \"confidence\": \"high\"/\"medium\"/\"low\"\n\
}\n\n\
Rules:\n\
- If the text is English, set is_foreign_language to false and detected_language to null\n\
- If the text is gibberish or random characters, set is_foreign_language to false and detected_language to null\n\
- If the text is clearly a non-English language, identify it\n\
- Only set confidence to \"high\" if you are very sure about the detection";

    let user_content = format!("Detect the language of this text:\n\n{}", text);

    vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(&user_content),
    ]
}

/// Builds the prompt messages for translating text from a detected language to English.
///
/// This should be called after `build_detect_language_prompt` confirms the text
/// is in a foreign language.
///
/// # Arguments
///
/// * `text` - The text to translate
/// * `source_language` - The detected source language (e.g., "French", "Japanese")
pub fn build_translate_prompt(text: &str, source_language: &str) -> Vec<ChatMessage> {
    let system_prompt = format!(
        "\
You are a professional translator. Translate the following text from {} to English. \
\n\n\
Rules:\n\
- Provide ONLY the English translation, nothing else\n\
- Preserve the original meaning as closely as possible\n\
- If the text contains technical terms, keep them accurate\n\
- If parts of the text are ambiguous, translate the most likely meaning",
        source_language
    );

    let user_content = format!(
        "Translate this {} text to English:\n\n{}",
        source_language, text
    );

    vec![
        ChatMessage::system(&system_prompt),
        ChatMessage::user(&user_content),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_step_prompt_without_key() {
        let messages = build_explain_step_prompt("Base64", "SGVsbG8=", "Hello", None);
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert!(messages[1].content.contains("Base64"));
        assert!(messages[1].content.contains("SGVsbG8="));
        assert!(messages[1].content.contains("Hello"));
        assert!(!messages[1].content.contains("Key:"));
    }

    #[test]
    fn test_explain_step_prompt_with_key() {
        let messages = build_explain_step_prompt("Caesar", "Uryyb", "Hello", Some("13"));
        assert_eq!(messages.len(), 2);
        assert!(messages[1].content.contains("Caesar"));
        assert!(messages[1].content.contains("Key: 13"));
    }

    #[test]
    fn test_detect_language_prompt() {
        let messages = build_detect_language_prompt("Bonjour le monde");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert!(messages[0].content.contains("JSON"));
        assert!(messages[1].content.contains("Bonjour le monde"));
    }

    #[test]
    fn test_translate_prompt() {
        let messages = build_translate_prompt("Bonjour le monde", "French");
        assert_eq!(messages.len(), 2);
        assert!(messages[0].content.contains("French"));
        assert!(messages[0].content.contains("English"));
        assert!(messages[1].content.contains("Bonjour le monde"));
    }

    #[test]
    fn test_explain_step_system_prompt_has_guidelines() {
        let messages = build_explain_step_prompt("Hex", "48656c6c6f", "Hello", None);
        let system = &messages[0].content;
        // System prompt should contain structure guidelines
        assert!(system.contains("One sentence"));
        assert!(system.contains("75 words"));
    }

    #[test]
    fn test_detect_language_system_prompt_has_json_format() {
        let messages = build_detect_language_prompt("test");
        let system = &messages[0].content;
        assert!(system.contains("is_foreign_language"));
        assert!(system.contains("detected_language"));
        assert!(system.contains("confidence"));
    }
}
