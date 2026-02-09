//! AI configuration screen for the setup wizard.
//!
//! This module renders the AI setup page where users can enable AI features
//! and configure their OpenAI-compatible API endpoint.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use super::tutorial::centered_rect;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Muted text color
const MUTED: Color = Color::DarkGray;
/// Success color
const SUCCESS: Color = Color::Rgb(80, 250, 123);

/// Focus state within the AI config screen.
#[derive(Debug, Clone, PartialEq)]
pub enum AiConfigFocus {
    /// Focused on the enable/disable toggle.
    EnableToggle,
    /// Focused on the API URL input field.
    ApiUrl,
    /// Focused on the API key input field.
    ApiKey,
    /// Focused on the model name input field.
    Model,
}

/// Draws the AI configuration screen.
///
/// # Arguments
///
/// * `frame` - The ratatui frame to render into
/// * `area` - The available area for rendering
/// * `selected` - Whether AI is enabled (0 = no, 1 = yes)
/// * `api_url` - The current API URL input
/// * `api_key` - The current API key input (will be masked)
/// * `model` - The current model name input
/// * `focus` - Which field is currently focused
/// * `cursor` - Cursor position in the active text field
#[allow(clippy::too_many_arguments)]
pub fn draw_ai_config(
    frame: &mut Frame,
    area: Rect,
    selected: usize,
    api_url: &str,
    api_key: &str,
    model: &str,
    focus: &AiConfigFocus,
    cursor: usize,
) {
    let content_area = centered_rect(area, 80, 85);

    let block = Block::default()
        .title(" AI Features ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines = vec![
        Line::from(Span::styled(
            "Would you like to enable AI-powered features?",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "AI features use an OpenAI-compatible API to provide:",
            Style::default().fg(MUTED),
        )),
        Line::from(Span::styled(
            "  - Step explanations (how decoders transform text)",
            Style::default().fg(MUTED),
        )),
        Line::from(Span::styled(
            "  - Language detection and translation",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
    ];

    // Enable/disable toggle
    let toggle_style = if *focus == AiConfigFocus::EnableToggle {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let no_style = if selected == 0 {
        Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let yes_style = if selected == 1 {
        Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let focus_indicator = if *focus == AiConfigFocus::EnableToggle {
        "> "
    } else {
        "  "
    };

    lines.push(Line::from(vec![
        Span::styled(focus_indicator, toggle_style),
        Span::styled("Enable AI: ", toggle_style),
        Span::styled(if selected == 0 { "[No]" } else { " No " }, no_style),
        Span::styled("  ", Style::default()),
        Span::styled(if selected == 1 { "[Yes]" } else { " Yes " }, yes_style),
    ]));

    // Only show config fields if AI is enabled
    if selected == 1 {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Configure your OpenAI-compatible API endpoint:",
            Style::default().fg(Color::White),
        )));
        lines.push(Line::from(""));

        // API URL field
        let url_focus = *focus == AiConfigFocus::ApiUrl;
        let url_indicator = if url_focus { "> " } else { "  " };
        let url_label_style = if url_focus {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let url_value = if api_url.is_empty() {
            "https://api.openai.com/v1"
        } else {
            api_url
        };
        let url_value_style = if api_url.is_empty() && !url_focus {
            Style::default().fg(MUTED)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(url_indicator, url_label_style),
            Span::styled("API URL:  ", url_label_style),
            Span::styled(url_value, url_value_style),
            Span::styled(
                if url_focus { "_" } else { "" },
                Style::default().fg(ACCENT),
            ),
        ]));

        // API Key field (masked)
        let key_focus = *focus == AiConfigFocus::ApiKey;
        let key_indicator = if key_focus { "> " } else { "  " };
        let key_label_style = if key_focus {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let masked_key = if api_key.is_empty() {
            if key_focus {
                String::new()
            } else {
                "(enter your API key)".to_string()
            }
        } else {
            // Show first 4 and last 4 chars, mask the rest
            if api_key.len() > 8 {
                let first: String = api_key.chars().take(4).collect();
                let last: String = api_key
                    .chars()
                    .rev()
                    .take(4)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect();
                format!("{}...{}", first, last)
            } else {
                "*".repeat(api_key.len())
            }
        };
        let key_value_style = if api_key.is_empty() && !key_focus {
            Style::default().fg(MUTED)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(key_indicator, key_label_style),
            Span::styled("API Key:  ", key_label_style),
            Span::styled(masked_key, key_value_style),
            Span::styled(
                if key_focus { "_" } else { "" },
                Style::default().fg(ACCENT),
            ),
        ]));

        // Model field
        let model_focus = *focus == AiConfigFocus::Model;
        let model_indicator = if model_focus { "> " } else { "  " };
        let model_label_style = if model_focus {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let model_value = if model.is_empty() {
            "gpt-4o-mini"
        } else {
            model
        };
        let model_value_style = if model.is_empty() && !model_focus {
            Style::default().fg(MUTED)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(model_indicator, model_label_style),
            Span::styled("Model:    ", model_label_style),
            Span::styled(model_value, model_value_style),
            Span::styled(
                if model_focus { "_" } else { "" },
                Style::default().fg(ACCENT),
            ),
        ]));

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Works with OpenAI, Ollama, LM Studio, and other compatible APIs.",
            Style::default().fg(MUTED),
        )));
        lines.push(Line::from(Span::styled(
            "  Your API key is stored in ~/.ciphey/config.toml.",
            Style::default().fg(MUTED),
        )));
    }

    // Set cursor position if we're editing a text field
    let _cursor_pos = cursor; // Used for future cursor rendering improvement
    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
