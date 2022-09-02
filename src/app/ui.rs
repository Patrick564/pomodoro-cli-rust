// use crossterm::terminal;
use tui::{
    Frame,
    backend::Backend,
    widgets::{Row, Cell, Paragraph, Block, Borders, Table, BorderType, ListItem, List},
    layout::{Direction, Layout, Constraint, Alignment, Rect},
    style::{Style, Color},
    text::{Span, Spans}
};

use crate::{App, InputMode};

fn draw_title<B>(f: &mut Frame<B>, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new("Pomodoro CLI")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(chunks);

    f.render_widget(paragraph, layout_chunk);
}

fn draw_add_block<B>(f: &mut Frame<B>, app: &App,layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Paragraph::new(app.input.as_ref())
        .style(
            match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            }
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Add new time (hours:seconds):")
        );

    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            f.set_cursor(
                layout_chunk.x + app.input.len() as u16 + 1,
                layout_chunk.y + 1
            );
        }
    }

    f.render_widget(chunks, layout_chunk);
}

fn draw_pomodoro_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend
{
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    let chunks = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Messages")
        );

    f.render_widget(chunks, layout_chunk);
}

fn draw_content<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where   B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10)
        ].as_ref())
        .split(layout_chunk);

    draw_add_block(f, app, chunks[0]);
    draw_pomodoro_block(f, app, chunks[1]);
}

fn draw_commands<B>(f: &mut Frame<B>, layout_chunk: Rect)
where
    B: Backend,
{
    let commands_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Commands");
    let rows = vec![
        Row::new(vec![
            Cell::from(Span::styled("<a>", Style::default().fg(Color::LightCyan))),
            Cell::from(Span::styled("Add new time", Style::default().fg(Color::Gray))),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("<p>", Style::default().fg(Color::LightCyan))),
            Cell::from(Span::styled("Pause time", Style::default().fg(Color::Gray))),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("<s>", Style::default().fg(Color::LightCyan))),
            Cell::from(Span::styled("Start time", Style::default().fg(Color::Gray))),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("<ctrl + c>", Style::default().fg(Color::LightCyan))),
            Cell::from(Span::styled("Exit", Style::default().fg(Color::Gray))),
        ]),
    ];
    let table =Table::new(rows)
        .block(commands_block)
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1);

    f.render_widget(table, layout_chunk);
}

fn draw_body<'a, B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Min(10),
        ].as_ref())
        .split(layout_chunk);

    draw_content(f, app, chunks[0]);
    draw_commands(f, chunks[1]);
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10)
        ].as_ref())
        .split(f.size());

    draw_title(f, parent_layout[0]);
    draw_body(f, app, parent_layout[1]);
}
