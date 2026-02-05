//! Color scheme selection screen rendering for the setup wizard.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use super::super::app::CustomColors;
use super::super::themes::{ColorScheme, THEMES};

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Secondary color (cyan)
const SECONDARY: Color = Color::Rgb(139, 233, 253);
/// Muted text color
const MUTED: Color = Color::DarkGray;

/// Draws the theme selection screen with live preview.
pub fn draw_theme_selection(
    frame: &mut Frame,
    area: Rect,
    selected: usize,
    custom_mode: bool,
    custom_colors: &CustomColors,
    custom_field: usize,
) {
    // Split into theme list and preview
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left side: theme list
    let list_block = Block::default()
        .title(" Select Theme ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(1, 1, 1, 1));

    let list_inner = list_block.inner(chunks[0]);
    frame.render_widget(list_block, chunks[0]);

    if custom_mode {
        // Show custom color input form
        draw_custom_color_form(frame, list_inner, custom_colors, custom_field);
    } else {
        // Show theme list
        let mut lines = Vec::new();
        for (i, theme) in THEMES.iter().enumerate() {
            let prefix = if i == selected { " > " } else { "   " };
            let name = if let Some(icon) = theme.icon {
                format!("{}. {} {}", i + 1, icon, theme.name)
            } else {
                format!("{}. {}", i + 1, theme.name)
            };

            let style = if i == selected {
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, name),
                style,
            )));
        }

        // Add custom option
        let custom_prefix = if selected == THEMES.len() {
            " > "
        } else {
            "   "
        };
        let custom_style = if selected == THEMES.len() {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        lines.push(Line::from(Span::styled(
            format!("{}{}. Custom...", custom_prefix, THEMES.len() + 1),
            custom_style,
        )));

        let list = Paragraph::new(lines);
        frame.render_widget(list, list_inner);
    }

    // Right side: live preview
    let preview_scheme = if custom_mode {
        custom_colors.to_scheme().unwrap_or_default()
    } else if selected < THEMES.len() {
        THEMES[selected].scheme.clone()
    } else {
        ColorScheme::default()
    };

    draw_theme_preview(frame, chunks[1], &preview_scheme);
}

/// Draws the custom color input form.
fn draw_custom_color_form(
    frame: &mut Frame,
    area: Rect,
    custom_colors: &CustomColors,
    current_field: usize,
) {
    let mut lines = vec![
        Line::from(Span::styled(
            "Enter RGB values (e.g., 255,128,64)",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
    ];

    for i in 0..5 {
        let label = CustomColors::field_name(i);
        let value = custom_colors.get_field(i);

        let prefix = if i == current_field { "> " } else { "  " };

        let style = if i == current_field {
            Style::default().fg(ACCENT)
        } else {
            Style::default().fg(Color::White)
        };

        let value_style = if i == current_field {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default().fg(MUTED)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{}: ", label), style),
            Span::styled(
                if value.is_empty() {
                    "___,___,___"
                } else {
                    value
                },
                value_style,
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Tab] Next field  [Enter] Confirm  [Esc] Cancel",
        Style::default().fg(MUTED),
    )));

    let form = Paragraph::new(lines);
    frame.render_widget(form, area);
}

/// Draws the theme preview panel.
fn draw_theme_preview(frame: &mut Frame, area: Rect, scheme: &ColorScheme) {
    let block = Block::default()
        .title(" Live Preview ")
        .title_style(Style::default().fg(SECONDARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(1, 1, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled("Informational", scheme.informational_style())),
        Line::from(Span::styled(
            "  Status updates and info",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled("Warning", scheme.warning_style())),
        Line::from(Span::styled(
            "  Cautions and alerts",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled("Success", scheme.success_style())),
        Line::from(Span::styled(
            "  Successful operations",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled("Error", scheme.error_style())),
        Line::from(Span::styled("  Error messages", Style::default().fg(MUTED))),
        Line::from(""),
        Line::from(Span::styled("Questions", scheme.question_style())),
        Line::from(Span::styled(
            "  Interactive prompts",
            Style::default().fg(MUTED),
        )),
    ];

    let preview = Paragraph::new(lines);
    frame.render_widget(preview, inner);
}
