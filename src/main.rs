use std::{io::{stdout, Error}, vec};

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, BorderType, Table, Cell, Row},
    Terminal,
    style::{Style, Color},
    layout::{Layout, Alignment, Direction, Constraint},
    text::{Span, Spans}
};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, read, Event, KeyCode},
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

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                .split(f.size());

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
                .split(chunks[1]);

            let rows = vec![
                Row::new(vec![
                    Cell::from(Span::styled("< q >", Style::default().fg(Color::LightCyan))),
                    Cell::from(Span::styled("Exit", Style::default().fg(Color::Gray))),
                ]),
            ];

            let title = Paragraph::new("Pomodoro CLI")
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                );
            let body = Paragraph::new(
                vec![
                    Spans::from(Span::raw("Loading")),
                    Spans::from(Span::raw("Ticker")),
                ]
            )
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );
            let help = Table::new(rows)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .title("Help"),
                )
                .widths(&[Constraint::Length(11), Constraint::Min(20)])
                .column_spacing(1);

            f.render_widget(title, chunks[0]);
            f.render_widget(body, body_chunks[0]);
            f.render_widget(help, body_chunks[1]);
        })?;

        let input = match read()? {
            Event::Key(event) => event.code,
            _ => continue,
        };

        if input == KeyCode::Char('q') {
            break;
        }
    }

    // thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;

    terminal.backend_mut()
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;

    terminal.show_cursor()?;

    Ok(())
}
