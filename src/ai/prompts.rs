//! Prompt templates for AI-powered features.
//!
//! This module contains the prompt builders for each AI use case:
//! - Explaining decoder steps (how a specific decoder transforms input to output)
//! - Asking questions about a decoder step (with conversation history support)
//! - Detecting the language of a piece of text
//! - Translating foreign language text into English with a language description

use super::client::ChatMessage;

/// Maximum character length for input/output text in explanation prompts.
const EXPLAIN_TRUNCATE_LEN: usize = 500;
/// Maximum character length for input text in language detection prompts.
const DETECT_LANGUAGE_TRUNCATE_LEN: usize = 500;
/// Maximum character length for input text in translation prompts.
const TRANSLATE_TRUNCATE_LEN: usize = 2000;
/// Maximum character length for context display in ask-about-step prompts.
const ASK_CONTEXT_TRUNCATE_LEN: usize = 200;
/// Maximum number of conversation history turns to include in ask-about-step prompts.
const MAX_HISTORY_TURNS: usize = 10;

/// Builds the prompt messages for explaining how a decoder step works.
///
/// The AI will explain the algorithm/encoding used by the decoder, how the
/// specific input is transformed into the output, and (if applicable) how
/// the key was used. For ciphers, it also explains the mathematical operation.
///
/// Input and output are truncated to [`EXPLAIN_TRUNCATE_LEN`] characters to
/// avoid wasting tokens on large payloads (e.g., long Base64 blobs).
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
You are a concise cryptography expert embedded in Ciphey, an automated decryption and decoding tool. \
Explain ONLY the specific transformation shown.

Your answer must be:
1. One sentence about what this decoder/cipher does
2. Exactly how THIS input became THIS output (be concrete, reference the actual data)
3. If a key was used, explain its role in the transformation
4. If this is a cipher, briefly explain the mathematical or algorithmic operation involved

Keep it under 120 words total. Be direct. Focus on the actual data transformation, not general theory.";

    let key_section = if let Some(key_val) = key {
        format!("\nKey: {}", key_val)
    } else {
        String::new()
    };

    let truncated_input = truncate_for_prompt(input, EXPLAIN_TRUNCATE_LEN);
    let truncated_output = truncate_for_prompt(output, EXPLAIN_TRUNCATE_LEN);

    let user_content = format!(
        "Decoder: {}\nInput: {}\nOutput: {}{}",
        decoder_name, truncated_input, truncated_output, key_section
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
/// so the AI can give a specific, accurate answer. Supports multi-turn conversation
/// by accepting prior conversation history.
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
/// * `history` - Previous conversation messages (user/assistant pairs) for multi-turn context
#[allow(clippy::too_many_arguments)]
pub fn build_ask_about_step_prompt(
    question: &str,
    decoder_name: &str,
    input: &str,
    output: &str,
    key: Option<&str>,
    description: &str,
    link: &str,
    history: &[ChatMessage],
) -> Vec<ChatMessage> {
    let system_prompt = "\
You are an expert in cryptography, encoding, and decoding, embedded in Ciphey (an automated decryption tool). \
Answer the user's question about this decoding step accurately and clearly. \
Use the provided context to give specific answers. \
If the user asks a follow-up question, use the conversation history for context. \
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
        truncate_for_prompt(input, ASK_CONTEXT_TRUNCATE_LEN),
        input.chars().count(),
        truncate_for_prompt(output, ASK_CONTEXT_TRUNCATE_LEN),
        output.chars().count(),
        key_str,
        link
    );

    let mut messages = Vec::new();
    messages.push(ChatMessage::system(system_prompt));

    // Include the context as the first user message
    messages.push(ChatMessage::user(&context));
    messages.push(ChatMessage::assistant(
        "I have the context for this decoding step. What would you like to know?",
    ));

    // Append conversation history (capped to prevent token explosion)
    let history_start = if history.len() > MAX_HISTORY_TURNS * 2 {
        history.len() - MAX_HISTORY_TURNS * 2
    } else {
        0
    };
    for msg in &history[history_start..] {
        messages.push(msg.clone());
    }

    // Append the new question
    messages.push(ChatMessage::user(question));

    messages
}

/// Builds the prompt messages for detecting whether text is a foreign language.
///
/// The AI will determine if the text is in a non-English language and identify
/// which language it is. Returns a structured JSON response for easy parsing.
///
/// Input is truncated to [`DETECT_LANGUAGE_TRUNCATE_LEN`] characters since
/// the first few hundred characters are sufficient for language detection.
///
/// Includes a few-shot example to improve JSON output reliability.
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

    // Few-shot example for reliable JSON formatting
    let example_user = "Detect the language of this text:\n\nBonjour tout le monde";
    let example_assistant =
        r#"{"is_foreign_language": true, "detected_language": "French", "confidence": "high"}"#;

    let truncated_text = truncate_for_prompt(text, DETECT_LANGUAGE_TRUNCATE_LEN);
    let user_content = format!("Detect the language of this text:\n\n{}", truncated_text);

    vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(example_user),
        ChatMessage::assistant(example_assistant),
        ChatMessage::user(&user_content),
    ]
}

/// Builds the prompt messages for translating text and providing a language description.
///
/// This combines translation with an educational description of the source language,
/// returning a structured JSON response containing both the translation and language info.
///
/// Input is truncated to [`TRANSLATE_TRUNCATE_LEN`] characters to avoid excessive
/// token usage on very long texts.
///
/// # Arguments
///
/// * `text` - The text to translate
/// * `source_language` - The detected source language (e.g., "French", "Japanese")
pub fn build_translate_with_description_prompt(
    text: &str,
    source_language: &str,
) -> Vec<ChatMessage> {
    let system_prompt = format!(
        "\
You are a professional translator and linguist. Your task is to translate text from {} to English \
and provide a brief description of the source language.\n\n\
Respond ONLY with a JSON object in this exact format, with no other text:\n\
{{\n\
  \"translation\": \"the English translation here\",\n\
  \"language_description\": \"1-2 sentence description of the language\"\n\
}}\n\n\
Rules for translation:\n\
- Preserve the original meaning as closely as possible\n\
- If the text contains technical terms, keep them accurate\n\
- If parts are ambiguous, translate the most likely meaning\n\n\
Rules for language_description:\n\
- Keep it to 1-2 sentences (under 50 words)\n\
- Include: language family, approximate number of speakers, and where it's primarily spoken\n\
- Mention the writing system if it's non-Latin (e.g., Cyrillic, Kanji, Arabic script)",
        source_language
    );

    let truncated_text = truncate_for_prompt(text, TRANSLATE_TRUNCATE_LEN);

    let user_content = format!(
        "Translate this {} text to English and describe the language:\n\n{}",
        source_language, truncated_text
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
    fn test_explain_step_prompt_truncates_long_input() {
        let long_input = "A".repeat(1000);
        let messages = build_explain_step_prompt("Base64", &long_input, "Hello", None);
        // Input should be truncated to EXPLAIN_TRUNCATE_LEN + "..."
        assert!(messages[1].content.len() < 1000);
        assert!(messages[1].content.contains("..."));
    }

    #[test]
    fn test_explain_step_system_prompt_has_ciphey_context() {
        let messages = build_explain_step_prompt("Hex", "48656c6c6f", "Hello", None);
        let system = &messages[0].content;
        assert!(system.contains("Ciphey"));
        assert!(system.contains("120 words"));
    }

    #[test]
    fn test_explain_step_system_prompt_mentions_cipher_math() {
        let messages = build_explain_step_prompt("Caesar", "Uryyb", "Hello", Some("13"));
        let system = &messages[0].content;
        assert!(system.contains("mathematical"));
    }

    #[test]
    fn test_detect_language_prompt() {
        let messages = build_detect_language_prompt("Bonjour le monde");
        // system + few-shot example (user + assistant) + actual user message = 4
        assert_eq!(messages.len(), 4);
        assert_eq!(messages[0].role, "system");
        assert!(messages[0].content.contains("JSON"));
        // Few-shot example
        assert_eq!(messages[1].role, "user");
        assert!(messages[1].content.contains("Bonjour tout le monde"));
        assert_eq!(messages[2].role, "assistant");
        assert!(messages[2].content.contains("French"));
        // Actual user message
        assert_eq!(messages[3].role, "user");
        assert!(messages[3].content.contains("Bonjour le monde"));
    }

    #[test]
    fn test_detect_language_prompt_truncates_long_text() {
        let long_text = "Bonjour ".repeat(200);
        let messages = build_detect_language_prompt(&long_text);
        let user_msg = &messages[3].content;
        assert!(user_msg.len() < long_text.len());
        assert!(user_msg.contains("..."));
    }

    #[test]
    fn test_detect_language_system_prompt_has_json_format() {
        let messages = build_detect_language_prompt("test");
        let system = &messages[0].content;
        assert!(system.contains("is_foreign_language"));
        assert!(system.contains("detected_language"));
        assert!(system.contains("confidence"));
    }

    #[test]
    fn test_translate_with_description_prompt() {
        let messages = build_translate_with_description_prompt("Bonjour le monde", "French");
        assert_eq!(messages.len(), 2);
        assert!(messages[0].content.contains("French"));
        assert!(messages[0].content.contains("English"));
        assert!(messages[0].content.contains("JSON"));
        assert!(messages[1].content.contains("Bonjour le monde"));
    }

    #[test]
    fn test_translate_with_description_truncates_long_text() {
        let long_text = "Hola ".repeat(1000);
        let messages = build_translate_with_description_prompt(&long_text, "Spanish");
        let user_msg = &messages[1].content;
        assert!(user_msg.len() < long_text.len());
        assert!(user_msg.contains("..."));
    }

    #[test]
    fn test_translate_with_description_system_prompt_has_json_format() {
        let messages = build_translate_with_description_prompt("test", "Spanish");
        let system = &messages[0].content;
        assert!(system.contains("translation"));
        assert!(system.contains("language_description"));
    }

    #[test]
    fn test_translate_with_description_system_prompt_has_guidelines() {
        let messages = build_translate_with_description_prompt("test", "German");
        let system = &messages[0].content;
        assert!(system.contains("language family"));
        assert!(system.contains("speakers"));
        assert!(system.contains("writing system"));
    }

    #[test]
    fn test_ask_about_step_prompt_without_history() {
        let messages = build_ask_about_step_prompt(
            "How does this work?",
            "Base64",
            "SGVsbG8=",
            "Hello",
            None,
            "Base64 decoder",
            "https://example.com",
            &[],
        );
        // system + context user + context assistant + question = 4
        assert_eq!(messages.len(), 4);
        assert_eq!(messages[0].role, "system");
        assert!(messages[0].content.contains("Ciphey"));
        assert_eq!(messages[1].role, "user");
        assert!(messages[1].content.contains("Base64"));
        assert_eq!(messages[2].role, "assistant");
        assert_eq!(messages[3].role, "user");
        assert!(messages[3].content.contains("How does this work?"));
    }

    #[test]
    fn test_ask_about_step_prompt_with_history() {
        let history = vec![
            ChatMessage::user("What is Base64?"),
            ChatMessage::assistant("Base64 is an encoding scheme."),
        ];
        let messages = build_ask_about_step_prompt(
            "Can you explain more?",
            "Base64",
            "SGVsbG8=",
            "Hello",
            None,
            "Base64 decoder",
            "https://example.com",
            &history,
        );
        // system + context user + context assistant + 2 history + question = 6
        assert_eq!(messages.len(), 6);
        assert!(messages[3].content.contains("What is Base64?"));
        assert!(messages[4].content.contains("encoding scheme"));
        assert!(messages[5].content.contains("Can you explain more?"));
    }

    #[test]
    fn test_ask_about_step_prompt_caps_long_history() {
        // Create history with more than MAX_HISTORY_TURNS * 2 messages
        let mut history = Vec::new();
        for i in 0..30 {
            history.push(ChatMessage::user(&format!("Question {}", i)));
            history.push(ChatMessage::assistant(&format!("Answer {}", i)));
        }
        let messages = build_ask_about_step_prompt(
            "Final question?",
            "Caesar",
            "Uryyb",
            "Hello",
            Some("13"),
            "Caesar cipher",
            "https://example.com",
            &history,
        );
        // Should be capped: system(1) + context(2) + MAX_HISTORY_TURNS*2(20) + question(1) = 24
        assert_eq!(messages.len(), 24);
    }

    #[test]
    fn test_ask_about_step_prompt_with_key() {
        let messages = build_ask_about_step_prompt(
            "What is the key?",
            "Caesar",
            "Uryyb",
            "Hello",
            Some("13"),
            "Caesar cipher decoder",
            "https://example.com",
            &[],
        );
        assert!(messages[1].content.contains("Key: 13"));
    }

    #[test]
    fn test_truncate_for_prompt_short() {
        let result = truncate_for_prompt("hello", 10);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_truncate_for_prompt_long() {
        let result = truncate_for_prompt("hello world!", 5);
        assert_eq!(result, "hello...");
    }

    #[test]
    fn test_truncate_for_prompt_exact() {
        let result = truncate_for_prompt("hello", 5);
        assert_eq!(result, "hello");
    }
}
