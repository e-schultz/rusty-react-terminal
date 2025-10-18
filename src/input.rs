use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::AppState;
use crate::models::View;

/// Handle keyboard input events
pub fn handle_key_event(app: &mut AppState, key: KeyEvent) {
    if app.show_command_palette {
        handle_command_palette_input(app, key);
    } else {
        handle_view_input(app, key);
    }
}

/// Handle input for the command palette
fn handle_command_palette_input(app: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.toggle_command_palette();
        }
        KeyCode::Enter => {
            process_command(app, app.command_input.clone());
            app.toggle_command_palette();
        }
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'u' => {
            app.command_input.clear();
        }
        KeyCode::Char(c) => {
            app.command_input.push(c);
        }
        _ => {}
    }
}

/// Process command palette commands
fn process_command(app: &mut AppState, command: String) {
    let cmd = command.trim().to_lowercase();

    // Parse commands like "view field-guide", "view sanctuary", "help", "quit"
    if let Some(view_cmd) = cmd.strip_prefix("view ") {
        if let Some(view) = View::from_str(view_cmd) {
            app.switch_view(view);
        }
    } else if cmd == "help" {
        // Help would be shown in a separate view
    } else if cmd == "quit" || cmd == "q" {
        // Signal to quit (handled in main loop)
    }
}

/// Handle input for the current view
fn handle_view_input(app: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(':') if key.modifiers.is_empty() => {
            app.toggle_command_palette();
        }
        KeyCode::Char('q') if key.modifiers.is_empty() => {
            // Signal to quit
        }
        KeyCode::Tab => {
            // Switch to next view
            match app.current_view {
                View::FieldGuide => app.switch_view(View::GlitchSanctuary),
                View::GlitchSanctuary => app.switch_view(View::FieldGuide),
            }
        }
        _ => {
            // Delegate to view-specific handlers
            match app.current_view {
                View::FieldGuide => handle_field_guide_input(app, key),
                View::GlitchSanctuary => handle_sanctuary_input(app, key),
            }
        }
    }
}

/// Handle input for the Field Guide view
fn handle_field_guide_input(app: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.field_guide.prev_section();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.field_guide.next_section();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.field_guide.prev_entry();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.field_guide.next_entry();
        }
        KeyCode::Enter => {
            app.field_guide.viewing_details = !app.field_guide.viewing_details;
        }
        KeyCode::Char('f') => {
            // Cycle through filters (this would be a command palette thing in real app)
        }
        KeyCode::Char('0') => {
            app.field_guide.active_filter = None;
        }
        _ => {}
    }
}

/// Handle input for the Glitch Sanctuary view
fn handle_sanctuary_input(app: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.sanctuary.prev_program();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.sanctuary.next_program();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.sanctuary.prev_record();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.sanctuary.next_record();
        }
        KeyCode::Enter => {
            if app.sanctuary.current_program().is_some() {
                app.sanctuary
                    .toggle_record_details(app.sanctuary.selected_record_idx);
            }
        }
        _ => {}
    }
}
