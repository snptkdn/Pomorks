use crate::app::App;
use crate::ui;
use anyhow::Result;
use chrono::prelude::*;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pomorks_data_manage::data_manage_trait::TaskLogJson;
use pomorks_data_manage::todo::State;
use pomorks_data_manage::todo::TodoItem;
use pomorks_data_manage::todo::TodoList;
use std::{
    io::stdout,
    sync::mpsc::{self},
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

type ShouldGoNextState = bool;
pub enum UpdateInfo {
    CountIncrement(TodoItem, ShouldGoNextState),
    AddNewTodo(TodoItem, ShouldGoNextState),
    ChangeFinishStatus(TodoItem, ShouldGoNextState),
    ArchiveFinishedTodo(ShouldGoNextState),
    StartTodo(DateTime<Local>, String, State),
    MovePrevState(),
    MoveNextState(),
}

/// Crossterm demo
#[derive(Debug)]
struct Cli {
    /// time in ms between two ticks.
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    enhanced_graphics: bool,
}

pub fn launch_tui(
    todo_list: &mut TodoList,
    state: &State,
    status: &str,
    id: &Option<String>,
    start_time: &Option<DateTime<Local>>,
    todays_executed_count: i64,
    task_log: &Vec<TaskLogJson>,
) -> Result<Option<UpdateInfo>> {
    let cli: Cli = Cli {
        tick_rate: 1000,
        enhanced_graphics: true,
    };

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(cli.tick_rate);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    match tx.send(Event::Input(key)) {
                        Err(_) => break,
                        _ => (),
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                match tx.send(Event::Tick) {
                    Err(_) => break,
                    _ => {}
                }
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new(
        "Crossterm Demo",
        cli.enhanced_graphics,
        todo_list,
        state,
        status.to_owned(),
        id,
        start_time,
        todays_executed_count,
        task_log,
    );

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.modifiers {
                KeyModifiers::NONE => match event.code {
                    KeyCode::Char(c) => match app.on_key(c, terminal.get_cursor().unwrap())? {
                        Some(info) => return Ok(Some(info)),
                        None => (),
                    },
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Enter => {
                        let res = app.on_enter()?;
                        match res {
                            Some(info) => return Ok(Some(info)),
                            None => (),
                        }
                    }
                    KeyCode::Delete => app.on_delete(),
                    KeyCode::Backspace => app.on_delete(),
                    KeyCode::Tab => app.on_change_tab(),
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        return Ok(None);
                    }
                    _ => {}
                },
                _ => match event {
                    _ => {}
                },
            },
            Event::Tick => {
                if let Some(info) = app.on_tick() {
                    return Ok(Some(info));
                }
            }
        }
        if app.should_quit {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(None)
}
