//! Ask AI modal widget for the Ciphey TUI.
//!
//! This module provides a floating modal overlay that allows users to ask
//! questions about a specific decoder step. The AI uses the full step context
//! (decoder name, input, output, key) to provide specific answers.

use ratatui::prelude::*;
use ratatui::style::Modifier;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

use super::super::colors::TuiColors;
use crate::tui::app::AskAiOverlay;

/// Ask AI modal width as percentage of screen width.
const MODAL_WIDTH_PERCENT: u16 = 70;
/// Ask AI modal height as percentage of screen height.
const MODAL_HEIGHT_PERCENT: u16 = 75;
/// Maximum text length for context display before truncation.
const MAX_CONTEXT_DISPLAY: usize = 60;

/// Truncates text for context display, adding ellipsis if needed.
fn truncate_display(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max_len).collect::<String>())
    }
}

/// Creates a centered rectangle within the given area.
fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;
    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
    Rect::new(x, y, popup_width, popup_height)
}

/// Renders the Ask AI modal overlay.
///
/// This floating modal displays:
/// - A multiline text input for the question
/// - Step context (decoder, input preview, output preview, key)
/// - AI response area (scrollable)
/// - Status bar with keybindings
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The full screen area (modal will be centered within this)
/// * `overlay` - The Ask AI overlay state
/// * `colors` - The TUI color scheme to use
pub fn render_ask_ai_modal(
    frame: &mut Frame,
    area: Rect,
    overlay: &AskAiOverlay,
    colors: &TuiColors,
) {
    let modal_area = centered_rect(area, MODAL_WIDTH_PERCENT, MODAL_HEIGHT_PERCENT);

    // Clear area behind modal
    frame.render_widget(Clear, modal_area);

    // Outer block
    let block = Block::default()
        .title(" Ask AI About This Step ")
        .title_style(colors.accent.add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(1, 1, 1, 0));

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    // Split into sections: context, question input, response, status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Context display
            Constraint::Length(5), // Question input
            Constraint::Min(5),    // Response area
            Constraint::Length(1), // Status bar
        ])
        .split(inner);

    // ── Context Display ──
    render_context(frame, chunks[0], overlay, colors);

    // ── Question Input ──
    render_question_input(frame, chunks[1], overlay, colors);

    // ── Response Area ──
    render_response(frame, chunks[2], overlay, colors);

    // ── Status Bar ──
    render_status_bar(frame, chunks[3], overlay, colors);
}

/// Renders the step context section.
fn render_context(frame: &mut Frame, area: Rect, overlay: &AskAiOverlay, colors: &TuiColors) {
    let input_preview = truncate_display(&overlay.step_input, MAX_CONTEXT_DISPLAY);
    let output_preview = truncate_display(&overlay.step_output, MAX_CONTEXT_DISPLAY);
    let key_display = overlay.step_key.as_deref().unwrap_or("N/A").to_string();

    let lines = vec![
        Line::from(vec![
            Span::styled("Decoder: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(&overlay.decoder_name, colors.value),
            Span::styled("  Key: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(key_display, colors.value),
        ]),
        Line::from(vec![
            Span::styled("Input:  ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(input_preview, colors.text_before),
        ]),
        Line::from(vec![
            Span::styled("Output: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(output_preview, colors.text_after),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(colors.border);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Renders the question input section.
fn render_question_input(
    frame: &mut Frame,
    area: Rect,
    overlay: &AskAiOverlay,
    colors: &TuiColors,
) {
    let input_block = Block::default()
        .title(" Your Question (Ctrl+Enter to submit) ")
        .title_style(colors.accent)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.accent)
        .padding(Padding::horizontal(1));

    let input_inner = input_block.inner(area);
    frame.render_widget(input_block, area);

    // Render the text input content with cursor
    let (cursor_line, cursor_col) = overlay.text_input.cursor_pos();
    let scroll_offset = overlay.text_input.scroll_offset();
    let visible_lines = input_inner.height as usize;

    let lines: Vec<Line> = overlay
        .text_input
        .lines()
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_lines)
        .map(|(line_idx, line_text)| {
            let is_cursor_line = line_idx == cursor_line;
            if is_cursor_line {
                let chars: Vec<char> = line_text.chars().collect();
                let before: String = chars.iter().take(cursor_col).collect();
                let cursor_char = chars
                    .get(cursor_col)
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| " ".to_string());
                let after: String = chars.iter().skip(cursor_col + 1).collect();
                Line::from(vec![
                    Span::styled(before, colors.text),
                    Span::styled(cursor_char, colors.accent.add_modifier(Modifier::REVERSED)),
                    Span::styled(after, colors.text),
                ])
            } else {
                Line::from(Span::styled(line_text.clone(), colors.text))
            }
        })
        .collect();

    // Show placeholder if empty
    let display_lines = if overlay.text_input.is_empty() {
        vec![Line::from(vec![
            Span::styled(" ", colors.accent.add_modifier(Modifier::REVERSED)),
            Span::styled(" Ask anything about this step...", colors.muted),
        ])]
    } else {
        lines
    };

    let paragraph = Paragraph::new(display_lines).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, input_inner);
}

/// Renders the AI response section.
fn render_response(frame: &mut Frame, area: Rect, overlay: &AskAiOverlay, colors: &TuiColors) {
    let response_block = Block::default()
        .title(" AI Response ")
        .title_style(colors.label)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border)
        .padding(Padding::horizontal(1));

    let response_inner = response_block.inner(area);
    frame.render_widget(response_block, area);

    if overlay.loading {
        let loading_text = Paragraph::new(Line::from(Span::styled(
            "Thinking...",
            colors.muted.add_modifier(Modifier::ITALIC),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(loading_text, response_inner);
    } else if let Some(ref error) = overlay.error {
        let error_text = Paragraph::new(Line::from(Span::styled(
            format!("Error: {}", error),
            colors.error,
        )))
        .wrap(Wrap { trim: false });
        frame.render_widget(error_text, response_inner);
    } else if let Some(ref response) = overlay.response {
        let lines: Vec<Line> = response
            .lines()
            .map(|line| Line::from(Span::styled(line.to_string(), colors.text)))
            .collect();
        let response_text = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((overlay.response_scroll, 0));
        frame.render_widget(response_text, response_inner);
    } else {
        let placeholder = Paragraph::new(Line::from(Span::styled(
            "Type your question above and press Ctrl+Enter",
            colors.muted,
        )))
        .alignment(Alignment::Center);
        frame.render_widget(placeholder, response_inner);
    }
}

/// Renders the status bar with keybindings.
fn render_status_bar(frame: &mut Frame, area: Rect, overlay: &AskAiOverlay, colors: &TuiColors) {
    let mut spans = vec![
        Span::styled("[Ctrl+Enter]", colors.accent),
        Span::styled(" Submit  ", colors.muted),
        Span::styled("[Esc]", colors.accent),
        Span::styled(" Close", colors.muted),
    ];

    if overlay.response.is_some() {
        spans.push(Span::styled("  ", colors.text));
        spans.push(Span::styled("[↑/↓]", colors.accent));
        spans.push(Span::styled(" Scroll  ", colors.muted));
        spans.push(Span::styled("[Ctrl+C]", colors.accent));
        spans.push(Span::styled(" Copy", colors.muted));
    }

    let status = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(status, area);
}
