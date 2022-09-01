pub mod app;
pub mod event;
pub mod handlers;

use std::io::{stdout, Error};

use tui::{backend::CrosstermBackend, Terminal, text::Spans};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, read, Event, KeyCode, KeyModifiers},
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use crate::app::ui::draw;
// use crate::event::key::Key;

fn main() -> Result<(), Error> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    stdout
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let timers: Vec<Spans> = vec![];

    loop {
        terminal.draw(|rect| draw(rect, timers.clone()))?;

        match read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break;
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    break;
                }

                if key.code == KeyCode::Char('a') {
                    // terminal.clear()?;
                }
            },

            _ => {}
        }
    }

    disable_raw_mode()?;

    terminal.backend_mut()
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;

    terminal.show_cursor()?;

    Ok(())
}
