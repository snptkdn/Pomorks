use crate::app::App;
use pomorks_data_manage::todo::TodoList;
use std::fs::{read, read_to_string};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, todo_list: &TodoList) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_title(f, app, chunks[0]);
    draw_tasks(f, app, chunks[1]);
    draw_status(f, app, chunks[2]);
}

// タイトルの描画
fn draw_title<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let version = env!("CARGO_PKG_VERSION");
    let title = Spans::from(vec![
        Span::styled(
            "**** Pomorks Tui ****",
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Gray),
        ),
        Span::raw("  "),
        Span::styled(
            format!("- version - {}", version),
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    let block = Block::default().borders(Borders::ALL);

    let paragraph = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_tasks<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    draw_task_list(f, app, chunks[0]);
    draw_selected_task_detail(f, app, chunks[1]);
}

fn draw_task_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .direction(Direction::Horizontal)
        .split(area);

    let todos: Vec<ListItem> = app
        .todos
        .items
        .iter()
        // format display
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(format!("{:?}", i)))]))
        .collect();

    let todos = List::new(todos)
        .block(Block::default().borders(Borders::ALL).title("Todo"))
        .highlight_style(Style::default().fg(Color::Red))
        .highlight_symbol("> ");
    f.render_stateful_widget(todos, chunks[0], &mut app.todos.state);
}

fn draw_selected_task_detail<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let selected_index = app.todos.state.selected();
    let task_detail = match selected_index {
        Some(ind) => {
            vec![
                Spans::from(vec![Span::styled(
                    format!("title: {}", app.todos.items[ind].title),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::LightRed),
                )]),
                Spans::from(vec![Span::styled(
                    format!("tag: #{}", app.todos.items[ind].tag),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::LightBlue),
                )]),
                Spans::from(vec![Span::styled(
                    format!("project: @{}", app.todos.items[ind].project),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::LightGreen),
                )]),
                Spans::from(vec![Span::styled(
                    format!(
                        "Pomodoro: {}",
                        if app.todos.items[ind].executed_count < app.todos.items[ind].estimate_count
                        {
                            "■".repeat(app.todos.items[ind].executed_count)
                                + &"□".repeat(
                                    app.todos.items[ind].estimate_count
                                        - app.todos.items[ind].executed_count,
                                )
                        } else {
                            "■".repeat(app.todos.items[ind].executed_count)
                        }
                    ),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Gray),
                )]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled(
                    "Detail:",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Gray),
                )]),
                Spans::from(vec![Span::styled(
                    format!("{}", app.todos.items[ind].detail),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Gray),
                )]),
            ]
        }
        None => vec![Spans::from(vec![Span::raw("")])],
    };

    let block = Block::default().borders(Borders::ALL);
    let parahraph = Paragraph::new(task_detail)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(parahraph, area);
}

fn draw_status<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    draw_timer(f, app, chunks[0]);
    draw_task_status(f, app, chunks[1]);
}

fn draw_timer<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(9, 10), Constraint::Ratio(1, 10)].as_ref())
        .margin(2)
        .split(area);

    let timer = Spans::from(vec![Span::styled(
        "25:00",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White),
    )]);

    let time = app.progress * 1000.0;
    let time = time as u16;
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Red))
        .percent(time);
    f.render_widget(gauge, chunks[0]);

    let timer_paragraph = Paragraph::new(timer)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(timer_paragraph, chunks[1]);
}

fn draw_task_status<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let task = vec![
        Spans::from(vec![
            Span::styled(
                format!("title: {}", "Task  "),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightRed),
            ),
            Span::styled(
                format!("tag: #{}", "tags  "),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightBlue),
            ),
            Span::styled(
                format!("project: {}", "project  "),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            ),
        ]),
        Spans::from(vec![Span::styled(
            format!("Pomodoro: {}", "■■□"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Gray),
        )]),
        Spans::from(vec![Span::styled(
            format!("Process: {}", "WORK_1"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Gray),
        )]),
    ];

    let block = Block::default().borders(Borders::ALL);
    let task_paragraph = Paragraph::new(task)
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(task_paragraph, area);
}
