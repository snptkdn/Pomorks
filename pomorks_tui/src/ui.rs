use crate::app::App;
use std::fs::{read_to_string, read};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem,
        Paragraph, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
        draw_first_tab(f, app, chunks[1]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);
    draw_charts(f, app, chunks[0]);
    draw_text(f, chunks[1]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[0]);

        // Draw tasks
        //f.render_stateful_widget(tasks, chunks[0], &mut app.folders[0].state);

        //TODO:プレビュー表示
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::from("キー: "),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("e", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
            Span::raw("\": "),
            Span::from("システムの終了"),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("j", Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue)),
            Span::raw("\": "),
            Span::from("next"),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("k", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green)),
            Span::raw("\": "),
            Span::from("pre"),
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let version = env!("CARGO_PKG_VERSION");
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        format!("Me'nMa {}",version),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
