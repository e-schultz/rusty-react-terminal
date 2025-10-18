use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::models::View;
use crate::state::AppState;

/// Render the entire application
pub fn render_app(f: &mut Frame, app: &AppState) {
    let area = f.area();

    match app.current_view {
        View::FieldGuide => render_field_guide(f, app, area),
        View::GlitchSanctuary => render_sanctuary(f, app, area),
    }

    if app.show_command_palette {
        render_command_palette(f, app, area);
    }
}

/// Render the Field Guide view with grid layout
fn render_field_guide(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header + filter
            Constraint::Min(10),   // Grid content
            Constraint::Length(2), // Footer
        ])
        .split(area);

    // ===== HEADER SECTION =====
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(3)])
        .split(chunks[0]);

    // Title
    let title_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let header = Paragraph::new(
        Line::from(vec![Span::styled(&app.field_guide.data.meta.title, title_style)])
            .alignment(Alignment::Left),
    );
    f.render_widget(header, header_chunks[0]);

    // Filter buttons
    render_filter_buttons(f, app, header_chunks[1]);

    // ===== CONTENT SECTION (Grid) =====
    render_field_guide_grid(f, app, chunks[1]);

    // ===== FOOTER =====
    let footer_text = format!(
        "Total: {} | Filter: {} | [h/l] Nav | [j/k] Entry | [Enter] Details | [:] Command | [Tab] Sanctuary",
        app.field_guide.data.sections.len(),
        app.field_guide
            .active_filter
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("NONE")
    );
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left);
    f.render_widget(footer, chunks[2]);

    // ===== DETAIL MODAL (if viewing details) =====
    if app.field_guide.viewing_details {
        render_entry_detail_modal(f, app);
    }
}

/// Render filter buttons
fn render_filter_buttons(f: &mut Frame, app: &AppState, area: Rect) {
    let colors = vec!["CYAN", "PURPLE", "GREEN", "AMBER", "BLUE", "TEAL", "ROSE"];
    let button_width = 9u16; // Width of each button including spacing

    let mut x = area.x;
    for color in colors {
        if x + button_width > area.right() {
            break;
        }

        let button_area = Rect {
            x,
            y: area.y,
            width: button_width,
            height: area.height,
        };

        let is_active = app.field_guide.active_filter.as_ref().map(|s| s.as_str()) == Some(color.to_lowercase().as_str());

        let style = if is_active {
            Style::default()
                .fg(Color::Black)
                .bg(color_from_string(color))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(color_from_string(color))
                .add_modifier(Modifier::DIM)
        };

        let button = Paragraph::new(color)
            .style(style)
            .alignment(Alignment::Center);

        f.render_widget(button, button_area);
        x += button_width;
    }
}

/// Render field guide as 3-column grid using Layout constraints with scrolling
fn render_field_guide_grid(f: &mut Frame, app: &AppState, area: Rect) {
    let state = &app.field_guide;
    let filtered = state.filtered_sections();

    // Calculate total rows needed and current viewport
    let total_rows = (filtered.len() + 2) / 3; // Ceiling division for rows of 3
    let max_visible_rows = (area.height as usize) / 7; // 7 lines per row (6 card + 1 spacing)

    // Show scroll indicator if content exceeds viewport
    if total_rows > max_visible_rows && state.scroll_offset > 0 {
        // Scroll indicator (⬆ for scroll up available)
        let scroll_up_text = "⬆ scroll up";
        let scroll_up = Paragraph::new(scroll_up_text)
            .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM))
            .alignment(Alignment::Right);
        f.render_widget(scroll_up, Rect {
            x: area.right().saturating_sub(scroll_up_text.len() as u16 + 1),
            y: area.y,
            width: scroll_up_text.len() as u16,
            height: 1,
        });
    }

    // Split into rows of 3 columns, starting from scroll offset
    let mut row_y = area.y;
    let start_offset = state.scroll_offset * 3;
    let visible_sections: Vec<usize> = filtered
        .iter()
        .skip(start_offset)
        .take(max_visible_rows * 3)
        .copied()
        .collect();

    let mut remaining_sections = visible_sections;

    while !remaining_sections.is_empty() && row_y < area.bottom().saturating_sub(1) {
        // Take up to 3 sections for this row
        let row_sections: Vec<usize> = remaining_sections.drain(0..remaining_sections.len().min(3)).collect();
        let row_height = area.bottom() - row_y - 1; // Leave space for next row

        // Calculate equal widths for columns
        let col_widths: Vec<u16> = vec![
            (area.width - 2) / 3; // Each column gets 1/3 of width minus margins
            row_sections.len()
        ];

        // Create column areas using Layout
        let col_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                col_widths
                    .iter()
                    .map(|&w| Constraint::Length(w))
                    .collect::<Vec<_>>()
            )
            .split(Rect {
                x: area.x + 1,
                y: row_y,
                width: area.width.saturating_sub(2),
                height: row_height.min(6), // Cards are 6 lines tall
            });

        // Render each card in this row
        for (col_idx, &section_idx) in row_sections.iter().enumerate() {
            if col_idx >= col_areas.len() {
                break;
            }

            let section = &state.data.sections[section_idx];
            let is_selected = state.selected_section_idx == section_idx;
            let color = color_from_string(&section.color);

            // Card styling
            let border_style = if is_selected {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            let card_block = Block::default()
                .title(format!(" {} ", &section.title))
                .borders(Borders::ALL)
                .style(border_style);

            let card_area = col_areas[col_idx];
            f.render_widget(card_block.clone(), card_area);

            // Content inside card
            let inner = card_area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1,
            });

            let content = vec![
                format!("📦 {} entries", section.entries.len()),
                format!("🔷 {}", section.icon.to_uppercase()),
            ]
            .join("\n");

            let content_para = Paragraph::new(content)
                .style(Style::default().fg(color))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(content_para, inner);
        }

        row_y += 7; // Move down for next row (6 for card + 1 for spacing)
    }

    // Show scroll down indicator if more content below
    if state.scroll_offset + max_visible_rows < total_rows && row_y < area.bottom() {
        let scroll_down_text = "⬇ scroll down";
        let scroll_down = Paragraph::new(scroll_down_text)
            .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM))
            .alignment(Alignment::Right);
        f.render_widget(scroll_down, Rect {
            x: area.right().saturating_sub(scroll_down_text.len() as u16 + 1),
            y: area.bottom().saturating_sub(2),
            width: scroll_down_text.len() as u16,
            height: 1,
        });
    }

    // Render scrollbar on the right side if content overflows
    if total_rows > max_visible_rows {
        let scrollbar_area = Rect {
            x: area.right().saturating_sub(1),
            y: area.y + 1, // Below title area
            width: 1,
            height: area.height.saturating_sub(3), // Account for title and footer
        };
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            scrollbar_area,
            &mut state.scrollbar_state.clone(),
        );
    }
}

/// Render the Glitch Sanctuary view with split pane
fn render_sanctuary(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(2)])
        .split(area);

    // ===== HEADER =====
    let status_text = format!("STATUS: {}", app.sanctuary.data.meta.status);
    let status_color = match app.sanctuary.data.meta.status.as_str() {
        "UNSTABLE" => Color::Red,
        "CAUTION" => Color::Yellow,
        _ => Color::Green,
    };

    let header = Paragraph::new(
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Red)),
            Span::styled(
                &app.sanctuary.data.meta.title,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                &status_text,
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Left),
    );
    f.render_widget(header, chunks[0]);

    // ===== MAIN CONTENT (split left/right) =====
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    // LEFT: Programs + Records
    render_sanctuary_programs(f, app, content_chunks[0]);

    // RIGHT: System metrics
    render_system_metrics(f, app, content_chunks[1]);

    // ===== FOOTER =====
    let footer = Paragraph::new("[h/l] Programs | [j/k] Records | [Enter] Expand | [:] Command | [Tab] Guide")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left);
    f.render_widget(footer, chunks[2]);

    // ===== DETAIL MODAL (if viewing details) =====
    if let Some(expanded_idx) = app.sanctuary.expanded_record_idx {
        if expanded_idx == app.sanctuary.selected_record_idx {
            render_record_detail_modal(f, app);
        }
    }
}

/// Render programs and records list with better layout
fn render_sanctuary_programs(f: &mut Frame, app: &AppState, area: Rect) {
    let state = &app.sanctuary;

    // Use Layout for proper sectioning
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    // ===== TAB NAVIGATION =====
    let mut tab_line = vec![];
    for (i, program) in state.data.programs.iter().enumerate() {
        let is_active = state.active_program_idx == i;
        let color = color_from_string(&program.color);

        if is_active {
            tab_line.push(Span::styled(
                format!("  {}  ", &program.title),
                Style::default()
                    .fg(Color::Black)
                    .bg(color)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            tab_line.push(Span::styled(
                format!("  {}  ", &program.title),
                Style::default().fg(color),
            ));
        }

        if i < state.data.programs.len() - 1 {
            tab_line.push(Span::raw(" "));
        }
    }

    let tabs = Paragraph::new(Line::from(tab_line))
        .block(Block::default().borders(Borders::BOTTOM).style(Style::default().fg(Color::DarkGray)))
        .alignment(Alignment::Left);
    f.render_widget(tabs, chunks[0]);

    // ===== RECORDS LIST =====
    if let Some(program) = state.current_program() {
        let records: Vec<ListItem> = program
            .records
            .iter()
            .enumerate()
            .map(|(i, record)| {
                let is_selected = state.selected_record_idx == i;

                let status_symbol = match record.status {
                    crate::models::RecordStatus::Operational => "✓",
                    crate::models::RecordStatus::Caution => "⚠",
                    crate::models::RecordStatus::Degraded => "⚡",
                    crate::models::RecordStatus::Critical => "✗",
                };

                let status_color = match record.status {
                    crate::models::RecordStatus::Operational => Color::Green,
                    crate::models::RecordStatus::Caution => Color::Yellow,
                    crate::models::RecordStatus::Degraded => Color::Magenta,
                    crate::models::RecordStatus::Critical => Color::Red,
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(status_color)
                };

                let label = format!(" {} {}", status_symbol, record.name);
                ListItem::new(Line::from(Span::styled(label, style)))
            })
            .collect();

        let record_list = List::new(records)
            .block(
                Block::default()
                    .title(format!(" Records ({}) ", program.records.len()))
                    .borders(Borders::ALL),
            );

        f.render_widget(record_list, chunks[1]);
    }
}

/// Render system metrics panel
fn render_system_metrics(f: &mut Frame, app: &AppState, area: Rect) {
    let metrics = &app.sanctuary.data.metrics;

    let metric_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Min(2),
        ])
        .split(area);

    // Focus gauge
    let focus_gauge = Gauge::default()
        .block(Block::default().title(" Focus ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(metrics.focus as u16);
    f.render_widget(focus_gauge, metric_chunks[0]);

    // Energy gauge
    let energy_gauge = Gauge::default()
        .block(Block::default().title(" Energy ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(metrics.energy as u16);
    f.render_widget(energy_gauge, metric_chunks[1]);

    // Stability gauge
    let stability_gauge = Gauge::default()
        .block(Block::default().title(" Stability ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(metrics.stability as u16);
    f.render_widget(stability_gauge, metric_chunks[2]);

    // Creativity gauge
    let creativity_gauge = Gauge::default()
        .block(Block::default().title(" Creativity ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(metrics.creativity as u16);
    f.render_widget(creativity_gauge, metric_chunks[3]);

    // Info
    let info_text = format!("Last Reset: {}\nUptime: 17d 8h 42m", metrics.last_reset);
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });
    f.render_widget(info, metric_chunks[4]);
}

/// Render the command palette overlay
fn render_command_palette(f: &mut Frame, app: &AppState, area: Rect) {
    let popup_width = 60;
    let popup_height = 5;
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    };

    let block = Block::default()
        .title(" Command ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let prompt_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 1,
    };

    let input_text = format!(": {}_", app.command_input);
    let prompt = Paragraph::new(input_text).style(Style::default().fg(Color::Green));
    f.render_widget(prompt, prompt_area);
}

/// Render a centered detail modal for entry/record information
fn render_detail_modal(f: &mut Frame, title: &str, content_lines: Vec<String>, color: Color) {
    let area = f.area();

    // Calculate modal size (centered, 80% of available space)
    let modal_width = (area.width as f32 * 0.8) as u16;
    let modal_height = (content_lines.len() as u16 + 4).min(area.height - 2);

    let modal_x = (area.width - modal_width) / 2;
    let modal_y = (area.height - modal_height) / 2;

    let modal_area = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_width,
        height: modal_height,
    };

    // Create semi-transparent background by rendering a dark overlay
    let bg_block = Block::default()
        .style(Style::default().bg(Color::Black));
    f.render_widget(bg_block, area);

    // Create modal with content
    let modal_block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD));

    f.render_widget(modal_block, modal_area);

    // Render content inside modal
    let inner = modal_area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    let content_text = content_lines.join("\n");
    let content_para = Paragraph::new(content_text)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(content_para, inner);
}

/// Render entry detail modal for Field Guide
fn render_entry_detail_modal(f: &mut Frame, app: &AppState) {
    if let Some(section) = app.field_guide.current_section() {
        if let Some(entry) = app.field_guide.current_entry() {
            let color = color_from_string(&section.color);

            let mut content = vec![
                format!("Pattern: {}", entry.pattern),
                String::new(),
                "Description:".to_string(),
                entry.description.clone(),
                String::new(),
            ];

            if !entry.signals.is_empty() {
                content.push("Signals:".to_string());
                for signal in &entry.signals {
                    content.push(format!("  • {}", signal));
                }
                content.push(String::new());
            }

            content.push(format!("Protocol: {}", entry.protocol));
            content.push(String::new());
            content.push("[Enter] Close | [j/k] Navigate entries".to_string());

            render_detail_modal(f, &format!("{} - {}", section.title, entry.pattern), content, color);
        }
    }
}

/// Render record detail modal for Glitch Sanctuary
fn render_record_detail_modal(f: &mut Frame, app: &AppState) {
    if let Some(program) = app.sanctuary.current_program() {
        if let Some(record) = app.sanctuary.current_record() {
            let color = color_from_string(&program.color);

            let mut content = vec![
                format!("Name: {}", record.name),
                format!("Status: {}", record.status),
                String::new(),
                "Description:".to_string(),
                record.description.clone(),
                String::new(),
            ];

            if !record.steps.is_empty() {
                content.push("Steps:".to_string());
                for (idx, step) in record.steps.iter().enumerate() {
                    content.push(format!("  {}. {}", idx + 1, step));
                }
                content.push(String::new());
            }

            content.push("[Enter] Close | [j/k] Navigate records".to_string());

            render_detail_modal(f, &format!("{} - {}", program.title, record.name), content, color);
        }
    }
}

/// Convert color string to ratatui Color
fn color_from_string(color: &str) -> Color {
    match color {
        "cyan" | "CYAN" => Color::Cyan,
        "purple" | "PURPLE" => Color::Magenta,
        "green" | "GREEN" => Color::Green,
        "amber" | "AMBER" => Color::Yellow,
        "blue" | "BLUE" => Color::Blue,
        "teal" | "TEAL" => Color::Cyan,
        "rose" | "ROSE" => Color::LightRed,
        "stone" | "STONE" => Color::Gray,
        _ => Color::White,
    }
}

