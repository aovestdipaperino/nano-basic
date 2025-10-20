//! A Commodore 64 BASIC V2 interpreter with a terminal UI.
//!
//! This interpreter emulates the behavior of the Commodore 64 BASIC V2 environment,
//! including tokenization, program storage, and execution. It provides a terminal-based
//! user interface that replicates the classic C64 blue screen aesthetic.

mod expression;
mod interpreter;
mod tokens;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = ui::App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut ui::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                KeyCode::Enter => {
                    if app.handle_enter() {
                        return Ok(()); // QUIT command was issued
                    }
                }
                KeyCode::Backspace => {
                    app.handle_backspace();
                }
                KeyCode::Char(c) => {
                    app.handle_char(c);
                }
                _ => {}
            }
        }
    }
}
