pub mod app;
pub mod event;
pub mod handlers;

use std::io::{stdout, Error};

use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, read, Event, KeyCode},
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use crate::app::{App, InputMode, ui::draw_main_layout};

fn main() -> Result<(), Error> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    stdout
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    loop {
        terminal.draw(|rect| draw_main_layout(rect, &app))?;

        match read()? {
            Event::Key(key) => {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('a') => app.set_editing_mode(),
                        KeyCode::Char('q') => break,

                        _ => {}
                    },
                    InputMode::Editing => app.handle_editing_mode(key)
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
