use crate::app::{App, Tab};
use crate::date_manage::{get_this_month, get_this_week};
use chrono::prelude::*;
use num_traits::FromPrimitive;
use pomorks_data_manage::data_manage_json::DATE_FORMAT;
use pomorks_data_manage::todo::TodoItem;
use pomorks_data_manage::todo::{State, ONE_MINUTE};
use std::cmp::min;
use std::ops::Div;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{BarChart, Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_title(f, chunks[0]);
    match app.selected_tab {
        Tab::Main => {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Min(0),
                        Constraint::Length(5),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);
            if app.show_add_todo {
                draw_add_todo(f, app);
            } else {
                draw_tasks(f, app, chunks[0]);
            }
            draw_status(f, app, chunks[1]);
            draw_under_status_bar(f, app, chunks[2]);
        }
        Tab::Statistics => draw_statics(f, app, chunks[1]),
    };
}

// タイトルの描画
fn draw_title<B: Backend>(f: &mut Frame<B>, area: Rect)
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
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .direction(Direction::Horizontal)
        .split(area);

    let is_selected = {
        |todo: &TodoItem| match &app.todo_focus {
            Some(focus) => todo.id == focus.id,
            None => false,
        }
    };

    let todos_title: Vec<ListItem> = app
        .todos
        .items
        .iter()
        .map(|todo| {
            ListItem::new(vec![Spans::from(Span::styled(
                format!("{}", todo.title),
                get_style(is_selected(todo), todo.finished),
            ))])
        })
        .collect();

    let todos_tag: Vec<ListItem> = app
        .todos
        .items
        .iter()
        .map(|todo| {
            ListItem::new(vec![Spans::from(Span::styled(
                format!("{}", todo.tag),
                get_style(is_selected(todo), todo.finished),
            ))])
        })
        .collect();

    let todos_project: Vec<ListItem> = app
        .todos
        .items
        .iter()
        .map(|todo| {
            ListItem::new(vec![Spans::from(Span::styled(
                format!("{}", todo.tag),
                get_style(is_selected(todo), todo.finished),
            ))])
        })
        .collect();

    let todos_title = List::new(todos_title)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .title("Todo"),
        )
        .highlight_style(Style::default().fg(Color::Red))
        .highlight_symbol("> ");

    let todos_tag = List::new(todos_tag)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::Red));

    let todos_project = List::new(todos_project)
        .block(Block::default().borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::Red));

    f.render_stateful_widget(todos_title, chunks[0], &mut app.todos.state);
    f.render_stateful_widget(todos_tag, chunks[1], &mut app.todos.state);
    f.render_stateful_widget(todos_project, chunks[2], &mut app.todos.state);
}

fn get_style(is_selected: bool, is_finished: bool) -> Style {
    match (is_selected, is_finished) {
        (true, true) => Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::CROSSED_OUT),
        (true, false) => Style::default().fg(Color::Green),
        (false, true) => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::CROSSED_OUT),
        (false, false) => Style::default(),
    }
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

    let progressed_time = if let Some(start) = app.start_time {
        (Local::now() - start).num_seconds() as i64
    } else {
        0
    };

    let remaind_time = app.limit_time as i64 - progressed_time;

    let timer = Spans::from(vec![Span::styled(
        format!(
            "{}:{:>02}",
            remaind_time.div(ONE_MINUTE as i64),
            remaind_time % ONE_MINUTE as i64
        ),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White),
    )]);

    let percentage = (progressed_time as f64 / app.limit_time as f64) * 100.0;
    let percentage = if percentage > 100.0 {
        100.0
    } else {
        percentage
    };
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Red))
        .percent(percentage as u16);
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
    let task_focus = app.todo_focus.clone();
    let (title, tag, project, estimate_count, executed_count) = match task_focus {
        Some(task) => (
            task.title,
            task.tag,
            task.project,
            task.estimate_count,
            task.executed_count,
        ),
        None => ("-".to_string(), "-".to_string(), "-".to_string(), 0, 0),
    };

    let status = vec![
        Spans::from(vec![
            Span::styled(
                format!("title: {}", title),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightRed),
            ),
            Span::styled(
                format!("  tag: #{}", tag),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightBlue),
            ),
            Span::styled(
                format!("  project: {}", project),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            ),
        ]),
        Spans::from(vec![Span::styled(
            format!(
                "Pomodoro: {}",
                if executed_count < estimate_count {
                    "■".repeat(executed_count) + &"□".repeat(estimate_count - executed_count)
                } else {
                    "■".repeat(executed_count)
                }
            ),
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::styled(
            format!("Process: {}", State::get_state_name(&app.state)),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Gray),
        )]),
    ];

    let block = Block::default().borders(Borders::ALL);
    let task_paragraph = Paragraph::new(status)
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(task_paragraph, area);
}

fn draw_add_todo<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Min(0),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Min(0),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunks_vert[1]);

    let status = vec![Spans::from(vec![Span::raw(app.new_todo_string.clone())])];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("ADD TODO")
        .style(Style::default().bg(Color::DarkGray));
    let task_paragraph = Paragraph::new(status)
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(task_paragraph, chunks[1]);
}

fn draw_under_status_bar<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(area);

    draw_message(f, app, chunks[0]);
    draw_today_workcount(f, app, chunks[1]);
    draw_total_estimate(f, app, chunks[2]);
}
fn draw_message<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let title = Spans::from(vec![Span::styled(
        format!("message: {}", app.status),
        Style::default().fg(Color::Red),
    )]);
    let block = Block::default().borders(Borders::ALL);

    let paragraph = Paragraph::new(title).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_today_workcount<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let message = Spans::from(vec![Span::styled(
        format!("today: {}", app.todays_executed_count),
        Style::default().fg(Color::Blue),
    )]);
    let block = Block::default().borders(Borders::ALL);

    let paragraph = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_total_estimate<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let estimate_count = app
        .todos
        .items
        .iter()
        .filter(|todo| !todo.finished)
        .fold(0, |sum, todo| sum + todo.estimate_count);

    let executed_except_overestimate_count = app
        .todos
        .items
        .iter()
        .filter(|todo| !todo.finished)
        .fold(0, |sum, todo| {
            sum + min(todo.executed_count, todo.estimate_count)
        });
    let message = Spans::from(vec![Span::styled(
        format!(
            "estimate: {}/{}",
            executed_except_overestimate_count, estimate_count
        ),
        Style::default().fg(Color::Green),
    )]);
    let block = Block::default().borders(Borders::ALL);

    let paragraph = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_statics<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(60),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(area);

    draw_chart_of_week(f, app, chunks[0]);
    draw_chart_of_month(f, app, chunks[1]);
    draw_chart_of_year(f, app, chunks[2]);
}

fn draw_chart_of_week<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block_weekly = Block::default()
        .borders(Borders::ALL)
        .title("THIS WEEK")
        .title_alignment(Alignment::Center);

    let one_week = get_this_week(Local::today()).unwrap();

    let mon_str = &Weekday::Mon.to_string() as &str;
    let tue_str = &Weekday::Tue.to_string() as &str;
    let wed_str = &Weekday::Wed.to_string() as &str;
    let thu_str = &Weekday::Thu.to_string() as &str;
    let fri_str = &Weekday::Fri.to_string() as &str;
    let sat_str = &Weekday::Sat.to_string() as &str;
    let sun_str = &Weekday::Sun.to_string() as &str;

    let one_week_str = vec![
        mon_str, tue_str, wed_str, thu_str, fri_str, sat_str, sun_str,
    ];

    let one_week_pomodoro_count: Vec<u64> = one_week
        .iter()
        .map(|date| {
            app.task_log
                .iter()
                .filter(|log| {
                    let date_each = Local
                        .datetime_from_str(&log.date, DATE_FORMAT)
                        .unwrap()
                        .date();
                    (date_each.year(), date_each.month(), date_each.day())
                        == (date.year(), date.month(), date.day())
                })
                .count() as u64
        })
        .collect();

    let one_week_data: Vec<(&str, u64)> = one_week_str
        .into_iter()
        .zip(one_week_pomodoro_count)
        .collect();

    let chart_weekly = BarChart::default()
        .block(block_weekly)
        .data(&one_week_data)
        .bar_width(3)
        .bar_style(Style::default().fg(Color::LightRed))
        .bar_gap(1);
    f.render_widget(chart_weekly, area);
}

fn draw_chart_of_month<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            "{}",
            Month::from_u32(Local::today().month()).unwrap().name()
        ))
        .title_alignment(Alignment::Center);

    let one_month = get_this_month(Local::today()).unwrap();

    // TODO:ここどうにかしたい。。
    let one_month_str = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    ];

    let one_month_pomodoro_count: Vec<u64> = one_month
        .iter()
        .map(|date| {
            app.task_log
                .iter()
                .filter(|log| {
                    let date_each = Local
                        .datetime_from_str(&log.date, DATE_FORMAT)
                        .unwrap()
                        .date();

                    (date_each.year(), date_each.month(), date_each.day())
                        == (date.year(), date.month(), date.day())
                })
                .count() as u64
        })
        .collect();

    let one_month_data: Vec<(&str, u64)> = one_month_str
        .into_iter()
        .zip(one_month_pomodoro_count)
        .collect();

    let chart_monthly = BarChart::default()
        .block(block)
        .data(&one_month_data)
        .bar_width(3)
        .bar_style(Style::default().fg(Color::LightGreen))
        .bar_gap(1);
    f.render_widget(chart_monthly, area);
}

fn draw_chart_of_year<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("YEAR {}", Local::today().year()))
        .title_alignment(Alignment::Center);

    let one_year = vec![
        Month::January,
        Month::February,
        Month::March,
        Month::April,
        Month::May,
        Month::June,
        Month::July,
        Month::August,
        Month::September,
        Month::October,
        Month::November,
        Month::December,
    ];
    // TODO:ここどうにかしたい。。
    let one_year_str = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
    ];

    let one_month_pomodoro_count: Vec<u64> = one_year
        .iter()
        .map(|month| {
            app.task_log
                .iter()
                .filter(|log| {
                    let date_each = Local
                        .datetime_from_str(&log.date, DATE_FORMAT)
                        .unwrap()
                        .date();

                    (date_each.year(), date_each.month())
                        == (Local::today().year(), month.number_from_month())
                })
                .count() as u64
        })
        .collect();

    let one_year_data: Vec<(&str, u64)> = one_year_str
        .into_iter()
        .zip(one_month_pomodoro_count)
        .collect();

    let chart_yearly = BarChart::default()
        .block(block)
        .data(&one_year_data)
        .bar_width(3)
        .bar_style(Style::default().fg(Color::LightBlue))
        .bar_gap(1);
    f.render_widget(chart_yearly, area);
}
