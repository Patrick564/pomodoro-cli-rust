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

use crate::app::ui::draw_main_layout;
// use crate::event::key::Key;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

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
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    break;
                }

                if key.code == KeyCode::Char('a') {
                    // terminal.clear()?;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing
                        },
                        KeyCode::Char('q') => {
                            break;
                        }

                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                        },
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        },
                        KeyCode::Backspace => {
                            app.input.pop();
                        },
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        },

                        _ => {}
                    }
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
