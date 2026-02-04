//! UI rendering for the setup wizard.
//!
//! This module handles rendering each step of the setup wizard
//! with a beautiful card-based design.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use super::app::{CustomColors, SetupApp, SetupState, TOTAL_STEPS};
use super::themes::{ColorScheme, THEMES};

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

/// Main draw function for the setup wizard.
pub fn draw_setup(frame: &mut Frame, app: &SetupApp) {
    let area = frame.area();

    // Draw outer border with title
    let outer_block = Block::default()
        .title(" Ciphey Setup ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray));

    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Main layout: progress bar at top, content in middle, controls at bottom
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Progress bar
            Constraint::Min(10),   // Content
            Constraint::Length(2), // Controls
        ])
        .split(inner_area);

    // Draw progress indicator
    draw_progress_bar(frame, main_chunks[0], app);

    // Draw state-specific content
    match &app.state {
        SetupState::Welcome => draw_welcome(frame, main_chunks[1], app),
        SetupState::Tutorial => draw_tutorial(frame, main_chunks[1]),
        SetupState::ThemeSelection {
            selected,
            custom_mode,
            custom_colors,
            custom_field,
        } => draw_theme_selection(
            frame,
            main_chunks[1],
            *selected,
            *custom_mode,
            custom_colors,
            *custom_field,
        ),
        SetupState::ResultsMode { selected } => draw_results_mode(frame, main_chunks[1], *selected),
        SetupState::TimeoutConfig { value, .. } => {
            draw_timeout_config(frame, main_chunks[1], *value)
        }
        SetupState::WordlistConfig {
            paths,
            current_input,
            cursor,
            input_focused,
        } => draw_wordlist_config(
            frame,
            main_chunks[1],
            paths,
            current_input,
            *cursor,
            *input_focused,
        ),
        SetupState::EnhancedDetection { selected } => {
            draw_enhanced_detection(frame, main_chunks[1], *selected)
        }
        SetupState::TokenInput { token, cursor } => {
            draw_token_input(frame, main_chunks[1], token, *cursor)
        }
        SetupState::Downloading {
            progress,
            status,
            failed,
            error,
        } => draw_downloading(
            frame,
            main_chunks[1],
            *progress,
            status,
            *failed,
            error.as_deref(),
        ),
        SetupState::CuteCat => draw_cute_cat_question(frame, main_chunks[1]),
        SetupState::Complete => draw_complete(frame, main_chunks[1], app),
    }

    // Draw controls at bottom
    draw_controls(frame, main_chunks[2], &app.state);
}

/// Draws the progress bar at the top.
fn draw_progress_bar(frame: &mut Frame, area: Rect, app: &SetupApp) {
    let current = app.current_step();

    // Create progress dots
    let mut spans = vec![Span::styled("  ", Style::default())];

    for i in 1..=TOTAL_STEPS {
        let (symbol, style) = if i < current {
            ("", Style::default().fg(SUCCESS))
        } else if i == current {
            ("", Style::default().fg(ACCENT))
        } else {
            ("", Style::default().fg(MUTED))
        };

        spans.push(Span::styled(symbol, style));

        if i < TOTAL_STEPS {
            let connector_style = if i < current {
                Style::default().fg(SUCCESS)
            } else {
                Style::default().fg(MUTED)
            };
            spans.push(Span::styled("───", connector_style));
        }
    }

    spans.push(Span::styled(
        format!("  Step {}/{}", current, TOTAL_STEPS),
        Style::default().fg(MUTED),
    ));

    let progress_line = Line::from(spans);
    let progress = Paragraph::new(progress_line).alignment(Alignment::Center);

    // Add a subtle box around progress
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(block, area);
    frame.render_widget(
        progress,
        Rect {
            y: area.y + 1,
            ..area
        },
    );
}

/// Draws the welcome screen with ASCII art.
fn draw_welcome(frame: &mut Frame, area: Rect, _app: &SetupApp) {
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

/// Draws the tutorial screen.
fn draw_tutorial(frame: &mut Frame, area: Rect) {
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

/// Draws the theme selection screen with live preview.
fn draw_theme_selection(
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
        Line::from(Span::styled("Questions", scheme.question_style())),
        Line::from(Span::styled(
            "  Interactive prompts",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled("Statements", scheme.statement_style())),
        Line::from(Span::styled(
            "  General output text",
            Style::default().fg(MUTED),
        )),
    ];

    let preview = Paragraph::new(lines);
    frame.render_widget(preview, inner);
}

/// Draws the results mode selection screen.
fn draw_results_mode(frame: &mut Frame, area: Rect, selected: usize) {
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
fn draw_timeout_config(frame: &mut Frame, area: Rect, value: u32) {
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

/// Draws the wordlist configuration screen.
fn draw_wordlist_config(
    frame: &mut Frame,
    area: Rect,
    paths: &[String],
    current_input: &str,
    _cursor: usize,
    input_focused: bool,
) {
    let content_area = centered_rect(area, 85, 85);

    let block = Block::default()
        .title(" Wordlist Configuration ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines = vec![
        Line::from(Span::styled(
            "Add custom wordlists for plaintext detection (optional)",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "Wordlists help identify plaintext by matching against known words.",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
    ];

    // Show added wordlists
    if !paths.is_empty() {
        lines.push(Line::from(Span::styled(
            "Added wordlists:",
            Style::default().fg(SECONDARY),
        )));
        for (i, path) in paths.iter().enumerate() {
            // Truncate long paths for display
            let display_path = if path.len() > 50 {
                format!("...{}", &path[path.len() - 47..])
            } else {
                path.clone()
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {}. ", i + 1), Style::default().fg(SUCCESS)),
                Span::styled(display_path, Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Input field
    let input_style = if input_focused {
        Style::default().fg(ACCENT)
    } else {
        Style::default().fg(MUTED)
    };

    let input_border = if input_focused { "> " } else { "  " };

    let display_input = if current_input.is_empty() {
        "Enter path to wordlist file..."
    } else {
        current_input
    };

    let text_style = if current_input.is_empty() {
        Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)
    } else {
        Style::default().fg(Color::White)
    };

    lines.push(Line::from(vec![
        Span::styled(input_border, input_style),
        Span::styled("Path: ", Style::default().fg(SECONDARY)),
        Span::styled(display_input, text_style),
        if input_focused {
            Span::styled("_", Style::default().fg(ACCENT))
        } else {
            Span::styled("", Style::default())
        },
    ]));

    lines.push(Line::from(""));

    // Done button
    let done_style = if !input_focused {
        Style::default()
            .fg(Color::Black)
            .bg(SUCCESS)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(SUCCESS)
    };

    let done_prefix = if !input_focused { "> " } else { "  " };

    lines.push(Line::from(vec![
        Span::styled(
            done_prefix,
            if !input_focused {
                Style::default().fg(SUCCESS)
            } else {
                Style::default().fg(MUTED)
            },
        ),
        Span::styled(" Done ", done_style),
        Span::styled(
            if paths.is_empty() {
                " (skip wordlists)"
            } else {
                ""
            },
            Style::default().fg(MUTED),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter] Add path  [Down/Tab] Go to Done  [Esc] Remove last/Back",
        Style::default().fg(MUTED),
    )));

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draws the enhanced detection configuration screen.
fn draw_enhanced_detection(frame: &mut Frame, area: Rect, selected: usize) {
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
        Line::from(Span::styled(
            "Requirements:",
            Style::default().fg(SECONDARY),
        )),
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
fn draw_token_input(frame: &mut Frame, area: Rect, token: &str, _cursor: usize) {
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
            Span::styled("Token: ", Style::default().fg(SECONDARY)),
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
fn draw_downloading(
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
fn draw_cute_cat_question(frame: &mut Frame, area: Rect) {
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
            "Do you want to see a cute cat?",
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

/// Draws the completion screen.
fn draw_complete(frame: &mut Frame, area: Rect, app: &SetupApp) {
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

/// Draws the control hints at the bottom of the screen.
fn draw_controls(frame: &mut Frame, area: Rect, state: &SetupState) {
    let controls = match state {
        SetupState::Welcome => vec![("[Enter]", "Begin"), ("[S]", "Skip"), ("[Q]", "Quit")],
        SetupState::Tutorial => vec![
            ("[Enter]", "Next"),
            ("[Backspace]", "Back"),
            ("[S]", "Skip"),
        ],
        SetupState::ThemeSelection { custom_mode, .. } => {
            if *custom_mode {
                vec![
                    ("[Tab]", "Next Field"),
                    ("[Enter]", "Confirm"),
                    ("[Esc]", "Cancel"),
                ]
            } else {
                vec![
                    ("[j/k]", "Navigate"),
                    ("[Enter]", "Select"),
                    ("[Backspace]", "Back"),
                ]
            }
        }
        SetupState::ResultsMode { .. } => vec![
            ("[j/k]", "Navigate"),
            ("[Enter]", "Select"),
            ("[Backspace]", "Back"),
        ],
        SetupState::TimeoutConfig { .. } => vec![
            ("[Up/Down]", "Adjust"),
            ("[Enter]", "Confirm"),
            ("[Backspace]", "Back"),
        ],
        SetupState::WordlistConfig { input_focused, .. } => {
            if *input_focused {
                vec![("[Enter]", "Add"), ("[Down]", "Done"), ("[Esc]", "Remove")]
            } else {
                vec![("[Enter]", "Continue"), ("[Up]", "Add more")]
            }
        }
        SetupState::EnhancedDetection { .. } => vec![
            ("[Y/N]", "Choose"),
            ("[Enter]", "Confirm"),
            ("[Backspace]", "Back"),
        ],
        SetupState::TokenInput { .. } => vec![("[Enter]", "Submit"), ("[Esc]", "Back")],
        SetupState::Downloading { failed, .. } => {
            if *failed {
                vec![("[Enter]", "Continue"), ("[Q]", "Quit")]
            } else {
                vec![("", "Please wait...")]
            }
        }
        SetupState::CuteCat => vec![("[Y]", "Yes!"), ("[N]", "No"), ("[Backspace]", "Back")],
        SetupState::Complete => vec![("[Enter]", "Finish")],
    };

    let mut spans = Vec::new();
    for (i, (key, desc)) in controls.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  |  ", Style::default().fg(MUTED)));
        }
        if !key.is_empty() {
            spans.push(Span::styled(
                *key,
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {}", desc),
                Style::default().fg(Color::White),
            ));
        } else {
            spans.push(Span::styled(*desc, Style::default().fg(MUTED)));
        }
    }

    let controls_line = Line::from(spans);
    let controls_paragraph = Paragraph::new(controls_line).alignment(Alignment::Center);
    frame.render_widget(controls_paragraph, area);
}

/// Creates a centered rectangle within the given area.
fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    Rect::new(x, y, popup_width, popup_height)
}
