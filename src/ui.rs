use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
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

/// Render field guide as 3-column grid using Layout constraints
fn render_field_guide_grid(f: &mut Frame, app: &AppState, area: Rect) {
    let state = &app.field_guide;
    let filtered = state.filtered_sections();

    // Split into rows of 3 columns
    let mut row_y = area.y;
    let mut remaining_sections = filtered.clone();

    while !remaining_sections.is_empty() && row_y < area.bottom() {
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

