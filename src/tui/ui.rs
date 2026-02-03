//! Main UI rendering for the Ciphey TUI.
//!
//! This module handles rendering the terminal user interface based on the current
//! application state. It provides the main `draw` function that dispatches to
//! state-specific renderers and handles overlay rendering.

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use super::app::{App, AppState};
use super::colors::TuiColors;
use super::spinner::Spinner;
use super::widgets::{render_step_details, render_text_panel, PathViewer};

/// Main draw function that renders the TUI based on current application state.
///
/// This function is called on each frame to render the appropriate screen based
/// on the current [`AppState`]. It handles:
///
/// - [`AppState::Loading`]: Centered spinner with rotating quotes
/// - [`AppState::Results`]: Three-column layout with input, path, and output
/// - [`AppState::Failure`]: Failure message with tips
///
/// Additionally, it renders overlays such as the help popup and status messages.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `app` - Reference to the application state
/// * `colors` - The color scheme to use for rendering
pub fn draw(frame: &mut Frame, app: &App, colors: &TuiColors) {
    let area = frame.area();

    // Render the appropriate screen based on state
    match &app.state {
        AppState::Loading {
            start_time,
            current_quote,
            spinner_frame,
        } => {
            draw_loading_screen(
                frame,
                area,
                *spinner_frame,
                *current_quote,
                start_time,
                colors,
            );
        }
        AppState::Results {
            result,
            selected_step,
            ..
        } => {
            draw_results_screen(frame, area, &app.input_text, result, *selected_step, colors);
        }
        AppState::Failure {
            input_text,
            elapsed,
        } => {
            draw_failure_screen(frame, area, input_text, *elapsed, colors);
        }
    }

    // Render help overlay if visible
    if app.show_help {
        draw_help_overlay(frame, area, colors);
    }

    // Render status message if present
    if let Some(ref msg) = app.status_message {
        draw_status_message(frame, area, msg, colors);
    }
}

/// Renders the loading screen with spinner and rotating quote.
///
/// Displays a centered panel with:
/// - "Decrypting..." title
/// - Animated spinner
/// - Current cryptography quote with attribution
/// - Elapsed time
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `spinner_frame` - Current animation frame index for the spinner
/// * `quote_index` - Current index into the quotes array
/// * `start_time` - When the loading started, for elapsed time calculation
/// * `colors` - The color scheme to use
fn draw_loading_screen(
    frame: &mut Frame,
    area: Rect,
    spinner_frame: usize,
    quote_index: usize,
    start_time: &std::time::Instant,
    colors: &TuiColors,
) {
    // Create outer block with title
    let outer_block = Block::default()
        .title(" Ciphey ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    frame.render_widget(outer_block, area);

    // Create a centered content area
    let inner_area = centered_rect(area, 80, 60);

    // Create spinner with current frame
    let mut spinner = Spinner::new();
    for _ in 0..spinner_frame {
        spinner.tick();
    }
    for _ in 0..quote_index {
        spinner.next_quote();
    }

    // Calculate elapsed time
    let elapsed = start_time.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();

    // Build the content
    let title_line = Line::from(Span::styled("Decrypting...", colors.highlight));

    let spinner_line = Line::from(Span::styled(spinner.current_frame(), colors.accent));

    // Parse quote and attribution
    let quote_text = spinner.current_quote();
    let (quote, attribution) = parse_quote(quote_text);

    let quote_line = Line::from(Span::styled(format!("\"{}\"", quote), colors.text));

    let attribution_line = if !attribution.is_empty() {
        Line::from(Span::styled(format!("- {}", attribution), colors.muted))
    } else {
        Line::from("")
    };

    let elapsed_line = Line::from(Span::styled(
        format!("Elapsed: {:.1}s", elapsed_secs),
        colors.muted,
    ));

    // Combine all lines with spacing
    let lines = vec![
        Line::from(""),
        title_line,
        Line::from(""),
        spinner_line,
        Line::from(""),
        quote_line,
        attribution_line,
        Line::from(""),
        elapsed_line,
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner_area);
}

/// Renders the results screen with three-column layout.
///
/// Layout:
/// - Top row: Input panel | Path viewer | Output panel
/// - Middle: Step details panel
/// - Bottom: Status bar with keybindings
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `input_text` - The original input text
/// * `result` - The successful decoding result
/// * `selected_step` - Index of the currently selected step
/// * `colors` - The color scheme to use
fn draw_results_screen(
    frame: &mut Frame,
    area: Rect,
    input_text: &str,
    result: &crate::DecoderResult,
    selected_step: usize,
    colors: &TuiColors,
) {
    // Calculate layout chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Top row (Input | Path | Output)
            Constraint::Min(8),    // Step details
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Split top row into three columns
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Input
            Constraint::Percentage(50), // Path
            Constraint::Percentage(25), // Output
        ])
        .split(chunks[0]);

    // Render input panel
    render_text_panel(
        top_chunks[0],
        frame.buffer_mut(),
        "Input",
        input_text,
        colors,
        false,
    );

    // Render path viewer
    let path_block = Block::default()
        .title(" Path ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    let path_inner = path_block.inner(top_chunks[1]);
    frame.render_widget(path_block, top_chunks[1]);

    let path_viewer = PathViewer::new();
    path_viewer.render(
        path_inner,
        frame.buffer_mut(),
        &result.path,
        selected_step,
        colors,
    );

    // Render output panel
    let output_text = result
        .text
        .first()
        .map(|s| s.as_str())
        .unwrap_or("(no output)");

    render_text_panel(
        top_chunks[2],
        frame.buffer_mut(),
        "Output",
        output_text,
        colors,
        true,
    );

    // Render step details
    let current_step = result.path.get(selected_step);
    render_step_details(chunks[1], frame.buffer_mut(), current_step, colors);

    // Render status bar
    draw_status_bar(frame, chunks[2], colors);
}

/// Renders the failure screen with tips.
///
/// Displays a centered panel with:
/// - "No solution found" message
/// - The original input
/// - Time spent trying
/// - Helpful tips for the user
/// - Exit instruction
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `input_text` - The original input that could not be decoded
/// * `elapsed` - How long the decoding attempt took
/// * `colors` - The color scheme to use
fn draw_failure_screen(
    frame: &mut Frame,
    area: Rect,
    input_text: &str,
    elapsed: std::time::Duration,
    colors: &TuiColors,
) {
    // Create outer block with title
    let outer_block = Block::default()
        .title(" Ciphey ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    frame.render_widget(outer_block, area);

    // Create inner area for content
    let inner_area = centered_rect(area, 80, 70);

    // Truncate input if too long
    let display_input = if input_text.len() > 50 {
        format!("{}...", &input_text[..50])
    } else {
        input_text.to_string()
    };

    let elapsed_secs = elapsed.as_secs_f64();

    // Build content lines
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "No solution found",
            colors.error.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Input: ", colors.label),
            Span::styled(display_input, colors.text),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Tried for {:.1} seconds", elapsed_secs),
            colors.muted,
        )),
        Line::from(""),
        Line::from(Span::styled("Tips:", colors.highlight)),
        Line::from(Span::styled(
            "- Try using --enable-enhanced-detection for complex ciphers",
            colors.text,
        )),
        Line::from(Span::styled(
            "- Check if the input is valid encoded/encrypted text",
            colors.text,
        )),
        Line::from(""),
        Line::from(Span::styled("Press 'q' to exit", colors.muted)),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner_area);
}

/// Renders the status bar with keybinding hints.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render the status bar
/// * `colors` - The color scheme to use
fn draw_status_bar(frame: &mut Frame, area: Rect, colors: &TuiColors) {
    let keybindings = [
        ("[q]", "Quit"),
        ("[←/→]", "Navigate"),
        ("[c]", "Copy"),
        ("[?]", "Help"),
    ];

    let mut spans = Vec::new();
    for (i, (key, desc)) in keybindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", colors.text));
        }
        spans.push(Span::styled(*key, colors.accent));
        spans.push(Span::styled(format!(" {}", desc), colors.muted));
    }

    let status = Paragraph::new(Line::from(spans));
    frame.render_widget(status, area);
}

/// Renders the help overlay popup.
///
/// Shows all available keybindings in a centered popup on top of the current screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The full screen area
/// * `colors` - The color scheme to use
fn draw_help_overlay(frame: &mut Frame, area: Rect, colors: &TuiColors) {
    // Calculate popup size and position
    let popup_area = centered_rect(area, 50, 60);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    // Create the popup block
    let block = Block::default()
        .title(" Help ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.accent);

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Build help content
    let keybindings = vec![
        ("q / Esc", "Quit the application"),
        ("← / h", "Select previous step"),
        ("→ / l", "Select next step"),
        ("c", "Copy output to clipboard"),
        ("C", "Copy full path to clipboard"),
        ("?", "Toggle this help overlay"),
        ("↑ / k", "Scroll up"),
        ("↓ / j", "Scroll down"),
    ];

    let mut lines = vec![
        Line::from(Span::styled(
            "Keybindings",
            colors.highlight.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (key, description) in keybindings {
        lines.push(Line::from(vec![
            Span::styled(format!("{:12}", key), colors.accent),
            Span::styled(description, colors.text),
        ]));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner_area);
}

/// Renders a status message at the bottom of the screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The full screen area
/// * `message` - The status message to display
/// * `colors` - The color scheme to use
fn draw_status_message(frame: &mut Frame, area: Rect, message: &str, colors: &TuiColors) {
    // Position at the bottom of the screen
    let msg_area = Rect {
        x: area.x + 1,
        y: area.y + area.height.saturating_sub(2),
        width: area.width.saturating_sub(2),
        height: 1,
    };

    let paragraph = Paragraph::new(Span::styled(message, colors.success));
    frame.render_widget(paragraph, msg_area);
}

/// Creates a centered rectangle within the given area.
///
/// # Arguments
///
/// * `area` - The outer area to center within
/// * `percent_x` - Width as a percentage of the outer area (0-100)
/// * `percent_y` - Height as a percentage of the outer area (0-100)
///
/// # Returns
///
/// A [`Rect`] that is centered within the given area with the specified dimensions.
///
/// # Example
///
/// ```ignore
/// let outer = Rect::new(0, 0, 100, 50);
/// let centered = centered_rect(outer, 50, 50);
/// // centered is a 50x25 rect centered within outer
/// ```
pub fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    Rect::new(x, y, popup_width, popup_height)
}

/// Parses a quote string into quote and attribution parts.
///
/// Expects format: "Quote text - Attribution" or just "Quote text"
///
/// # Arguments
///
/// * `quote_text` - The full quote string
///
/// # Returns
///
/// A tuple of (quote, attribution) where attribution may be empty.
fn parse_quote(quote_text: &str) -> (&str, &str) {
    if let Some(dash_pos) = quote_text.rfind(" - ") {
        let quote = &quote_text[..dash_pos];
        let attribution = &quote_text[dash_pos + 3..];
        (quote, attribution)
    } else {
        (quote_text, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_rect_basic() {
        let area = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(area, 50, 50);

        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 25);
        assert_eq!(centered.x, 25);
        assert_eq!(centered.y, 12);
    }

    #[test]
    fn test_centered_rect_full_size() {
        let area = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(area, 100, 100);

        assert_eq!(centered.width, 100);
        assert_eq!(centered.height, 50);
        assert_eq!(centered.x, 0);
        assert_eq!(centered.y, 0);
    }

    #[test]
    fn test_centered_rect_small() {
        let area = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(area, 10, 10);

        assert_eq!(centered.width, 10);
        assert_eq!(centered.height, 5);
        assert_eq!(centered.x, 45);
        assert_eq!(centered.y, 22);
    }

    #[test]
    fn test_centered_rect_with_offset() {
        let area = Rect::new(10, 5, 100, 50);
        let centered = centered_rect(area, 50, 50);

        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 25);
        assert_eq!(centered.x, 35); // 10 + (100-50)/2 = 10 + 25
        assert_eq!(centered.y, 17); // 5 + (50-25)/2 = 5 + 12
    }

    #[test]
    fn test_parse_quote_with_attribution() {
        let (quote, attribution) = parse_quote("Some quote here - Author Name");
        assert_eq!(quote, "Some quote here");
        assert_eq!(attribution, "Author Name");
    }

    #[test]
    fn test_parse_quote_without_attribution() {
        let (quote, attribution) = parse_quote("Just a quote without attribution");
        assert_eq!(quote, "Just a quote without attribution");
        assert_eq!(attribution, "");
    }

    #[test]
    fn test_parse_quote_with_multiple_dashes() {
        let (quote, attribution) = parse_quote("Quote with dash-in-middle - The Author");
        assert_eq!(quote, "Quote with dash-in-middle");
        assert_eq!(attribution, "The Author");
    }
}
