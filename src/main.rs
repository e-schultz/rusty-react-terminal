mod data;
mod input;
mod models;
mod state;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    // Load data
    tracing::info!("Loading application data...");
    let field_guide_data = data::load_field_guide("data/field_guide.toml")?;
    let sanctuary_data = data::load_sanctuary("data/sanctuary.toml")?;

    tracing::info!(
        "Loaded field guide with {} sections",
        field_guide_data.sections.len()
    );
    tracing::info!(
        "Loaded sanctuary with {} programs",
        sanctuary_data.programs.len()
    );

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = AppState::new(field_guide_data, sanctuary_data);
    let result = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| ui::render_app(f, &app))?;

        // Handle input
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Check for quit early
                if matches!(key.code, KeyCode::Char('q')) && !app.show_command_palette {
                    break;
                }

                input::handle_key_event(&mut app, key);
            }
        }
    }

    Ok(())
}
