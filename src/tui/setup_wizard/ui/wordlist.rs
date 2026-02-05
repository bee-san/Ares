//! Wordlist configuration screen rendering for the setup wizard.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use super::super::app::{DownloadProgress, WordlistFocus};
use super::tutorial::centered_rect;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Secondary color (cyan)
const SECONDARY: Color = Color::Rgb(139, 233, 253);
/// Muted text color
const MUTED: Color = Color::DarkGray;
/// Success color
const SUCCESS: Color = Color::Rgb(80, 250, 123);
/// Error color
const ERROR: Color = Color::Rgb(255, 85, 85);

/// Draws the wordlist configuration screen.
#[allow(clippy::too_many_arguments)]
pub fn draw_wordlist_config(
    frame: &mut Frame,
    area: Rect,
    custom_paths: &[String],
    current_input: &str,
    _cursor: usize,
    selected_predefined: &[usize],
    focus: &WordlistFocus,
    custom_url: &str,
    custom_url_source: &str,
    download_progress: Option<&DownloadProgress>,
) {
    // If downloading, show progress overlay
    if let Some(progress) = download_progress {
        draw_wordlist_download_progress(frame, area, progress);
        return;
    }

    let content_area = centered_rect(area, 90, 90);

    let block = Block::default()
        .title(" Wordlist Configuration ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    // Split into left and right panels
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    // Left panel: Predefined wordlists
    draw_predefined_wordlists(frame, panels[0], selected_predefined, focus);

    // Right panel: Custom options
    draw_custom_wordlists(
        frame,
        panels[1],
        custom_paths,
        current_input,
        custom_url,
        custom_url_source,
        focus,
    );
}

/// Draws the predefined wordlists panel.
fn draw_predefined_wordlists(
    frame: &mut Frame,
    area: Rect,
    selected_predefined: &[usize],
    focus: &WordlistFocus,
) {
    let mut lines = vec![
        Line::from(Span::styled(
            "Predefined Wordlists",
            Style::default().fg(SECONDARY).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Select wordlists to download",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
    ];

    let predefined_wordlists = crate::storage::download::get_predefined_wordlists();
    let is_focused = matches!(focus, WordlistFocus::PredefinedList);

    for (i, wordlist) in predefined_wordlists.iter().enumerate() {
        let is_selected = selected_predefined.contains(&i);
        let checkbox = if is_selected { "[✓]" } else { "[ ]" };

        let prefix = if is_focused && i == 0 { "> " } else { "  " };

        let style = if is_focused && i == 0 {
            Style::default().fg(ACCENT)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(
                format!("{} ", checkbox),
                if is_selected {
                    Style::default().fg(SUCCESS)
                } else {
                    Style::default().fg(MUTED)
                },
            ),
            Span::styled(format!("{}. {}", i + 1, wordlist.name), style),
        ]));

        lines.push(Line::from(Span::styled(
            format!("     {}", wordlist.description),
            Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
        )));
        lines.push(Line::from(""));
    }

    if predefined_wordlists.is_empty() {
        lines.push(Line::from(Span::styled(
            "No predefined wordlists available",
            Style::default().fg(MUTED),
        )));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, area);
}

/// Draws the custom wordlists panel.
#[allow(clippy::too_many_arguments)]
fn draw_custom_wordlists(
    frame: &mut Frame,
    area: Rect,
    custom_paths: &[String],
    current_input: &str,
    custom_url: &str,
    custom_url_source: &str,
    focus: &WordlistFocus,
) {
    let mut lines = vec![
        Line::from(Span::styled(
            "Custom Options",
            Style::default().fg(SECONDARY).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Section 1: Custom File Paths
    lines.push(Line::from(Span::styled(
        "File Paths:",
        Style::default().fg(Color::White),
    )));

    if !custom_paths.is_empty() {
        for (i, path) in custom_paths.iter().enumerate() {
            let display_path = if path.len() > 35 {
                format!("...{}", &path[path.len() - 32..])
            } else {
                path.clone()
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {}. ", i + 1), Style::default().fg(SUCCESS)),
                Span::styled(display_path, Style::default().fg(Color::White)),
            ]));
        }
    }

    // Input field for custom path
    let is_path_focused = matches!(focus, WordlistFocus::CustomInput);
    let path_prefix = if is_path_focused { "> " } else { "  " };

    let display_input = if current_input.is_empty() {
        "Enter file path..."
    } else {
        current_input
    };

    let text_style = if current_input.is_empty() {
        Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
    } else {
        Style::default().fg(Color::White)
    };

    lines.push(Line::from(vec![
        Span::styled(
            path_prefix,
            if is_path_focused {
                Style::default().fg(ACCENT)
            } else {
                Style::default().fg(MUTED)
            },
        ),
        Span::styled(display_input, text_style),
        if is_path_focused {
            Span::styled("_", Style::default().fg(ACCENT))
        } else {
            Span::styled("", Style::default())
        },
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Section 2: Custom URLs
    lines.push(Line::from(Span::styled(
        "Custom URL:",
        Style::default().fg(Color::White),
    )));

    let is_url_focused = matches!(focus, WordlistFocus::CustomUrlInput);
    let url_prefix = if is_url_focused { "> " } else { "  " };

    let display_url = if custom_url.is_empty() {
        "Enter URL..."
    } else {
        custom_url
    };

    let url_text_style = if custom_url.is_empty() {
        Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
    } else {
        Style::default().fg(Color::White)
    };

    lines.push(Line::from(vec![
        Span::styled(
            url_prefix,
            if is_url_focused {
                Style::default().fg(ACCENT)
            } else {
                Style::default().fg(MUTED)
            },
        ),
        Span::styled(display_url, url_text_style),
        if is_url_focused {
            Span::styled("_", Style::default().fg(ACCENT))
        } else {
            Span::styled("", Style::default())
        },
    ]));

    // Source name input (shown if URL is not empty)
    if !custom_url.is_empty() {
        let is_source_focused = matches!(focus, WordlistFocus::CustomUrlSource);
        let source_prefix = if is_source_focused { "> " } else { "  " };

        let display_source = if custom_url_source.is_empty() {
            "Source name..."
        } else {
            custom_url_source
        };

        let source_text_style = if custom_url_source.is_empty() {
            Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(
                source_prefix,
                if is_source_focused {
                    Style::default().fg(ACCENT)
                } else {
                    Style::default().fg(MUTED)
                },
            ),
            Span::styled("Name: ", Style::default().fg(MUTED)),
            Span::styled(display_source, source_text_style),
            if is_source_focused {
                Span::styled("_", Style::default().fg(ACCENT))
            } else {
                Span::styled("", Style::default())
            },
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Done button
    let is_done_focused = matches!(focus, WordlistFocus::Done);
    let done_style = if is_done_focused {
        Style::default()
            .fg(Color::Black)
            .bg(SUCCESS)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(SUCCESS)
    };

    let done_prefix = if is_done_focused { "> " } else { "  " };

    lines.push(Line::from(vec![
        Span::styled(
            done_prefix,
            if is_done_focused {
                Style::default().fg(SUCCESS)
            } else {
                Style::default().fg(MUTED)
            },
        ),
        Span::styled(" Done ", done_style),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Space/Enter] Toggle  [Tab] Next  [1-2] Quick toggle  [Esc] Back",
        Style::default().fg(MUTED),
    )));

    let content = Paragraph::new(lines);
    frame.render_widget(content, area);
}

/// Draws the download progress overlay.
fn draw_wordlist_download_progress(frame: &mut Frame, area: Rect, progress: &DownloadProgress) {
    let overlay_area = centered_rect(area, 60, 40);

    let block = Block::default()
        .title(" Downloading Wordlists ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(overlay_area);
    frame.render_widget(block, overlay_area);

    let mut lines = vec![
        Line::from(Span::styled(
            format!("Downloading {} of {}", progress.current, progress.total),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            &progress.status,
            Style::default().fg(SECONDARY),
        )),
        Line::from(""),
    ];

    // Progress bar
    let progress_pct = if progress.total > 0 {
        (progress.current as f32 / progress.total as f32 * 100.0) as u16
    } else {
        0
    };

    let bar_width = 40;
    let filled =
        (bar_width as f32 * progress.current as f32 / progress.total.max(1) as f32) as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}] {}%",
        "=".repeat(filled),
        " ".repeat(empty),
        progress_pct
    );

    lines.push(Line::from(Span::styled(bar, Style::default().fg(ACCENT))));
    lines.push(Line::from(""));

    // Show failures if any
    if !progress.failed.is_empty() {
        lines.push(Line::from(Span::styled(
            "Failed downloads:",
            Style::default().fg(ERROR),
        )));
        for failure in &progress.failed {
            lines.push(Line::from(Span::styled(
                format!("  • {}", failure),
                Style::default().fg(MUTED),
            )));
        }
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
