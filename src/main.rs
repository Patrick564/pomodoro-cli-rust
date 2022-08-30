use std::{io::{stdout, Error}, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
    style::{Style, Color},
    layout::Alignment
};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

fn main() -> Result<(), Error> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    stdout
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // loop {
    terminal.draw(|f| {
        // let size = f.size();
        // let block = Block::default()
        //     .title("Block")
        //     .borders(Borders::ALL);

        // f.render_widget(block, size);

        let title = Paragraph::new("Pomodoro CLI")
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
            );

        f.render_widget(title, f.size());
    })?;
    // }

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;

    terminal.backend_mut()
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;

    terminal.show_cursor()?;

    Ok(())
}
