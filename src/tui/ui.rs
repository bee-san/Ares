//! Main UI rendering for the Ciphey TUI.
//!
//! This module handles rendering the terminal user interface based on the current
//! application state. It provides the main `draw` function that dispatches to
//! state-specific renderers and handles overlay rendering.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

use super::app::{App, AppState, HumanConfirmationRequest, WordlistManagerFocus};
use super::colors::TuiColors;
use super::settings::SettingsModel;
use super::spinner::{Spinner, ENHANCED_SPINNER_FRAMES};
use super::widgets::{
    render_list_editor, render_settings_screen as render_settings_panel, render_step_details,
    render_text_panel, render_wordlist_manager, PathViewer, WordlistFocus,
};

/// Modal width as percentage of screen width.
const MODAL_WIDTH_PERCENT: u16 = 65;
/// Modal height as percentage of screen height.
const MODAL_HEIGHT_PERCENT: u16 = 55;
/// Maximum plaintext preview length before truncation.
const MAX_PLAINTEXT_PREVIEW_LEN: usize = 200;
/// Help overlay width as percentage of screen.
const HELP_WIDTH_PERCENT: u16 = 50;
/// Help overlay height as percentage of screen.
const HELP_HEIGHT_PERCENT: u16 = 60;
/// Loading screen content width percentage.
const LOADING_WIDTH_PERCENT: u16 = 80;
/// Loading screen content height percentage.
const LOADING_HEIGHT_PERCENT: u16 = 70;

/// Decorated title for Ciphey using box drawing characters.
const DECORATED_TITLE: &str = " ══ Ciphey ══ ";

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
        AppState::HumanConfirmation {
            start_time,
            current_quote,
            spinner_frame,
            request,
            response_sender: _,
        } => {
            // Draw loading screen in background
            draw_loading_screen(
                frame,
                area,
                *spinner_frame,
                *current_quote,
                start_time,
                colors,
            );
            // Draw confirmation modal on top
            draw_human_confirmation_screen(frame, area, request, colors);
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
        AppState::Settings {
            settings,
            selected_section,
            selected_field,
            editing_mode,
            input_buffer,
            cursor_pos,
            scroll_offset,
            validation_errors,
            ..
        } => {
            draw_settings_screen(
                frame,
                area,
                settings,
                *selected_section,
                *selected_field,
                *editing_mode,
                input_buffer,
                *cursor_pos,
                *scroll_offset,
                validation_errors,
                settings.has_changes(),
                colors,
            );
        }
        AppState::ListEditor {
            field_label,
            items,
            selected_item,
            input_buffer,
            cursor_pos,
            ..
        } => {
            draw_list_editor_screen(
                frame,
                area,
                field_label,
                items,
                *selected_item,
                input_buffer,
                *cursor_pos,
                colors,
            );
        }
        AppState::WordlistManager {
            wordlist_files,
            selected_row,
            focus,
            new_path_input,
            pending_changes,
            ..
        } => {
            draw_wordlist_manager_screen(
                frame,
                area,
                wordlist_files,
                *selected_row,
                focus,
                new_path_input,
                !pending_changes.is_empty(),
                colors,
            );
        }
        AppState::ThemePicker {
            selected_theme,
            custom_mode,
            custom_colors,
            custom_field,
            ..
        } => {
            draw_theme_picker_screen(
                frame,
                area,
                *selected_theme,
                *custom_mode,
                custom_colors,
                *custom_field,
                colors,
            );
        }
        AppState::SaveConfirmation { .. } => {
            // Render the settings screen in the background (dimmed)
            // Then render the confirmation modal on top
            draw_save_confirmation_modal(&area, &mut frame.buffer_mut(), colors);
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
    // Create outer block with decorated title
    let outer_block = Block::default()
        .title(DECORATED_TITLE)
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    frame.render_widget(outer_block, area);

    // Create a centered content area
    let inner_area = centered_rect(area, LOADING_WIDTH_PERCENT, LOADING_HEIGHT_PERCENT);

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

    // Get enhanced spinner frame (multiple characters for visibility)
    let enhanced_frame = ENHANCED_SPINNER_FRAMES[spinner_frame % ENHANCED_SPINNER_FRAMES.len()];
    let spinner_display = format!(
        "  {}  {}  {}  ",
        enhanced_frame, enhanced_frame, enhanced_frame
    );

    // Parse quote and attribution
    let quote_text = spinner.current_quote();
    let (quote, attribution) = parse_quote(quote_text);

    // Layout the inner area into sections
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top padding
            Constraint::Length(3), // Title box
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Spinner
            Constraint::Length(2), // Spacing
            Constraint::Min(5),    // Quote box
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Elapsed time
            Constraint::Length(1), // Bottom padding
        ])
        .split(inner_area);

    // Render decorated title "Decrypting..."
    let title_text = "  Decrypting...  ";
    let title_decoration = format!("╭{}╮", "─".repeat(title_text.len()));
    let title_bottom = format!("╰{}╯", "─".repeat(title_text.len()));

    let title_lines = vec![
        Line::from(Span::styled(&title_decoration, colors.accent)),
        Line::from(vec![
            Span::styled("│", colors.accent),
            Span::styled(title_text, colors.highlight),
            Span::styled("│", colors.accent),
        ]),
        Line::from(Span::styled(&title_bottom, colors.accent)),
    ];

    let title_paragraph = Paragraph::new(title_lines).alignment(Alignment::Center);
    frame.render_widget(title_paragraph, inner_chunks[1]);

    // Render enhanced spinner
    let spinner_line = Line::from(Span::styled(&spinner_display, colors.accent));
    let spinner_paragraph = Paragraph::new(spinner_line).alignment(Alignment::Center);
    frame.render_widget(spinner_paragraph, inner_chunks[3]);

    // Calculate quote box dimensions
    let quote_area = inner_chunks[5];

    // Create a framed quote box
    let quote_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.muted)
        .padding(Padding::horizontal(1));

    let quote_inner = quote_block.inner(quote_area);
    frame.render_widget(quote_block, quote_area);

    // Render quote content inside the box
    let mut quote_lines = vec![Line::from(Span::styled(
        format!("\"{}\"", quote),
        colors.text,
    ))];

    if !attribution.is_empty() {
        quote_lines.push(Line::from(Span::styled(
            format!("  — {}", attribution),
            colors.muted,
        )));
    }

    let quote_paragraph = Paragraph::new(quote_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    frame.render_widget(quote_paragraph, quote_inner);

    // Render elapsed time
    let elapsed_line = Line::from(Span::styled(
        format!("Elapsed: {:.1}s", elapsed_secs),
        colors.muted,
    ));
    let elapsed_paragraph = Paragraph::new(elapsed_line).alignment(Alignment::Center);
    frame.render_widget(elapsed_paragraph, inner_chunks[7]);
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
    // Calculate layout chunks - balanced split between top row and step details
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Top row (Input | Path | Output)
            Constraint::Length(1),      // Visual separator
            Constraint::Min(12),        // Step details - gets remaining space
            Constraint::Length(1),      // Status bar
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

    // Render path viewer with decorated title
    let path_block = Block::default()
        .title(" ─ Path ─ ")
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

    // Render visual separator line
    let separator_line = Line::from(Span::styled(
        "─".repeat(chunks[1].width as usize),
        colors.border,
    ));
    let separator_paragraph = Paragraph::new(separator_line);
    frame.render_widget(separator_paragraph, chunks[1]);

    // Render step details
    let current_step = result.path.get(selected_step);
    render_step_details(chunks[2], frame.buffer_mut(), current_step, colors);

    // Render status bar
    draw_status_bar(frame, chunks[3], colors);
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
    // Create outer block with decorated title
    let outer_block = Block::default()
        .title(DECORATED_TITLE)
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    frame.render_widget(outer_block, area);

    // Create inner area for content
    let inner_area = centered_rect(area, LOADING_WIDTH_PERCENT, LOADING_HEIGHT_PERCENT);

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
        ("[y]", "Yank"),
        ("[Enter]", "Rerun"),
        ("[Ctrl+S]", "Settings"),
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
    let popup_area = centered_rect(area, HELP_WIDTH_PERCENT, HELP_HEIGHT_PERCENT);

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
        ("y", "Yank (copy) selected step to clipboard"),
        ("Enter", "Rerun Ciphey from selected step"),
        ("Ctrl+S", "Open settings panel"),
        ("?", "Toggle this help overlay"),
        ("Home", "Go to first step"),
        ("End", "Go to last step"),
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

/// Renders the human confirmation modal for plaintext verification.
///
/// Displays a centered modal popup asking the user to confirm whether the
/// detected plaintext is correct. The modal appears over the loading screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The full screen area
/// * `request` - The confirmation request containing candidate text and checker info
/// * `colors` - The color scheme to use
fn draw_human_confirmation_screen(
    frame: &mut Frame,
    area: Rect,
    request: &HumanConfirmationRequest,
    colors: &TuiColors,
) {
    // Calculate modal size (65% width, 55% height for better padding)
    let modal_area = centered_rect(area, MODAL_WIDTH_PERCENT, MODAL_HEIGHT_PERCENT);

    // Clear the area behind the modal
    frame.render_widget(Clear, modal_area);

    // Create the modal block with double border
    let modal_block = Block::default()
        .title(" Confirm Plaintext? ")
        .title_style(colors.highlight)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(2, 2, 1, 1)); // Add padding inside the modal

    let inner_area = modal_block.inner(modal_area);
    frame.render_widget(modal_block, modal_area);

    // Calculate layout for the inner content
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // "Detected by:" line with icon
            Constraint::Length(1), // Spacing
            Constraint::Min(5),    // Plaintext box (increased min height)
            Constraint::Length(2), // Spacing
            Constraint::Length(1), // Instructions with styled buttons
        ])
        .split(inner_area);

    // Render "Detected by:" line with magnifying glass icon
    let detected_by_line = Line::from(vec![
        Span::styled("🔍 ", colors.accent),
        Span::styled("Detected by: ", colors.label),
        Span::styled(
            format!("{} ({})", request.checker_name, request.description),
            colors.text,
        ),
    ]);
    let detected_paragraph = Paragraph::new(detected_by_line);
    frame.render_widget(detected_paragraph, inner_chunks[0]);

    // Prepare the plaintext text (truncate if too long)
    let display_text = if request.text.len() > MAX_PLAINTEXT_PREVIEW_LEN {
        format!("{}...", &request.text[..MAX_PLAINTEXT_PREVIEW_LEN])
    } else {
        request.text.clone()
    };

    // Create the plaintext box with rounded border and padding
    let plaintext_block = Block::default()
        .title(" Candidate Plaintext ")
        .title_style(colors.muted)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border)
        .padding(Padding::horizontal(1));

    let plaintext_inner = plaintext_block.inner(inner_chunks[2]);
    frame.render_widget(plaintext_block, inner_chunks[2]);

    // Render the plaintext text inside the box
    let plaintext_paragraph =
        Paragraph::new(Span::styled(&display_text, colors.text)).wrap(Wrap { trim: false });
    frame.render_widget(plaintext_paragraph, plaintext_inner);

    // Render styled button instructions at the bottom
    let instructions = Line::from(vec![
        Span::styled("Press ", colors.muted),
        Span::styled(
            " [Y] ",
            colors
                .success
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        ),
        Span::styled(" to accept  ", colors.muted),
        Span::styled(
            " [N] ",
            colors
                .error
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        ),
        Span::styled(" to reject", colors.muted),
    ]);
    let instructions_paragraph = Paragraph::new(instructions).alignment(Alignment::Center);
    frame.render_widget(instructions_paragraph, inner_chunks[4]);
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

/// Renders the settings screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `settings` - The settings model to display
/// * `selected_section` - Index of selected section
/// * `selected_field` - Index of selected field within section
/// * `editing_mode` - Whether currently editing a field
/// * `input_buffer` - Current input buffer contents
/// * `cursor_pos` - Cursor position in input buffer
/// * `scroll_offset` - Scroll offset for long lists
/// * `validation_errors` - Map of field_id -> error message
/// * `has_changes` - Whether settings have been modified
/// * `colors` - The color scheme to use
#[allow(clippy::too_many_arguments)]
fn draw_settings_screen(
    frame: &mut Frame,
    area: Rect,
    settings: &SettingsModel,
    selected_section: usize,
    selected_field: usize,
    editing_mode: bool,
    input_buffer: &str,
    cursor_pos: usize,
    scroll_offset: usize,
    validation_errors: &std::collections::HashMap<String, String>,
    has_changes: bool,
    colors: &TuiColors,
) {
    render_settings_panel(
        area,
        frame.buffer_mut(),
        settings,
        selected_section,
        selected_field,
        editing_mode,
        input_buffer,
        cursor_pos,
        scroll_offset,
        validation_errors,
        has_changes,
        colors,
    );
}

/// Renders the list editor screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `field_label` - Name of the field being edited
/// * `items` - Current list items
/// * `selected_item` - Currently selected item index
/// * `input_buffer` - Input buffer for new items
/// * `cursor_pos` - Cursor position in input buffer
/// * `colors` - The color scheme to use
#[allow(clippy::too_many_arguments)]
fn draw_list_editor_screen(
    frame: &mut Frame,
    area: Rect,
    field_label: &str,
    items: &[String],
    selected_item: Option<usize>,
    input_buffer: &str,
    cursor_pos: usize,
    colors: &TuiColors,
) {
    render_list_editor(
        area,
        frame.buffer_mut(),
        field_label,
        items,
        selected_item,
        input_buffer,
        cursor_pos,
        colors,
    );
}

/// Renders the wordlist manager screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `wordlist_files` - List of wordlist files
/// * `selected_row` - Currently selected row
/// * `focus` - Current focus state
/// * `path_input` - Input buffer for new path
/// * `has_pending_changes` - Whether there are unsaved changes
/// * `colors` - The color scheme to use
#[allow(clippy::too_many_arguments)]
fn draw_wordlist_manager_screen(
    frame: &mut Frame,
    area: Rect,
    wordlist_files: &[super::app::WordlistFileInfo],
    selected_row: usize,
    focus: &WordlistManagerFocus,
    path_input: &str,
    has_pending_changes: bool,
    colors: &TuiColors,
) {
    // Convert app focus to widget focus
    let widget_focus = match focus {
        WordlistManagerFocus::Table => WordlistFocus::Table,
        WordlistManagerFocus::AddPathInput => WordlistFocus::AddPath,
        WordlistManagerFocus::DoneButton => WordlistFocus::Done,
    };

    render_wordlist_manager(
        area,
        frame.buffer_mut(),
        wordlist_files,
        selected_row,
        widget_focus,
        path_input,
        has_pending_changes,
        colors,
    );
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

/// Renders the theme picker screen.
fn draw_theme_picker_screen(
    frame: &mut Frame,
    area: Rect,
    selected: usize,
    custom_mode: bool,
    custom_colors: &super::widgets::theme_picker::ThemePickerCustomColors,
    custom_field: usize,
    colors: &TuiColors,
) {
    use super::widgets::theme_picker::ThemePicker;

    // Create outer block with title
    let block = Block::default()
        .title(" Choose Theme ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.accent);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Render theme picker widget
    let picker = ThemePicker::new();
    let mut buf = frame.buffer_mut();
    picker.render(
        inner,
        buf,
        selected,
        custom_mode,
        custom_colors,
        custom_field,
        colors,
    );
}

/// Renders the save confirmation modal as an overlay.
fn draw_save_confirmation_modal(area: &Rect, buf: &mut Buffer, colors: &TuiColors) {
    use ratatui::layout::Alignment;
    use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

    // Create a centered modal area
    let modal_width = 50;
    let modal_height = 7;

    let x = (area.width.saturating_sub(modal_width)) / 2;
    let y = (area.height.saturating_sub(modal_height)) / 2;

    let modal_area = Rect {
        x: area.x + x,
        y: area.y + y,
        width: modal_width.min(area.width),
        height: modal_height.min(area.height),
    };

    // Clear the modal area (makes it stand out from background)
    Clear.render(modal_area, buf);

    // Create modal block
    let block = Block::default()
        .title(" Unsaved Changes ")
        .title_style(colors.accent.add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(modal_area);
    block.render(modal_area, buf);

    // Create modal content
    let lines = vec![
        Line::from(Span::styled(
            "Do you want to save your changes?",
            colors.text.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Y]", colors.accent.add_modifier(Modifier::BOLD)),
            Span::styled("es  ", colors.text),
            Span::styled("[N]", colors.accent.add_modifier(Modifier::BOLD)),
            Span::styled("o  ", colors.text),
            Span::styled("[C]", colors.accent.add_modifier(Modifier::BOLD)),
            Span::styled("ancel", colors.text),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    paragraph.render(inner, buf);
}
