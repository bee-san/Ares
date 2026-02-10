//! Main UI rendering for the Ciphey TUI.
//!
//! This module handles rendering the terminal user interface based on the current
//! application state. It provides the main `draw` function that dispatches to
//! state-specific renderers and handles overlay rendering.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};

use super::app::{
    App, AppState, HistoryEntry, HumanConfirmationRequest, StatusSeverity, WordlistManagerFocus,
};
use super::colors::TuiColors;
use super::multiline_text_input::MultilineTextInput;
use super::settings::SettingsModel;
use super::spinner::{ENHANCED_SPINNER_FRAMES, QUOTES};
use super::widgets::{
    render_ask_ai_modal, render_list_editor, render_settings_screen as render_settings_panel,
    render_step_details, render_toggle_list_editor, render_wordlist_manager, TreeViewer,
    WordlistFocus,
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
/// - [`AppState::Home`]: Two-panel homescreen (30% history, 70% input)
/// - [`AppState::Loading`]: Centered spinner with rotating quotes
/// - [`AppState::Results`]: Path viewer with step details (full-width layout)
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
        AppState::Home(home) => {
            draw_home_screen(
                frame,
                area,
                &home.text_input,
                &home.history,
                home.selected_history,
                home.history_scroll_offset,
                colors,
            );
        }
        AppState::Loading(ls) => {
            let tried = ls.decoders_tried.load(std::sync::atomic::Ordering::Relaxed);
            draw_loading_screen(
                frame,
                area,
                ls.spinner_frame,
                ls.current_quote,
                &ls.start_time,
                tried,
                colors,
            );
        }
        AppState::HumanConfirmation(hc) => {
            // Draw loading screen in background (no progress counter available)
            draw_loading_screen(
                frame,
                area,
                hc.spinner_frame,
                hc.current_quote,
                &hc.start_time,
                0,
                colors,
            );
            // Draw confirmation modal on top
            draw_human_confirmation_screen(frame, area, &hc.request, colors);
        }
        AppState::Results(rs) => {
            draw_results_screen(frame, area, &app.input_text, rs, colors);
        }
        AppState::Failure(fs) => {
            draw_failure_screen(frame, area, &fs.input_text, fs.elapsed, colors);
        }
        AppState::Settings(ss) => {
            draw_settings_screen(
                frame,
                area,
                &ss.settings,
                ss.selected_section,
                ss.selected_field,
                ss.editing_mode,
                ss.text_input.get_text(),
                ss.text_input.cursor_pos(),
                ss.scroll_offset,
                &ss.validation_errors,
                ss.settings.has_changes(),
                colors,
            );
        }
        AppState::ListEditor(le) => {
            draw_list_editor_screen(
                frame,
                area,
                &le.field_label,
                &le.items,
                le.selected_item,
                le.text_input.get_text(),
                le.text_input.cursor_pos(),
                colors,
            );
        }
        AppState::WordlistManager(wm) => {
            draw_wordlist_manager_screen(
                frame,
                area,
                &wm.wordlist_files,
                wm.selected_row,
                &wm.focus,
                wm.text_input.get_text(),
                !wm.pending_changes.is_empty(),
                colors,
            );
        }
        AppState::ThemePicker(tp) => {
            draw_theme_picker_screen(
                frame,
                area,
                tp.selected_theme,
                tp.custom_mode,
                &tp.custom_colors,
                tp.custom_field,
                colors,
            );
        }
        AppState::SaveConfirmation(sc) => {
            // Render the settings screen in the background first
            draw_settings_screen(
                frame,
                area,
                &sc.parent_settings.settings,
                sc.parent_settings.selected_section,
                sc.parent_settings.selected_field,
                false, // not editing
                "",    // empty input buffer
                0,     // cursor at 0
                sc.parent_settings.scroll_offset,
                &sc.parent_settings.validation_errors,
                sc.parent_settings.settings.has_changes(),
                colors,
            );
            // Then render the confirmation modal on top
            draw_save_confirmation_modal(&area, &mut frame.buffer_mut(), colors);
        }
        AppState::ToggleListEditor(tle) => {
            draw_toggle_list_editor_screen(
                frame,
                area,
                &tle.field_label,
                &tle.all_items,
                &tle.selected_items,
                tle.cursor_index,
                tle.scroll_offset,
                colors,
            );
        }
        AppState::BranchModePrompt(bmp) => {
            draw_branch_mode_prompt(frame, area, bmp.selected_mode, colors);
        }
    }

    // Render decoder search overlay if active (floats on top of Results screen)
    if let Some(ref overlay) = app.decoder_search {
        draw_decoder_search(
            frame,
            area,
            overlay.text_input.get_text(),
            &overlay.filtered_decoders,
            overlay.selected_index,
            colors,
        );
    }

    // Render quick search overlay if active (floats on top of Results screen)
    if let Some(ref overlay) = app.quick_search {
        draw_quick_search(
            frame,
            area,
            &overlay.entries,
            overlay.selected_index,
            colors,
        );
    }

    // Render Ask AI overlay if active (floats on top of Results screen)
    if let Some(ref overlay) = app.ask_ai {
        render_ask_ai_modal(frame, area, overlay, colors);
    }

    // Render help overlay if visible
    if app.show_help {
        draw_help_overlay(frame, area, app.help_context(), colors);
    }

    // Render status message if present
    if let Some(ref msg) = app.status_message {
        draw_status_message(frame, area, &msg.text, msg.severity, colors);
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
    decoders_tried: usize,
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

    // Get quote directly by index (quote_index is randomized at state creation
    // and advanced by app.tick(), so no need to create a Spinner each frame)
    let quote_text = QUOTES[quote_index % QUOTES.len()];

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

    // Render elapsed time and progress counter
    let progress_text = if decoders_tried > 0 {
        format!(
            "Elapsed: {:.1}s  •  Tried {} decoder combinations",
            elapsed_secs, decoders_tried
        )
    } else {
        format!("Elapsed: {:.1}s", elapsed_secs)
    };
    let elapsed_line = Line::from(Span::styled(progress_text, colors.muted));
    let elapsed_paragraph = Paragraph::new(elapsed_line).alignment(Alignment::Center);
    frame.render_widget(elapsed_paragraph, inner_chunks[7]);
}

/// Renders the results screen with horizontal split layout.
///
/// Layout:
/// - Left (~38%): Step details panel
/// - Right (~62%): Two vertically stacked panels
///   - Top: Birds-eye tree view
///   - Bottom: Level detail (scrollable branch list)
/// - Bottom (full width): Status bar with keybindings
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `input_text` - The original input text
/// * `rs` - The results state data
/// * `colors` - The color scheme to use
fn draw_results_screen(
    frame: &mut Frame,
    area: Rect,
    _input_text: &str,
    rs: &super::app::ResultsState,
    colors: &TuiColors,
) {
    use super::app::ResultsFocus;

    // Outer split: main area + status bar
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Main area
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Main area: left (step details) + right (tree + level)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(38), // Step details
            Constraint::Percentage(62), // Tree view + Level detail
        ])
        .split(outer_chunks[0]);

    // Right side: tree view (top) + level detail (bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // Tree view
            Constraint::Percentage(45), // Level detail
        ])
        .split(main_chunks[1]);

    // ── Left Panel: Step Details ──
    let step_is_focused = rs.focus == ResultsFocus::StepDetails;
    let step_details_block = Block::default()
        .title(" Step Details ")
        .title_style(if step_is_focused {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.title
        })
        .borders(Borders::ALL)
        .border_type(if step_is_focused {
            BorderType::Double
        } else {
            BorderType::Plain
        })
        .border_style(if step_is_focused {
            colors.accent
        } else {
            colors.border
        });

    let step_details_inner = step_details_block.inner(main_chunks[0]);
    frame.render_widget(step_details_block, main_chunks[0]);

    let current_step = rs.result.path.get(rs.selected_step);
    render_step_details(
        step_details_inner,
        frame.buffer_mut(),
        current_step,
        colors,
        rs.ai_explanation.as_deref(),
        rs.ai_loading,
    );

    // ── Right Top Panel: Birds-Eye Tree View ──
    let tree_is_focused = rs.focus == ResultsFocus::TreeView;
    let tree_title = format!(" Tree ({}) ", rs.branch_path.display());
    let tree_block = Block::default()
        .title(tree_title)
        .title_style(if tree_is_focused {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.title
        })
        .borders(Borders::ALL)
        .border_type(if tree_is_focused {
            BorderType::Double
        } else {
            BorderType::Plain
        })
        .border_style(if tree_is_focused {
            colors.accent
        } else {
            colors.border
        });

    let tree_inner = tree_block.inner(right_chunks[0]);
    frame.render_widget(tree_block, right_chunks[0]);

    let tree_viewer = TreeViewer::new();
    tree_viewer.render(
        tree_inner,
        frame.buffer_mut(),
        &rs.result.path,
        rs.selected_step,
        &rs.tree_branches,
        colors,
    );

    // ── Right Bottom Panel: Level Detail (Branch List) ──
    let level_is_focused = rs.focus == ResultsFocus::LevelDetail;
    let level_title = if rs.current_branches.is_empty() {
        " Branches ".to_string()
    } else {
        format!(
            " Branches from step {} ({} total) ",
            rs.selected_step,
            rs.current_branches.len()
        )
    };
    let level_block = Block::default()
        .title(level_title)
        .title_style(if level_is_focused {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.title
        })
        .borders(Borders::ALL)
        .border_type(if level_is_focused {
            BorderType::Double
        } else {
            BorderType::Plain
        })
        .border_style(if level_is_focused {
            colors.accent
        } else {
            colors.border
        });

    let level_inner = level_block.inner(right_chunks[1]);
    frame.render_widget(level_block, right_chunks[1]);

    if rs.current_branches.is_empty() {
        // Show placeholder when no branches
        let placeholder = Paragraph::new(Line::from(Span::styled(
            "No branches at this step. Press [Enter] to create one, or [/] to search decoders.",
            colors.muted,
        )))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, level_inner);
    } else {
        // Render branch list
        render_branch_list(
            level_inner,
            frame.buffer_mut(),
            &rs.current_branches,
            rs.highlighted_branch,
            rs.branch_scroll_offset,
            rs.selected_step,
            colors,
        );
    }

    // ── Status Bar ──
    draw_status_bar(frame, outer_chunks[1], rs.focus, colors);
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
    let header_style = colors.muted;
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
        buf.set_string(indicator_x, area.y, &indicator, colors.muted);
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
        Span::styled(" --> ", colors.muted),
        Span::styled(format!("\"{}\"", preview), style),
    ];

    if branch.successful {
        spans.push(Span::styled(success_indicator, success_style));
    }

    if !sub_count.is_empty() {
        spans.push(Span::styled(sub_count, colors.muted));
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
            let status_width = 2u16;
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
fn format_relative_time(timestamp: &str) -> String {
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
                dt.format("%b %d").to_string()
            }
        }
        Err(_) => timestamp.to_string(),
    }
}

/// Renders the main input area with welcome message and text input.
fn draw_main_input_area(
    frame: &mut Frame,
    area: Rect,
    text_input: &MultilineTextInput,
    is_focused: bool,
    colors: &TuiColors,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(8),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

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
                let display_line_idx = line_idx - scroll_offset;
                if display_line_idx < visible_lines {
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

    let display_lines = if text_input.is_empty() {
        if is_focused {
            vec![Line::from(vec![
                Span::styled("█", colors.accent.add_modifier(Modifier::SLOW_BLINK)),
                Span::styled(" Paste or type ciphertext here...", colors.muted),
            ])]
        } else {
            vec![Line::from(Span::styled(
                "Paste or type ciphertext here...",
                colors.muted,
            ))]
        }
    } else {
        lines
    };

    let input_paragraph = Paragraph::new(display_lines).wrap(Wrap { trim: false });
    frame.render_widget(input_paragraph, input_inner);

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
fn draw_status_bar(
    frame: &mut Frame,
    area: Rect,
    focus: super::app::ResultsFocus,
    colors: &TuiColors,
) {
    use super::app::ResultsFocus;

    let focus_label = match focus {
        ResultsFocus::TreeView => "Tree",
        ResultsFocus::LevelDetail => "Level",
        ResultsFocus::StepDetails => "Step",
    };

    let keybindings: &[(&str, &str)] = match focus {
        ResultsFocus::TreeView => &[
            ("[h/l]", "Step"),
            ("[gg/G]", "First/Last"),
            ("[e]", "Explain"),
            ("[a]", "Ask AI"),
            ("[Tab]", "Focus"),
            ("[Enter]", "Branch"),
            ("[/]", "Search"),
            ("[o]", "Open"),
            ("[y]", "Yank"),
            ("[b]", "Home"),
            ("[?]", "Help"),
        ],
        ResultsFocus::LevelDetail => &[
            ("[j/k]", "Browse"),
            ("[Enter]", "Select"),
            ("[e]", "Explain"),
            ("[a]", "Ask AI"),
            ("[Tab]", "Focus"),
            ("[/]", "Search"),
            ("[o]", "Open"),
            ("[y]", "Yank"),
            ("[b]", "Home"),
            ("[?]", "Help"),
        ],
        ResultsFocus::StepDetails => &[
            ("[e]", "Explain"),
            ("[a]", "Ask AI"),
            ("[Tab]", "Focus"),
            ("[y]", "Yank"),
            ("[/]", "Search"),
            ("[o]", "Open"),
            ("[b]", "Home"),
            ("[?]", "Help"),
        ],
    };

    let mut spans = vec![Span::styled(
        format!("[{}] ", focus_label),
        colors.accent.add_modifier(Modifier::BOLD),
    )];
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
fn draw_help_overlay(
    frame: &mut Frame,
    area: Rect,
    context: super::app::HelpContext,
    colors: &TuiColors,
) {
    use super::app::HelpContext;

    let popup_area = centered_rect(area, HELP_WIDTH_PERCENT, HELP_HEIGHT_PERCENT);
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Help ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_style(colors.accent);

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let keybindings: Vec<(&str, &str)> = match context {
        HelpContext::Home => vec![
            ("Navigation", ""),
            ("Tab", "Switch between history and input"),
            ("↑ / k", "Navigate history up"),
            ("↓ / j", "Navigate history down"),
            ("← / →", "Move cursor / switch panels"),
            ("Ctrl+← / →", "Move cursor by word"),
            ("", ""),
            ("Actions", ""),
            ("Enter", "Submit input / Select history entry"),
            ("Ctrl+Enter", "Insert newline in input"),
            ("Ctrl+S", "Open settings panel"),
            ("Esc", "Quit / Deselect history"),
        ],
        HelpContext::Results => vec![
            ("Navigation", ""),
            ("← / h", "Select previous step"),
            ("→ / l", "Select next step"),
            ("↑ / k", "Select previous branch"),
            ("↓ / j", "Select next branch"),
            ("gg / Home", "Go to first step"),
            ("G / End", "Go to last step"),
            ("", ""),
            ("Actions", ""),
            ("y / c", "Yank (copy) output to clipboard"),
            ("e", "Explain step with AI"),
            ("a", "Ask AI a question about this step"),
            ("o", "Open output in browser"),
            ("Enter", "Select branch or create new branch"),
            ("Backspace", "Return to parent branch"),
            ("/", "Search and run specific decoder"),
            ("b", "Return to home screen"),
            ("", ""),
            ("General", ""),
            ("Ctrl+S", "Open settings panel"),
            ("?", "Toggle this help overlay"),
            ("q / Esc", "Quit the application"),
        ],
        HelpContext::Settings => vec![
            ("Navigation", ""),
            ("Tab / Shift+Tab", "Cycle through sections"),
            ("↑ / k", "Previous field"),
            ("↓ / j", "Next field"),
            ("← / h", "Previous section"),
            ("→ / l", "Next section"),
            ("", ""),
            ("Actions", ""),
            ("Enter", "Edit selected field"),
            ("Space", "Toggle boolean field"),
            ("Ctrl+S", "Save settings and close"),
            ("Esc", "Close settings / Cancel edit"),
        ],
        HelpContext::Loading => vec![
            ("General", ""),
            ("Esc / b", "Cancel decode and return home"),
            ("q", "Quit the application"),
            ("Ctrl+C", "Quit the application"),
            ("Ctrl+S", "Open settings panel"),
            ("?", "Toggle this help overlay"),
        ],
    };

    let mut lines = vec![
        Line::from(Span::styled(
            "Keybindings",
            colors.highlight.add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (key, description) in keybindings {
        if key.is_empty() && description.is_empty() {
            lines.push(Line::from(""));
        } else if description.is_empty() {
            lines.push(Line::from(Span::styled(
                key,
                colors.label.add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("{:16}", key), colors.accent),
                Span::styled(description, colors.text),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner_area);
}

/// Renders the human confirmation modal for plaintext verification.
fn draw_human_confirmation_screen(
    frame: &mut Frame,
    area: Rect,
    request: &HumanConfirmationRequest,
    colors: &TuiColors,
) {
    let modal_area = centered_rect(area, MODAL_WIDTH_PERCENT, MODAL_HEIGHT_PERCENT);
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::default()
        .title(" Confirm Plaintext? ")
        .title_style(colors.highlight)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(2, 2, 1, 1));

    let inner_area = modal_block.inner(modal_area);
    frame.render_widget(modal_block, modal_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(inner_area);

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

    let display_text = if request.text.len() > MAX_PLAINTEXT_PREVIEW_LEN {
        format!("{}...", &request.text[..MAX_PLAINTEXT_PREVIEW_LEN])
    } else {
        request.text.clone()
    };

    let plaintext_block = Block::default()
        .title(" Candidate Plaintext ")
        .title_style(colors.muted)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border)
        .padding(Padding::horizontal(1));

    let plaintext_inner = plaintext_block.inner(inner_chunks[2]);
    frame.render_widget(plaintext_block, inner_chunks[2]);

    let plaintext_paragraph =
        Paragraph::new(Span::styled(&display_text, colors.text)).wrap(Wrap { trim: false });
    frame.render_widget(plaintext_paragraph, plaintext_inner);

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

/// Renders a status message at the bottom of the screen with severity-based styling.
///
/// - `Info` / `Success` — uses success color
/// - `Warning` — uses warning color with ⚠ prefix
/// - `Error` — uses error color with ✗ prefix
fn draw_status_message(
    frame: &mut Frame,
    area: Rect,
    message: &str,
    severity: StatusSeverity,
    colors: &TuiColors,
) {
    let msg_area = Rect {
        x: area.x + 1,
        y: area.y + area.height.saturating_sub(2),
        width: area.width.saturating_sub(2),
        height: 1,
    };

    let (prefix, style) = match severity {
        StatusSeverity::Info => ("", colors.text),
        StatusSeverity::Success => ("✓ ", colors.success),
        StatusSeverity::Warning => ("⚠ ", colors.accent),
        StatusSeverity::Error => ("✗ ", colors.error),
    };

    let display = format!("{}{}", prefix, message);
    let paragraph = Paragraph::new(Span::styled(display, style));
    frame.render_widget(paragraph, msg_area);
}

/// Creates a centered rectangle within the given area.
pub fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    Rect::new(x, y, popup_width, popup_height)
}

/// Parses a quote string into quote and attribution parts.
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

    let block = Block::default()
        .title(" Choose Theme ")
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.accent);

    let inner = block.inner(area);
    frame.render_widget(block, area);

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

    Clear.render(modal_area, buf);

    let block = Block::default()
        .title(" Unsaved Changes ")
        .title_style(colors.accent.add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(colors.accent)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(modal_area);
    block.render(modal_area, buf);

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

    frame.render_widget(Clear, modal_area);

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

    let modal_area = Rect {
        x: area.x + 2,
        y: area.y + area.height.saturating_sub(modal_height + 2),
        width: modal_width.min(area.width.saturating_sub(4)),
        height: modal_height.min(area.height.saturating_sub(4)),
    };

    frame.render_widget(Clear, modal_area);

    let title = format!(" /{} ", search_text);
    let block = Block::default()
        .title(title)
        .title_style(colors.accent)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border);

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

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

/// Renders the quick search overlay.
fn draw_quick_search(
    frame: &mut Frame,
    area: Rect,
    entries: &[(String, String)],
    selected_index: usize,
    colors: &TuiColors,
) {
    let modal_width: u16 = 30;
    let modal_height: u16 = (entries.len() as u16 + 4).min(14);

    let modal_area = Rect {
        x: area.x + 2,
        y: area.y + area.height.saturating_sub(modal_height + 2),
        width: modal_width.min(area.width.saturating_sub(4)),
        height: modal_height.min(area.height.saturating_sub(4)),
    };

    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(" Open in... ")
        .title_style(colors.accent)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.border);

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    let visible_count = inner.height.saturating_sub(1) as usize;
    let start = if selected_index >= visible_count {
        selected_index - visible_count + 1
    } else {
        0
    };

    let lines: Vec<Line> = entries
        .iter()
        .enumerate()
        .skip(start)
        .take(visible_count)
        .map(|(idx, (name, _url))| {
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
        "[Enter] Open  [Esc] Cancel",
        colors.muted,
    )));

    let paragraph = Paragraph::new(all_lines);
    frame.render_widget(paragraph, inner);
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
        assert_eq!(centered.x, 35);
        assert_eq!(centered.y, 17);
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
