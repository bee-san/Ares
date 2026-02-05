//! Tutorial screen rendering for the setup wizard.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Secondary color (cyan)
const SECONDARY: Color = Color::Rgb(139, 233, 253);
/// Muted text color
const MUTED: Color = Color::DarkGray;

/// Draws the tutorial screen.
pub fn draw_tutorial(frame: &mut Frame, area: Rect) {
    let content_area = centered_rect(area, 90, 90);

    let block = Block::default()
        .title(" Quick Tutorial ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    // Split into two columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    // Left column - Basic usage
    let left_lines = vec![
        Line::from(Span::styled(
            "Basic Usage",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("ciphey -t ", Style::default().fg(SECONDARY)),
            Span::styled("'encoded text'", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            "  Decode text automatically",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("ciphey -f ", Style::default().fg(SECONDARY)),
            Span::styled("file.txt", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            "  Decode contents of a file",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "With a Crib/Regex",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("ciphey -t ", Style::default().fg(SECONDARY)),
            Span::styled("'text' ", Style::default().fg(Color::White)),
            Span::styled("-r ", Style::default().fg(SECONDARY)),
            Span::styled("'flag{'", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            "  Match plaintext against pattern",
            Style::default().fg(MUTED),
        )),
    ];

    // Right column - Modes & Options
    let right_lines = vec![
        Line::from(Span::styled(
            "TUI vs CLI Mode",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "TUI mode (default in terminal):",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  Interactive UI with visual feedback",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("ciphey --no-tui ", Style::default().fg(SECONDARY)),
            Span::styled("-t 'text'", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            "  CLI mode for scripting/piping",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Useful Options",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("--top-results", Style::default().fg(SECONDARY)),
            Span::styled("  Show all matches", Style::default().fg(MUTED)),
        ]),
        Line::from(vec![
            Span::styled("-c ", Style::default().fg(SECONDARY)),
            Span::styled("<secs>", Style::default().fg(Color::White)),
            Span::styled("       Set timeout", Style::default().fg(MUTED)),
        ]),
        Line::from(vec![
            Span::styled("-d", Style::default().fg(SECONDARY)),
            Span::styled("              Disable prompts", Style::default().fg(MUTED)),
        ]),
    ];

    let left_para = Paragraph::new(left_lines);
    let right_para = Paragraph::new(right_lines);

    frame.render_widget(left_para, columns[0]);
    frame.render_widget(right_para, columns[1]);
}

/// Creates a centered rectangle within the given area.
pub fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    Rect::new(x, y, popup_width, popup_height)
}
