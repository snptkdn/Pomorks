use crate::app::App;
use crate::ui;
use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pomorks_data_manage::todo::TodoList;
use std::{
    error::Error,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

/// Crossterm demo
#[derive(Debug)]
struct Cli {
    /// time in ms between two ticks.
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    enhanced_graphics: bool,
}

pub fn launch_tui(todo_list: &mut TodoList) -> Result<()> {
    let cli: Cli = Cli {
        tick_rate: 5000,
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
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                match tx.send(Event::Tick) {
                    Err(e) => {
                        panic!("send error:{}", e);
                    }
                    _ => {}
                }
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new("Crossterm Demo", cli.enhanced_graphics, &todo_list);

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app, &todo_list))?;
        match rx.recv()? {
            Event::Input(event) => match event.modifiers {
                KeyModifiers::NONE => match event.code {
                    KeyCode::Char(c) => {
                        app.on_key(c, terminal.get_cursor().unwrap());
                    }
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Enter => app.on_enter_dir(),
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        return Ok(());
                    }
                    _ => {}
                },
                _ => match event {
                    _ => {}
                },
            },
            Event::Tick => {
                app.on_tick();
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

    Ok(())
}