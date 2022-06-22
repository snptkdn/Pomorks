use crate::statefull_list::StatefulList;
use anyhow::Result;
use pomorks_data_manage::todo::{TodoItem, TodoList};

pub const ONE_MINUTE: usize = 60;
type WorkCount = usize;

pub enum State {
    WORK(WorkCount),
    BREAK(WorkCount),
    LUNCH(WorkCount),
}

impl State {
    pub fn get_next_state(state_now: &Self) -> State {
        match state_now {
            State::WORK(work_count) if *work_count == 4 => State::LUNCH(*work_count),
            State::WORK(work_count) => State::BREAK(*work_count),
            State::LUNCH(_) => State::WORK(0),
            State::BREAK(work_count) => State::WORK(*work_count + 1),
        }
    }

    pub fn get_state_name(state: &Self) -> String {
        match state {
            State::WORK(work_count) => format!("WORK_{}", work_count),
            State::BREAK(_) => format!("BREAK"),
            State::LUNCH(_) => format!("LUNCH"),
        }
    }

    pub fn get_limit_time(state: &Self) -> usize {
        match state {
            State::WORK(_) => 25 * ONE_MINUTE,
            State::BREAK(_) => 5 * ONE_MINUTE,
            State::LUNCH(_) => 30 * ONE_MINUTE,
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub show_chart: bool,
    pub progress: f64,
    pub time: usize,
    pub limit_time: usize,
    pub on_progress: bool,
    pub state: State,
    pub enhanced_graphics: bool,
    pub todos: StatefulList<TodoItem>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, todo_list: &TodoList) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_chart: false,
            progress: 0.0,
            time: 0,
            limit_time: State::get_limit_time(&State::WORK(1)),
            on_progress: true,
            state: State::WORK(1),
            todos: StatefulList::with_items(todo_list.get_vec_of_todo()),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.todos.previous();
    }

    pub fn on_down(&mut self) {
        self.todos.next();
    }

    pub fn on_right(&mut self) {
        //self.tabs.next();
    }

    pub fn on_left(&mut self) {
        //self.tabs.previous();
    }

    pub fn on_enter_dir(&mut self) {
        //match self.folders[self.folders_index].state.selected() {
        //_ => {}
        //}
    }

    pub fn on_focus_left_pain(&mut self) {}

    pub fn on_focus_right_pain(&mut self) {}

    pub fn on_key(&mut self, c: char, _: (u16, u16)) {
        match c {
            'e' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'c' => {
                self.on_enter_dir();
            }
            'l' => {
                self.on_focus_right_pain();
            }
            'h' => {
                self.on_focus_left_pain();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
        if self.on_progress {
            self.time += 1;
        }
        if self.time >= self.limit_time {
            self.time = 0;
            self.on_progress = false;
            self.state = State::get_next_state(&self.state);
            self.limit_time = State::get_limit_time(&self.state);
        }
    }
}
