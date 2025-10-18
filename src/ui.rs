use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

/// Render the Field Guide view
fn render_field_guide(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);

    // Header
    let header_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let header = Paragraph::new(
        Line::from(vec![
            Span::styled("  ", header_style),
            Span::styled(&app.field_guide.data.meta.title, header_style),
        ])
        .alignment(Alignment::Center),
    );

    f.render_widget(header, chunks[0]);

    // Main content area (sections grid)
    let state = &app.field_guide;
    let filtered = state.filtered_sections();

    let sections: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(_i, &section_idx)| {
            let section = &state.data.sections[section_idx];
            let is_selected = state.selected_section_idx == section_idx;

            let style = if is_selected {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                let color = color_from_string(&section.color);
                Style::default().fg(color)
            };

            ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(&section.title, style),
                Span::styled(
                    format!(" [{}]", section.entries.len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let sections_list = List::new(sections)
        .block(Block::default().borders(Borders::ALL).title(" Sections "));

    f.render_widget(sections_list, chunks[1]);

    // Footer
    let footer_text = if state.active_filter.is_some() {
        format!(
            "Filter: {} | [↔] Navigate | [↕] Entries | [Enter] Details | [:] Command | [Tab] Switch",
            state.active_filter.as_ref().unwrap()
        )
    } else {
        "[↔] Navigate | [↕] Entries | [Enter] Details | [:] Command | [Tab] Switch".to_string()
    };

    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

/// Render the Glitch Sanctuary view
fn render_sanctuary(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);

    // Header with status
    let header_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);

    let status_text = format!("STATUS: {}", app.sanctuary.data.meta.status);
    let header = Paragraph::new(
        Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Red)),
            Span::styled(&app.sanctuary.data.meta.title, header_style),
            Span::raw("  "),
            Span::styled(
                &status_text,
                Style::default().fg(Color::Red),
            ),
        ])
        .alignment(Alignment::Center),
    );

    f.render_widget(header, chunks[0]);

    // Programs/tabs
    let state = &app.sanctuary;
    let programs: Vec<ListItem> = state
        .data
        .programs
        .iter()
        .enumerate()
        .map(|(i, program)| {
            let is_selected = state.active_program_idx == i;

            let style = if is_selected {
                let color = color_from_string(&program.color);
                Style::default()
                    .fg(color)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                let color = color_from_string(&program.color);
                Style::default().fg(color)
            };

            ListItem::new(Line::from(vec![Span::styled(&program.title, style)]))
        })
        .collect();

    let programs_list = List::new(programs)
        .block(Block::default().borders(Borders::ALL).title(" Programs "));

    f.render_widget(programs_list, chunks[1]);

    // Footer
    let footer = Paragraph::new("[↔] Programs | [↕] Records | [Enter] Expand | [:] Command | [Tab] Switch")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
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

    let input_text = format!(": {}_", app.command_input); // Simple _ cursor
    let prompt = Paragraph::new(input_text).style(Style::default().fg(Color::Green));
    f.render_widget(prompt, prompt_area);
}

/// Convert color string to ratatui Color
fn color_from_string(color: &str) -> Color {
    match color {
        "cyan" => Color::Cyan,
        "purple" => Color::Magenta,
        "green" => Color::Green,
        "amber" => Color::Yellow,
        "blue" => Color::Blue,
        "teal" => Color::Cyan,
        "rose" => Color::LightRed,
        "stone" => Color::Gray,
        _ => Color::White,
    }
}
