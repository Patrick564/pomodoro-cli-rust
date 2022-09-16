use indicatif::ProgressBar;
use tui::{
    Frame,
    backend::Backend,
    widgets::{Row, Cell, Paragraph, Block, Borders, Table, BorderType, ListItem, List, Gauge},
    layout::{Direction, Layout, Constraint, Alignment, Rect},
    style::{Style, Color, Modifier},
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
                .title("Add new time (hours:seconds)")
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

fn draw_time_bar<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend
{
    let bar = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Pomodoro Time"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black).add_modifier(Modifier::ITALIC))
        .label("a")
        .percent(app.progress);

    // let pbar = ProgressBar::new(10);

    f.render_widget(bar, layout_chunk);
}

fn draw_pomodoro_block<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend
{
    let pomodoro_time: Vec<ListItem> = app
        .messages
        .iter()
        .map(|time| {
            ListItem::new(Spans::from(Span::styled(time, Style::default().fg(Color::LightCyan))))
        }).collect();

    let chunksa = List::new(pomodoro_time)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Clock")
        );

    let chunksb = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(1),
        ].as_ref())
        .margin(1)
        .split(layout_chunk);

    let chunks = Block::default()
        .borders(Borders::ALL);

    f.render_widget(chunks, layout_chunk);

    draw_time_bar(f, app, chunksb[0]);

}

fn draw_content<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend,
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

fn draw_body<'a, B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
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

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &mut App)
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
