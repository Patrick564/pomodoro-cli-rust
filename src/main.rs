pub mod app;
pub mod event;
pub mod handlers;

use std::{io::{stdout, Error}, time::{Instant, Duration}};

use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, read, Event, KeyCode, poll},
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

    let app = &mut App::default();

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(200);

    loop {
        terminal.draw(|rect| draw_main_layout(rect, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if poll(timeout)? {
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

        if app.total_time.len() > 0 {
            // if app.progress <= app.total_time[0] {
                app.on_tick();
                last_tick = Instant::now();
            // }
        }
    }

    disable_raw_mode()?;

    terminal.backend_mut()
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;

    terminal.show_cursor()?;

    Ok(())
}
