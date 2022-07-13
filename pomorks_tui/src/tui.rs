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

pub enum UpdateInfo {
    CountIncrement(TodoItem),
    AddNewTodo(TodoItem),
    ChangeFinishStatus(TodoItem),
    ArchiveFinishedTodo(),
    StartTodo(DateTime<Local>, String, State),
    MovePrevState(),
    MoveNextState(),
}

impl UpdateInfo {
    pub fn should_go_next_state(info: &Self) -> bool {
        match info {
            Self::CountIncrement(_) => true,
            Self::AddNewTodo(_) => true,
            Self::ChangeFinishStatus(_) => true,
            Self::ArchiveFinishedTodo() => true,
            Self::MoveNextState() => true,

            Self::StartTodo(_, _, _) => false,
            Self::MovePrevState() => false,
        }
    }

    pub fn should_delete_time_stamp_of_task_start(info: &Self) -> bool {
        match info {
            Self::CountIncrement(_) => true,
            Self::MoveNextState() => true,
            Self::MovePrevState() => true,

            Self::StartTodo(_, _, _) => false,
            Self::AddNewTodo(_) => false,
            Self::ChangeFinishStatus(_) => false,
            Self::ArchiveFinishedTodo() => false,
        }
    }
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
                    if tx.send(Event::Input(key)).is_err() {
                        break;
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if tx.send(Event::Tick).is_err() {
                    break;
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
            Event::Input(event) => {
                if event.modifiers == KeyModifiers::NONE {
                    match event.code {
                        KeyCode::Char(c) => {
                            if let Some(info) = app.on_key(c, terminal.get_cursor().unwrap())? {
                                return Ok(Some(info));
                            }
                        }
                        KeyCode::Left => app.on_left(),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Enter => {
                            let res = app.on_enter()?;
                            if let Some(info) = res {
                                return Ok(Some(info));
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
                    }
                }
            }
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
