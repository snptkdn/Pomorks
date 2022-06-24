use crate::notifications::send_notification;
use crate::statefull_list::StatefulList;
use crate::tui::UpdateInfo;
use anyhow::Result;
use pomorks_data_manage::todo::{TodoItem, TodoList};

pub const ONE_MINUTE: usize = 1;
type WorkCount = usize;

#[derive(Clone)]
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
    pub show_add_todo: bool,
    pub show_chart: bool,
    pub progress: f64,
    pub time: usize,
    pub limit_time: usize,
    pub on_progress: bool,
    pub state: &'a State,
    pub enhanced_graphics: bool,
    pub todos: StatefulList<TodoItem>,
    pub todo_focus: Option<TodoItem>,
    pub new_todo_string: String,
    pub status: &'a String,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        todo_list: &TodoList,
        state: &'a State,
        status: &'a String,
    ) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_add_todo: false,
            show_chart: false,
            progress: 0.0,
            time: 0,
            limit_time: State::get_limit_time(state),
            on_progress: false,
            state,
            todos: StatefulList::with_items(todo_list.get_vec_of_todo()),
            enhanced_graphics,
            todo_focus: None,
            new_todo_string: String::new(),
            status,
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

    pub fn on_enter(&mut self) -> Result<Option<UpdateInfo>> {
        if self.show_add_todo {
            self.show_add_todo = false;

            Ok(Some(UpdateInfo::AddNewTodo(
                TodoItem::from_str(&self.new_todo_string)?,
                false,
            )))
        } else {
            self.todo_focus = match self.todos.state.selected() {
                Some(ind) => Some(self.todos.items[ind].clone()),
                None => None,
            };
            self.on_progress = true;
            Ok(None)
        }
    }

    pub fn on_delete(&mut self) {
        if self.show_add_todo {
            self.new_todo_string.pop();
        }
    }

    pub fn on_focus_left_pain(&mut self) {}

    pub fn on_focus_right_pain(&mut self) {}

    pub fn on_key(&mut self, c: char, _: (u16, u16)) {
        if self.show_add_todo {
            self.new_todo_string.push(c);
        } else {
            match c {
                'a' => {
                    self.show_add_todo = true;
                }
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
                    self.on_enter();
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
    }

    pub fn on_tick(&mut self) -> Option<UpdateInfo> {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
        if self.on_progress {
            self.time += 1;
        }
        if self.time >= self.limit_time {
            send_notification(self.state);
            self.time = 0;
            self.on_progress = false;

            return match &self.todo_focus {
                // TODO!:このCloneは微妙。Lifetime付けたいが、、、
                Some(todo) => Some(UpdateInfo::CountIncrement(todo.clone(), true)),
                None => None,
            };
        }

        None
    }
}
