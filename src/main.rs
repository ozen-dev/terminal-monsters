mod models;
mod ui;
mod watcher;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use models::{
    dex::load_dex,
    party::{get_random_party_mon, initialize_party, save_party},
};
use ratatui::prelude::*;
use std::io::{stdout, Result};
use ui::{header_ui::render_header_ui, party_ui::render_party_ui};

fn main() -> Result<()> {
    // Setup terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Load data
    let dex = load_dex();
    let mut party = initialize_party();
    let mut selected_row = 0;

    loop {
        // Draw UI
        terminal.draw(|frame| {
            let area = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(4), Constraint::Percentage(100)].as_ref())
                .split(area);

            render_header_ui(frame, chunks[0]);
            render_party_ui(frame, chunks[1], &party, selected_row);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => {
                            let new_party_mon = get_random_party_mon(&dex);
                            party.push(new_party_mon);
                            if let Err(e) = save_party(&party) {
                                eprintln!("Failed to save party: {}", e);
                            }
                        }
                        KeyCode::Down => {
                            if selected_row < party.len().saturating_sub(1) {
                                selected_row += 1;
                            }
                        }
                        KeyCode::Up => {
                            if selected_row > 0 {
                                selected_row -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Terminate
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
