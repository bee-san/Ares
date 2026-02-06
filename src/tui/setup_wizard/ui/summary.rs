//! Summary and completion screens for the setup wizard.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use super::super::app::SetupApp;
use super::tutorial::centered_rect;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Muted text color
const MUTED: Color = Color::DarkGray;
/// Success color
const SUCCESS: Color = Color::Rgb(80, 250, 123);
/// Error color
const ERROR: Color = Color::Rgb(255, 85, 85);

/// Cute cat ASCII art for the easter egg.
const CAT_ART: &str = r#"
    /\_____/\
   /  o   o  \
  ( ==  ^  == )
   )         (
  (           )
 ( (  )   (  ) )
(__(__)___(__)__)
"#;

/// Draws the results mode selection screen.
pub fn draw_results_mode(frame: &mut Frame, area: Rect, selected: usize) {
    let content_area = centered_rect(area, 80, 80);

    let block = Block::default()
        .title(" Results Mode ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let option1_style = if selected == 0 {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let option2_style = if selected == 1 {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let lines = vec![
        Line::from(Span::styled(
            "How should Ciphey handle potential plaintexts?",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(if selected == 0 { " > " } else { "   " }, option1_style),
            Span::styled("1. ", option1_style),
            Span::styled("Ask me each time", option1_style),
        ]),
        Line::from(Span::styled(
            "      Ciphey will prompt you to confirm each potential plaintext",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(if selected == 1 { " > " } else { "   " }, option2_style),
            Span::styled("2. ", option2_style),
            Span::styled("Collect all results", option2_style),
        ]),
        Line::from(Span::styled(
            "      Ciphey will gather all possible plaintexts and show them at the end",
            Style::default().fg(MUTED),
        )),
    ];

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draws the timeout configuration screen.
pub fn draw_timeout_config(frame: &mut Frame, area: Rect, value: u32) {
    let content_area = centered_rect(area, 70, 70);

    let block = Block::default()
        .title(" Timeout Configuration ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let lines = vec![
        Line::from(Span::styled(
            "How long should Ciphey run before stopping?",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "For 'Collect all results' mode, we recommend 3 seconds.",
            Style::default().fg(MUTED),
        )),
        Line::from(Span::styled(
            "Higher values may use significant CPU resources.",
            Style::default().fg(ERROR),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Timeout: ", Style::default().fg(Color::White)),
            Span::styled(
                format!(" {} ", value),
                Style::default()
                    .fg(Color::Black)
                    .bg(ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" seconds", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Use [Up]/[Down] or type a number (1-500)",
            Style::default().fg(MUTED),
        )),
    ];

    let content = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(content, inner);
}

/// Draws the enhanced detection configuration screen.
pub fn draw_enhanced_detection(frame: &mut Frame, area: Rect, selected: usize) {
    let content_area = centered_rect(area, 80, 85);

    let block = Block::default()
        .title(" Enhanced Detection ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let no_style = if selected == 0 {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let yes_style = if selected == 1 {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let lines = vec![
        Line::from(Span::styled(
            "Enable Enhanced Plaintext Detection?",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This uses an AI model to improve accuracy by ~40%",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "and reduces false positive prompts significantly.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled("Requirements:", Style::default().fg(ACCENT))),
        Line::from(Span::styled(
            "  - Downloads a ~500MB AI model",
            Style::default().fg(MUTED),
        )),
        Line::from(Span::styled(
            "  - Requires a HuggingFace account and READ token",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "You can enable this later with: ciphey --enable-enhanced-detection",
            Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(if selected == 0 { " > " } else { "   " }, no_style),
            Span::styled("[N] ", no_style),
            Span::styled("No, skip for now", no_style),
        ]),
        Line::from(vec![
            Span::styled(if selected == 1 { " > " } else { "   " }, yes_style),
            Span::styled("[Y] ", yes_style),
            Span::styled("Yes, set it up", yes_style),
        ]),
    ];

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draws the token input screen.
pub fn draw_token_input(frame: &mut Frame, area: Rect, token: &str, _cursor: usize) {
    let content_area = centered_rect(area, 80, 70);

    let block = Block::default()
        .title(" HuggingFace Token ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    // Mask the token
    let masked: String = "*".repeat(token.len());

    let lines = vec![
        Line::from(Span::styled(
            "Please enter your HuggingFace token:",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "1. Create account: https://huggingface.co/",
            Style::default().fg(MUTED),
        )),
        Line::from(Span::styled(
            "2. Create READ token: https://huggingface.co/settings/tokens",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your token will not be stored.",
            Style::default().fg(SUCCESS),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Token: ", Style::default().fg(ACCENT)),
            Span::styled(
                if masked.is_empty() {
                    "Enter token here..."
                } else {
                    &masked
                },
                if masked.is_empty() {
                    Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
            Span::styled("_", Style::default().fg(ACCENT)), // Cursor
        ]),
    ];

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draws the downloading screen with progress bar.
pub fn draw_downloading(
    frame: &mut Frame,
    area: Rect,
    progress: f32,
    status: &str,
    failed: bool,
    error: Option<&str>,
) {
    let content_area = centered_rect(area, 70, 60);

    let border_color = if failed { ERROR } else { Color::Gray };

    let block = Block::default()
        .title(if failed {
            " Download Failed "
        } else {
            " Downloading Model "
        })
        .title_style(
            Style::default()
                .fg(if failed { ERROR } else { ACCENT })
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    if failed {
        let mut lines = vec![
            Line::from(Span::styled(
                "Failed to download the AI model.",
                Style::default().fg(ERROR),
            )),
            Line::from(""),
        ];

        if let Some(err) = error {
            lines.push(Line::from(Span::styled(
                format!("Error: {}", err),
                Style::default().fg(MUTED),
            )));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "Enhanced detection will be disabled.",
            Style::default().fg(Color::White),
        )));
        lines.push(Line::from(Span::styled(
            "You can try again later with: ciphey --enable-enhanced-detection",
            Style::default().fg(MUTED),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press [Enter] to continue",
            Style::default().fg(ACCENT),
        )));

        let content = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(content, inner);
    } else {
        // Calculate progress bar
        let bar_width = 40;
        let filled = ((progress * bar_width as f32) as usize).min(bar_width);
        let empty = bar_width - filled;

        let progress_bar = format!(
            "[{}{}] {:.0}%",
            "=".repeat(filled),
            " ".repeat(empty),
            progress * 100.0
        );

        let lines = vec![
            Line::from(Span::styled(status, Style::default().fg(Color::White))),
            Line::from(""),
            Line::from(Span::styled(progress_bar, Style::default().fg(ACCENT))),
            Line::from(""),
            Line::from(Span::styled("Please wait...", Style::default().fg(MUTED))),
        ];

        let content = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(content, inner);
    }
}

/// Draws the cute cat question screen.
pub fn draw_cute_cat_question(frame: &mut Frame, area: Rect) {
    let content_area = centered_rect(area, 60, 50);

    let block = Block::default()
        .title(" One Last Thing... ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Do you want to see my cute cat?",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Y]",
                Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Yes please!", Style::default().fg(Color::White)),
            Span::styled("    ", Style::default()),
            Span::styled(
                "[N]",
                Style::default().fg(ERROR).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" No thanks", Style::default().fg(Color::White)),
        ]),
    ];

    let content = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(content, inner);
}

/// Draws the cute cat display screen (shown for 3 seconds).
pub fn draw_showing_cat(frame: &mut Frame, area: Rect) {
    let content_area = centered_rect(area, 60, 60);

    let block = Block::default()
        .title(" Meow! ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines = vec![];

    // Show the cute cat!
    for line in CAT_ART.lines() {
        lines.push(Line::from(Span::styled(line, Style::default().fg(ACCENT))));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Meet Ruhee!",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));

    let content = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(content, inner);
}

/// Draws the completion screen.
pub fn draw_complete(frame: &mut Frame, area: Rect, app: &SetupApp) {
    let content_area = centered_rect(area, 70, 80);

    let block = Block::default()
        .title(" Setup Complete! ")
        .title_style(Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(SUCCESS))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines = vec![];

    if app.show_cat {
        // Show the cute cat!
        for line in CAT_ART.lines() {
            lines.push(Line::from(Span::styled(line, Style::default().fg(ACCENT))));
        }
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "Ciphey is ready to use!",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Your preferences have been saved.",
        Style::default().fg(MUTED),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press [Enter] to start using Ciphey",
        Style::default().fg(ACCENT),
    )));

    let content = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(content, inner);
}
