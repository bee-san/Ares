//! UI rendering for the setup wizard.
//!
//! This module handles rendering each step of the setup wizard
//! with a beautiful card-based design.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use super::app::{SetupApp, SetupState, WordlistFocus, TOTAL_STEPS};

// Submodules
pub mod ai;
pub mod colors;
pub mod quick_searches;
pub mod summary;
pub mod tutorial;
pub mod welcome;
pub mod wordlist;

// Re-export commonly used functions
pub use ai::draw_ai_config;
pub use colors::draw_theme_selection;
pub use quick_searches::draw_quick_searches;
pub use summary::{
    draw_complete, draw_cute_cat_question, draw_downloading, draw_enhanced_detection,
    draw_results_mode, draw_showing_cat, draw_timeout_config, draw_token_input,
};
pub use tutorial::draw_tutorial;
pub use welcome::draw_welcome;
pub use wordlist::draw_wordlist_config;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Muted text color
const MUTED: Color = Color::DarkGray;
/// Success color
const SUCCESS: Color = Color::Rgb(80, 250, 123);

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
            custom_paths,
            current_input,
            cursor,
            selected_predefined,
            focus,
            custom_url,
            custom_url_source,
            download_progress,
        } => draw_wordlist_config(
            frame,
            main_chunks[1],
            custom_paths,
            current_input,
            *cursor,
            selected_predefined,
            focus,
            custom_url,
            custom_url_source,
            download_progress.as_ref(),
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
        SetupState::ShowingCat => draw_showing_cat(frame, main_chunks[1]),
        SetupState::AiConfig {
            selected,
            api_url,
            api_key,
            model,
            focus,
            cursor,
        } => draw_ai_config(
            frame,
            main_chunks[1],
            *selected,
            api_url,
            api_key,
            model,
            focus,
            *cursor,
        ),
        SetupState::Complete => draw_complete(frame, main_chunks[1], app),
        SetupState::QuickSearches {
            entries,
            selected,
            current_input,
            cursor,
        } => draw_quick_searches(
            frame,
            main_chunks[1],
            entries,
            *selected,
            current_input,
            *cursor,
        ),
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
        SetupState::WordlistConfig { focus, .. } => match focus {
            WordlistFocus::PredefinedList { .. } => vec![
                ("[Space]", "Toggle"),
                ("[Tab]", "Next"),
                ("[1-2]", "Quick"),
                ("[Esc]", "Back"),
            ],
            WordlistFocus::CustomInput
            | WordlistFocus::CustomUrlInput
            | WordlistFocus::CustomUrlSource => {
                vec![("[Enter]", "Add"), ("[Tab]", "Next"), ("[Esc]", "Clear")]
            }
            WordlistFocus::Done => vec![("[Enter]", "Continue"), ("[Tab]", "Back")],
        },
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
        SetupState::AiConfig {
            selected, focus, ..
        } => {
            use super::ui::ai::AiConfigFocus;
            if *selected == 1 && *focus != AiConfigFocus::EnableToggle {
                vec![
                    ("[Tab]", "Next Field"),
                    ("[Enter]", "Confirm"),
                    ("[Esc]", "Back"),
                ]
            } else {
                vec![
                    ("[Y/N]", "Choose"),
                    ("[Enter]", "Confirm"),
                    ("[Backspace]", "Back"),
                ]
            }
        }
        SetupState::ShowingCat => vec![("", "Admiring cat...")],
        SetupState::QuickSearches { .. } => vec![
            ("[j/k]", "Navigate"),
            ("[Del]", "Remove"),
            ("[Enter]", "Add/Continue"),
            ("[Esc]", "Back"),
        ],
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
