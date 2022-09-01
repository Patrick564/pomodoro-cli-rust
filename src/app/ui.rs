// use crossterm::terminal;
use tui::{
    Frame,
    backend::Backend,
    widgets::{Row, Cell, Paragraph, Block, Borders, Table, BorderType},
    layout::{Direction, Layout, Constraint, Alignment},
    style::{Style, Color},
    text::{Span, Spans}
};

fn draw_title<'a>() -> Paragraph<'a> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    Paragraph::new("Pomodoro CLI")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(title_block)
}

fn draw_body<'a>(c: Vec<Spans<'a>>) -> Paragraph<'a> {
    let body_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);


    Paragraph::new(c)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(body_block)
}

fn draw_add_pomodoro<'a>() -> Block<'a> {
    // Terminal::show_cursor(&mut terminal)?;

    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain)
        .title("Set in format hours:minutes")

    // Paragraph::new("Set in format hours:minutes")
    //     .style(Style::default().fg(Color::LightCyan))
    //     .alignment(Alignment::Center)
    //     .block(add_pomodoro_block)
}

fn draw_commands<'a>() -> Table<'a> {
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

    Table::new(rows)
        .block(commands_block)
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

pub fn draw<B>(screen: &mut Frame<B>, clock: Vec<Spans>)
where
    B: Backend,
{
    let area = screen.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(area);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let add_pomodoro_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(body_chunks[0]);

    let title = draw_title();
    let body = draw_body(clock);
    let add_pomodoro = draw_add_pomodoro();
    let commands = draw_commands();

    screen.set_cursor(32, 32);

    screen.render_widget(title, chunks[0]);
    screen.render_widget(body, add_pomodoro_chunks[1]);
    screen.render_widget(add_pomodoro, add_pomodoro_chunks[0]);
    screen.render_widget(commands, body_chunks[1]);
}
