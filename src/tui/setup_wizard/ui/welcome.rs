//! Welcome screen rendering for the setup wizard.

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use super::super::app::SetupApp;

/// ASCII art logo for the welcome screen.
const LOGO: &str = r#"
   _____ _       _                
  / ____(_)     | |               
 | |     _ _ __ | |__   ___ _   _ 
 | |    | | '_ \| '_ \ / _ \ | | |
 | |____| | |_) | | | |  __/ |_| |
  \_____|_| .__/|_| |_|\___|\__, |
          | |                __/ |
          |_|               |___/ 
"#;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Secondary color (cyan)
const SECONDARY: Color = Color::Rgb(139, 233, 253);
/// Muted text color
const MUTED: Color = Color::DarkGray;

/// Draws the welcome screen with ASCII art.
pub fn draw_welcome(frame: &mut Frame, area: Rect, _app: &SetupApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Logo
            Constraint::Length(2),  // Spacing
            Constraint::Length(3),  // Welcome text
            Constraint::Min(3),     // Instructions
        ])
        .split(area);

    // Draw animated logo
    let logo_style = Style::default().fg(ACCENT).add_modifier(Modifier::BOLD);
    let logo = Paragraph::new(LOGO)
        .style(logo_style)
        .alignment(Alignment::Center);
    frame.render_widget(logo, chunks[0]);

    // Welcome message
    let welcome_lines = vec![
        Line::from(Span::styled(
            "Welcome to Ciphey!",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Identify and decrypt anything",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Made by Bee  https://skerritt.blog",
            Style::default().fg(SECONDARY),
        )),
    ];
    let welcome = Paragraph::new(welcome_lines).alignment(Alignment::Center);
    frame.render_widget(welcome, chunks[2]);

    // Instructions
    let instruction_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Let's set up your preferences.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(MUTED)),
            Span::styled(
                "[Enter]",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to begin setup, or ", Style::default().fg(MUTED)),
            Span::styled(
                "[S]",
                Style::default().fg(SECONDARY).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to skip and use defaults", Style::default().fg(MUTED)),
        ]),
    ];
    let instructions = Paragraph::new(instruction_lines).alignment(Alignment::Center);
    frame.render_widget(instructions, chunks[3]);
}
