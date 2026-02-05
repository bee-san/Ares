//! Main UI rendering for the Ciphey TUI.
//!
//! This module handles rendering the terminal user interface based on the current
//! application state. It provides the main `draw` function that dispatches to
//! state-specific renderers and handles overlay rendering.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

use super::app::{
    App, AppState, BranchPath, HistoryEntry, HumanConfirmationRequest, WordlistManagerFocus,
};
use super::colors::TuiColors;
use super::multiline_text_input::MultilineTextInput;
use super::settings::SettingsModel;
use super::spinner::{Spinner, ENHANCED_SPINNER_FRAMES};
use super::widgets::{
    render_list_editor, render_settings_screen as render_settings_panel, render_step_details,
    render_toggle_list_editor, render_wordlist_manager, PathViewer, WordlistFocus,
};
use crate::storage::database::BranchSummary;

/// Modal width as percentage of screen width.
const MODAL_WIDTH_PERCENT: u16 = 65;
/// Modal height as percentage of screen height.
const MODAL_HEIGHT_PERCENT: u16 = 55;
/// Maximum plaintext preview length before truncation.
const MAX_PLAINTEXT_PREVIEW_LEN: usize = 200;
/// Help overlay width as percentage of screen.
const HELP_WIDTH_PERCENT: u16 = 55;
/// Help overlay height as percentage of screen.
const HELP_HEIGHT_PERCENT: u16 = 75;
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
/// - [`AppState::Home`]: Homescreen with text input for pasting ciphertext
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
        AppState::Home {
            text_input,
            history,
            selected_history,
            history_scroll_offset,
        } => {
            draw_home_screen(
                frame,
                area,
                text_input,
                history,
                *selected_history,
                *history_scroll_offset,
                colors,
            );
        }
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
            branch_path,
            current_branches,
            highlighted_branch,
            branch_scroll_offset,
            ..
        } => {
            draw_results_screen(
                frame,
                area,
                &app.input_text,
                result,
                *selected_step,
                branch_path,
                current_branches,
                *highlighted_branch,
                *branch_scroll_offset,
                colors,
            );
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
            text_input,
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
                text_input.get_text(),
                text_input.cursor_pos(),
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
            text_input,
            ..
        } => {
            draw_list_editor_screen(
                frame,
                area,
                field_label,
                items,
                *selected_item,
                text_input.get_text(),
                text_input.cursor_pos(),
                colors,
            );
        }
        AppState::WordlistManager {
            wordlist_files,
            selected_row,
            focus,
            text_input,
            pending_changes,
            ..
        } => {
            draw_wordlist_manager_screen(
                frame,
                area,
                wordlist_files,
                *selected_row,
                focus,
                text_input.get_text(),
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
        AppState::ToggleListEditor {
            field_label,
            all_items,
            selected_items,
            cursor_index,
            scroll_offset,
            ..
        } => {
            draw_toggle_list_editor_screen(
                frame,
                area,
                field_label,
                all_items,
                selected_items,
                *cursor_index,
                *scroll_offset,
                colors,
            );
        }
        AppState::BranchModePrompt {
            selected_mode,
            branch_context: _,
        } => {
            draw_branch_mode_prompt(frame, area, *selected_mode, colors);
        }
        AppState::DecoderSearch {
            text_input,
            all_decoders: _,
            filtered_decoders,
            selected_index,
            branch_context,
        } => {
            // First, render the underlying Results screen from the branch context
            // Load the parent result from the database to show underneath
            if let Some(parent_id) = branch_context.parent_cache_id {
                if let Ok(Some(cache_row)) = crate::storage::database::get_cache_by_id(parent_id) {
                    let crack_results: Vec<crate::decoders::crack_results::CrackResult> = cache_row
                        .path
                        .iter()
                        .filter_map(|json_str| serde_json::from_str(json_str).ok())
                        .collect();

                    let result = crate::DecoderResult {
                        text: vec![cache_row.decoded_text.clone()],
                        path: crack_results,
                    };

                    // Draw the Results screen as background
                    draw_results_screen(
                        frame,
                        area,
                        &cache_row.encoded_text,
                        &result,
                        branch_context.branch_step,
                        &BranchPath::new(),
                        &[], // No branches shown
                        None,
                        0,
                        colors,
                    );
                }
            }

            // Then overlay the decoder search modal
            draw_decoder_search(
                frame,
                area,
                text_input.get_text(),
                filtered_decoders,
                *selected_index,
                colors,
            );
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
/// * `branch_path` - Current position in branch hierarchy
/// * `current_branches` - Branches for the currently selected step
/// * `highlighted_branch` - Index of highlighted branch (if any)
/// * `branch_scroll_offset` - Scroll offset for branch list
/// * `colors` - The color scheme to use
#[allow(clippy::too_many_arguments)]
fn draw_results_screen(
    frame: &mut Frame,
    area: Rect,
    _input_text: &str,
    result: &crate::DecoderResult,
    selected_step: usize,
    branch_path: &BranchPath,
    current_branches: &[BranchSummary],
    highlighted_branch: Option<usize>,
    branch_scroll_offset: usize,
    colors: &TuiColors,
) {
    // Calculate layout chunks - full-width path panel on top, step details below
    // Branch list is integrated into the path panel
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Path panel (with optional branch list)
            Constraint::Length(1), // Visual separator
            Constraint::Min(8),    // Step details
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Render path panel with breadcrumb header
    let path_title = format!(" ─ Path ({}) ─ ", branch_path.display());
    let path_block = Block::default()
        .title(path_title)
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    let path_inner = path_block.inner(chunks[0]);
    frame.render_widget(path_block, chunks[0]);

    // Split path panel into path viewer and branch list (if branches exist)
    if current_branches.is_empty() {
        // No branches - full area for path viewer
        let path_viewer = PathViewer::new();
        path_viewer.render(
            path_inner,
            frame.buffer_mut(),
            &result.path,
            selected_step,
            colors,
        );
    } else {
        // Split: 55% path viewer, 45% branch list
        let path_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(55), // Path viewer
                Constraint::Min(4),         // Branch list
            ])
            .split(path_inner);

        // Render path viewer with branch count indicator
        let path_viewer = PathViewer::new();
        path_viewer.render_with_branch_count(
            path_chunks[0],
            frame.buffer_mut(),
            &result.path,
            selected_step,
            current_branches.len(),
            colors,
        );

        // Render branch list section
        render_branch_list(
            path_chunks[1],
            frame.buffer_mut(),
            current_branches,
            highlighted_branch,
            branch_scroll_offset,
            selected_step,
            colors,
        );
    }

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

/// Renders the branch list section below the path viewer.
///
/// Shows a header line with branch count, then a scrollable list of branches.
/// Each branch shows: decoder name, final text preview, and indicators for
/// success (checkmark) and sub-branches count.
///
/// # Arguments
///
/// * `area` - The area to render the branch list
/// * `buf` - The buffer to render into
/// * `branches` - List of branches to display
/// * `highlighted` - Index of the currently highlighted branch (if any)
/// * `scroll_offset` - Number of branches scrolled past
/// * `parent_step` - The step index these branches originate from
/// * `colors` - The color scheme to use
fn render_branch_list(
    area: Rect,
    buf: &mut Buffer,
    branches: &[BranchSummary],
    highlighted: Option<usize>,
    scroll_offset: usize,
    parent_step: usize,
    colors: &TuiColors,
) {
    if area.height < 2 || area.width < 10 {
        return;
    }

    // Get decoder name for the parent step (for header)
    let header_text = format!(
        "─── Branches from step {} ({} total) ───",
        parent_step,
        branches.len()
    );

    // Render header
    let header_style = colors.text_dimmed;
    let header_line = Line::from(Span::styled(&header_text, header_style));
    let header_para = Paragraph::new(header_line).alignment(Alignment::Center);

    let header_area = Rect::new(area.x, area.y, area.width, 1);
    header_para.render(header_area, buf);

    // Calculate visible branches
    let list_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(1),
    );
    let visible_count = list_area.height as usize;

    // Calculate scroll indicators
    let branches_above = scroll_offset;
    let branches_below = branches.len().saturating_sub(scroll_offset + visible_count);

    // Render each visible branch
    for (display_idx, branch_idx) in (scroll_offset..)
        .take(visible_count)
        .enumerate()
        .filter(|(_, idx)| *idx < branches.len())
    {
        let branch = &branches[branch_idx];
        let is_highlighted = highlighted == Some(branch_idx);

        let y = list_area.y + display_idx as u16;
        if y >= list_area.y + list_area.height {
            break;
        }

        render_branch_row(
            Rect::new(list_area.x, y, list_area.width, 1),
            buf,
            branch,
            is_highlighted,
            colors,
        );
    }

    // Render scroll indicator in the right margin if needed
    if branches_above > 0 || branches_below > 0 {
        let indicator = format!("[^{} v{}]", branches_above, branches_below);
        let indicator_width = indicator.chars().count() as u16;
        let indicator_x = area.x + area.width.saturating_sub(indicator_width + 1);
        buf.set_string(indicator_x, area.y, &indicator, colors.text_dimmed);
    }
}

/// Renders a single branch row.
///
/// Format: "> [Decoder] --> \"preview...\" ✓ (N sub)"
fn render_branch_row(
    area: Rect,
    buf: &mut Buffer,
    branch: &BranchSummary,
    is_highlighted: bool,
    colors: &TuiColors,
) {
    if area.width < 5 {
        return;
    }

    // Build the row content
    let prefix = if is_highlighted { " > " } else { "   " };
    let success_indicator = if branch.successful { " ✓" } else { "" };
    let sub_count = if branch.sub_branch_count > 0 {
        format!(" ({} sub)", branch.sub_branch_count)
    } else {
        String::new()
    };

    // Truncate preview to fit
    let fixed_parts_len =
        prefix.len() + branch.first_decoder.len() + 8 + success_indicator.len() + sub_count.len();
    let available_preview = (area.width as usize).saturating_sub(fixed_parts_len);
    let preview = if branch.final_text_preview.len() > available_preview {
        format!(
            "{}...",
            branch
                .final_text_preview
                .chars()
                .take(available_preview.saturating_sub(3))
                .collect::<String>()
        )
    } else {
        branch.final_text_preview.clone()
    };

    // Choose style based on highlight state
    let style = if is_highlighted {
        colors
            .accent
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
    } else {
        colors.text
    };

    let success_style = if is_highlighted {
        style
    } else {
        colors.success
    };

    // Build spans
    let mut spans = vec![
        Span::styled(prefix, style),
        Span::styled(format!("[{}]", branch.first_decoder), style),
        Span::styled(" --> ", colors.text_dimmed),
        Span::styled(format!("\"{}\"", preview), style),
    ];

    if branch.successful {
        spans.push(Span::styled(success_indicator, success_style));
    }

    if !sub_count.is_empty() {
        spans.push(Span::styled(sub_count, colors.text_dimmed));
    }

    let line = Line::from(spans);
    let para = Paragraph::new(line);
    para.render(area, buf);
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

    // Truncate input if too long (UTF-8 safe)
    let display_input = if input_text.chars().count() > 50 {
        format!("{}...", input_text.chars().take(50).collect::<String>())
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
        Line::from(Span::styled(
            "Press 'b' for home  |  'q' to exit",
            colors.muted,
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner_area);
}

/// Renders the home screen with history panel and text input for pasting ciphertext.
///
/// Displays a 30/70 split layout with:
/// - Left (30%): History panel showing previous decode attempts
/// - Right (70%): Main content with welcome message and text input
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `text_input` - The multi-line text input component
/// * `history` - The list of previous decode attempts
/// * `selected_history` - Currently selected history entry index (None = input focused)
/// * `history_scroll_offset` - Scroll offset for the history panel
/// * `colors` - The color scheme to use
fn draw_home_screen(
    frame: &mut Frame,
    area: Rect,
    text_input: &MultilineTextInput,
    history: &[HistoryEntry],
    selected_history: Option<usize>,
    history_scroll_offset: usize,
    colors: &TuiColors,
) {
    // Create outer block with decorated title
    let outer_block = Block::default()
        .title(DECORATED_TITLE)
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.border);

    frame.render_widget(outer_block, area);

    // Create inner content area
    let inner_area = Rect {
        x: area.x + 2,
        y: area.y + 2,
        width: area.width.saturating_sub(4),
        height: area.height.saturating_sub(4),
    };

    // 30/70 horizontal split
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // History panel
            Constraint::Percentage(70), // Main content
        ])
        .split(inner_area);

    // Draw history panel (left side)
    draw_history_panel(
        frame,
        main_chunks[0],
        history,
        selected_history,
        history_scroll_offset,
        colors,
    );

    // Draw main content (right side)
    draw_main_input_area(
        frame,
        main_chunks[1],
        text_input,
        selected_history.is_none(),
        colors,
    );
}

/// Renders the history panel showing previous decode attempts.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `history` - The list of previous decode attempts
/// * `selected_history` - Currently selected history entry index
/// * `scroll_offset` - Scroll offset for the list
/// * `colors` - The color scheme to use
fn draw_history_panel(
    frame: &mut Frame,
    area: Rect,
    history: &[HistoryEntry],
    selected_history: Option<usize>,
    scroll_offset: usize,
    colors: &TuiColors,
) {
    let is_focused = selected_history.is_some();

    // Create the history block with appropriate styling based on focus
    let history_block = Block::default()
        .title(" History ")
        .title_style(if is_focused {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.label
        })
        .borders(Borders::ALL)
        .border_type(if is_focused {
            BorderType::Double
        } else {
            BorderType::Rounded
        })
        .border_style(if is_focused {
            colors.accent
        } else {
            colors.border
        })
        .padding(Padding::horizontal(1));

    let history_inner = history_block.inner(area);
    frame.render_widget(history_block, area);

    if history.is_empty() {
        // Show placeholder when no history
        let placeholder = Paragraph::new(Line::from(Span::styled("No history yet", colors.muted)))
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, history_inner);
        return;
    }

    // Calculate visible lines
    let visible_lines = history_inner.height as usize;

    // Auto-scroll to keep selected item visible
    let effective_scroll = if let Some(idx) = selected_history {
        if idx >= scroll_offset + visible_lines {
            idx.saturating_sub(visible_lines - 1)
        } else if idx < scroll_offset {
            idx
        } else {
            scroll_offset
        }
    } else {
        scroll_offset
    };

    // Build history lines
    let lines: Vec<Line> = history
        .iter()
        .enumerate()
        .skip(effective_scroll)
        .take(visible_lines)
        .map(|(idx, entry)| {
            let is_selected = selected_history == Some(idx);

            // Status emoji
            let status = if entry.successful { "✓ " } else { "✗ " };
            let status_style = if entry.successful {
                colors.success
            } else {
                colors.error
            };

            // Format the relative time
            let time_str = format_relative_time(&entry.timestamp);

            // Build the line
            let mut spans = vec![
                Span::styled(status, status_style),
                Span::styled(
                    entry.encoded_text_preview.clone(),
                    if is_selected {
                        colors
                            .accent
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::REVERSED)
                    } else {
                        colors.text
                    },
                ),
            ];

            // Add time on a new conceptual "line" but we'll truncate to fit
            // For simplicity, append time with dimmed style
            // Note: Use chars().count() for proper Unicode width calculation
            let status_width = 2u16; // "✓ " or "✗ " is 2 display cells
            let preview_width = entry.encoded_text_preview.chars().count() as u16;
            let remaining_width = area.width.saturating_sub(4 + status_width + preview_width);
            if remaining_width > 6 {
                spans.push(Span::styled(" ", colors.text));
                spans.push(Span::styled(
                    time_str
                        .chars()
                        .take(remaining_width as usize - 1)
                        .collect::<String>(),
                    colors.muted,
                ));
            }

            Line::from(spans)
        })
        .collect();

    let history_paragraph = Paragraph::new(lines);
    frame.render_widget(history_paragraph, history_inner);

    // Show scroll indicator if needed
    if history.len() > visible_lines {
        let scroll_info = format!("{}/{}", effective_scroll + 1, history.len());
        let scroll_indicator = Paragraph::new(Line::from(Span::styled(scroll_info, colors.muted)))
            .alignment(Alignment::Right);
        // Render at bottom of area
        let indicator_area = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };
        frame.render_widget(scroll_indicator, indicator_area);
    }
}

/// Formats a timestamp into a relative time string.
///
/// # Arguments
///
/// * `timestamp` - The timestamp string in "YYYY-MM-DD HH:MM:SS" format
///
/// # Returns
///
/// A human-readable relative time string like "2m ago", "1h ago", "Yesterday"
fn format_relative_time(timestamp: &str) -> String {
    // Parse the timestamp (format: "YYYY-MM-DD HH:MM:SS")
    let parsed = chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S");

    match parsed {
        Ok(dt) => {
            let now = chrono::Local::now().naive_local();
            let duration = now.signed_duration_since(dt);

            if duration.num_seconds() < 60 {
                "just now".to_string()
            } else if duration.num_minutes() < 60 {
                format!("{}m ago", duration.num_minutes())
            } else if duration.num_hours() < 24 {
                format!("{}h ago", duration.num_hours())
            } else if duration.num_days() == 1 {
                "Yesterday".to_string()
            } else if duration.num_days() < 7 {
                format!("{}d ago", duration.num_days())
            } else {
                // Show date for older entries
                dt.format("%b %d").to_string()
            }
        }
        Err(_) => timestamp.to_string(), // Fallback to raw timestamp
    }
}

/// Renders the main input area with welcome message and text input.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `text_input` - The multi-line text input component
/// * `is_focused` - Whether the input area is currently focused
/// * `colors` - The color scheme to use
fn draw_main_input_area(
    frame: &mut Frame,
    area: Rect,
    text_input: &MultilineTextInput,
    is_focused: bool,
    colors: &TuiColors,
) {
    // Layout the main area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Welcome message and instructions
            Constraint::Length(1), // Spacing
            Constraint::Min(8),    // Text input area
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Status bar with keybindings
        ])
        .split(area);

    // Render welcome message and instructions
    let welcome_lines = vec![
        Line::from(Span::styled(
            "Welcome to Ciphey",
            colors.highlight.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Paste your ciphertext below and press Enter to decode",
            colors.text,
        )),
        Line::from(Span::styled(
            "(Use Ctrl+Enter to insert newlines for multi-line input)",
            colors.muted,
        )),
    ];

    let welcome_paragraph = Paragraph::new(welcome_lines).alignment(Alignment::Center);
    frame.render_widget(welcome_paragraph, chunks[0]);

    // Create the text input box with focus styling
    let input_block = Block::default()
        .title(" Ciphertext ")
        .title_style(if is_focused {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.label
        })
        .borders(Borders::ALL)
        .border_type(if is_focused {
            BorderType::Double
        } else {
            BorderType::Rounded
        })
        .border_style(if is_focused {
            colors.accent
        } else {
            colors.border
        })
        .padding(Padding::horizontal(1));

    let input_inner = input_block.inner(chunks[2]);
    frame.render_widget(input_block, chunks[2]);

    // Render the text input content with cursor
    let (cursor_line, cursor_col) = text_input.cursor_pos();
    let scroll_offset = text_input.scroll_offset();
    let visible_lines = input_inner.height as usize;

    let lines: Vec<Line> = text_input
        .lines()
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_lines)
        .map(|(line_idx, line_text)| {
            let is_cursor_line = line_idx == cursor_line;

            if is_cursor_line && line_idx >= scroll_offset && is_focused {
                // Build line with cursor (only show cursor when focused)
                let display_line_idx = line_idx - scroll_offset;
                if display_line_idx < visible_lines {
                    // Insert cursor character at the right position
                    let chars: Vec<char> = line_text.chars().collect();
                    let before: String = chars.iter().take(cursor_col).collect();
                    let after: String = chars.iter().skip(cursor_col).collect();

                    Line::from(vec![
                        Span::styled(before, colors.text),
                        Span::styled("█", colors.accent.add_modifier(Modifier::SLOW_BLINK)),
                        Span::styled(after, colors.text),
                    ])
                } else {
                    Line::from(Span::styled(line_text.clone(), colors.text))
                }
            } else {
                Line::from(Span::styled(line_text.clone(), colors.text))
            }
        })
        .collect();

    // If input is empty, show placeholder
    let display_lines = if text_input.is_empty() {
        if is_focused {
            vec![Line::from(vec![
                Span::styled("█", colors.accent.add_modifier(Modifier::SLOW_BLINK)),
                Span::styled(" Type or paste ciphertext here...", colors.muted),
            ])]
        } else {
            vec![Line::from(Span::styled(
                "Type or paste ciphertext here...",
                colors.muted,
            ))]
        }
    } else {
        lines
    };

    let input_paragraph = Paragraph::new(display_lines).wrap(Wrap { trim: false });
    frame.render_widget(input_paragraph, input_inner);

    // Render status bar with keybindings
    let keybindings = [
        ("[Tab]", "Switch"),
        ("[Enter]", "Decode"),
        ("[Ctrl+S]", "Settings"),
        ("[Esc]", "Quit"),
    ];

    let mut spans = Vec::new();
    for (i, (key, desc)) in keybindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", colors.text));
        }
        spans.push(Span::styled(*key, colors.accent));
        spans.push(Span::styled(format!(" {}", desc), colors.muted));
    }

    let status = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(status, chunks[4]);
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
        ("[h/l]", "Step"),
        ("[j/k]", "Branch"),
        ("[/]", "Search"),
        ("[y]", "Yank"),
        ("[b]", "Home"),
        ("[Enter]", "Select"),
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

    // Build help content - Results screen keybindings
    let keybindings = vec![
        ("Navigation", ""),
        ("← / h", "Select previous step"),
        ("→ / l", "Select next step"),
        ("↑ / k", "Select previous branch"),
        ("↓ / j", "Select next branch"),
        ("gg", "Go to first step"),
        ("G / End", "Go to last step"),
        ("Home", "Go to first step"),
        ("", ""),
        ("Actions", ""),
        ("y / c", "Yank (copy) output to clipboard"),
        ("Enter", "Select branch or create new branch"),
        ("Backspace", "Return to parent branch"),
        ("/", "Search and run specific decoder"),
        ("b", "Return to home screen"),
        ("", ""),
        ("General", ""),
        ("Ctrl+S", "Open settings panel"),
        ("?", "Toggle this help overlay"),
        ("q / Esc", "Quit the application"),
    ];

    let mut lines = vec![
        Line::from(Span::styled(
            "Keybindings",
            colors.highlight.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (key, description) in keybindings {
        if key.is_empty() && description.is_empty() {
            // Empty line separator
            lines.push(Line::from(""));
        } else if description.is_empty() {
            // Section header
            lines.push(Line::from(Span::styled(
                key,
                colors.label.add_modifier(Modifier::BOLD),
            )));
        } else {
            // Regular keybinding
            lines.push(Line::from(vec![
                Span::styled(format!("{:12}", key), colors.accent),
                Span::styled(description, colors.text),
            ]));
        }
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

/// Renders the toggle list editor screen.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `field_label` - Name of the field being edited
/// * `all_items` - All available items
/// * `selected_items` - Currently selected/enabled items
/// * `cursor_index` - Currently highlighted item
/// * `scroll_offset` - Scroll offset for long lists
/// * `colors` - The color scheme to use
#[allow(clippy::too_many_arguments)]
fn draw_toggle_list_editor_screen(
    frame: &mut Frame,
    area: Rect,
    field_label: &str,
    all_items: &[String],
    selected_items: &[String],
    cursor_index: usize,
    scroll_offset: usize,
    colors: &TuiColors,
) {
    render_toggle_list_editor(
        area,
        frame.buffer_mut(),
        field_label,
        all_items,
        selected_items,
        cursor_index,
        scroll_offset,
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
    let buf = frame.buffer_mut();
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

/// Renders the branch mode prompt modal.
///
/// Displays a centered modal for choosing between full A* search and single-layer decoding.
fn draw_branch_mode_prompt(
    frame: &mut Frame,
    area: Rect,
    selected_mode: super::app::BranchMode,
    colors: &TuiColors,
) {
    use super::app::BranchMode;

    let modal_width: u16 = 50;
    let modal_height: u16 = 12;

    let x = (area.width.saturating_sub(modal_width)) / 2;
    let y = (area.height.saturating_sub(modal_height)) / 2;

    let modal_area = Rect {
        x: area.x + x,
        y: area.y + y,
        width: modal_width.min(area.width),
        height: modal_height.min(area.height),
    };

    // Clear the modal area
    frame.render_widget(Clear, modal_area);

    // Create modal block
    let block = Block::default()
        .title(" How do you want to branch? ")
        .title_style(colors.accent.add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    let is_full_search = selected_mode == BranchMode::FullSearch;

    let full_search_style = if is_full_search {
        colors
            .accent
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
    } else {
        colors.text
    };

    let single_layer_style = if !is_full_search {
        colors
            .accent
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
    } else {
        colors.text
    };

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            if is_full_search {
                "> Full A* Search"
            } else {
                "  Full A* Search"
            },
            full_search_style,
        )),
        Line::from(Span::styled(
            "    Run complete search to find plaintext",
            colors.muted,
        )),
        Line::from(""),
        Line::from(Span::styled(
            if !is_full_search {
                "> Single Layer"
            } else {
                "  Single Layer"
            },
            single_layer_style,
        )),
        Line::from(Span::styled(
            "    Run all decoders once and show results",
            colors.muted,
        )),
        Line::from(""),
        Line::from(Span::styled("[Enter] Select  [Esc] Cancel", colors.muted)),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Renders the decoder search modal (vim-style).
///
/// Displays a search input at the bottom-left of the screen with filtered decoder list.
fn draw_decoder_search(
    frame: &mut Frame,
    area: Rect,
    search_text: &str,
    filtered_decoders: &[&str],
    selected_index: usize,
    colors: &TuiColors,
) {
    let modal_width: u16 = 35;
    let modal_height: u16 = 12.min(filtered_decoders.len() as u16 + 4);

    // Position at bottom-left
    let modal_area = Rect {
        x: area.x + 2,
        y: area.y + area.height.saturating_sub(modal_height + 2),
        width: modal_width.min(area.width.saturating_sub(4)),
        height: modal_height.min(area.height.saturating_sub(4)),
    };

    // Clear the modal area
    frame.render_widget(Clear, modal_area);

    // Create modal block with search prompt
    let title = format!(" /{} ", search_text);
    let block = Block::default()
        .title(title)
        .title_style(colors.accent)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border);

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    // Render filtered decoder list
    let visible_count = inner.height.saturating_sub(1) as usize;
    let start = if selected_index >= visible_count {
        selected_index - visible_count + 1
    } else {
        0
    };

    let lines: Vec<Line> = filtered_decoders
        .iter()
        .enumerate()
        .skip(start)
        .take(visible_count)
        .map(|(idx, name)| {
            let is_selected = idx == selected_index;
            let style = if is_selected {
                colors
                    .accent
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                colors.text
            };
            Line::from(Span::styled(
                if is_selected {
                    format!("> {}", name)
                } else {
                    format!("  {}", name)
                },
                style,
            ))
        })
        .collect();

    let mut all_lines = lines;
    all_lines.push(Line::from(Span::styled(
        "[Enter] Run  [Esc] Cancel",
        colors.muted,
    )));

    let paragraph = Paragraph::new(all_lines);
    frame.render_widget(paragraph, inner);
}
