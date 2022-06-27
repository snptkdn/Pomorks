use crate::notifications::send_notification;
use crate::statefull_list::StatefulList;
use crate::tui::UpdateInfo;
use anyhow::Result;
use chrono::prelude::*;
use pomorks_data_manage::todo::{TodoItem, TodoList};

#[cfg(debug_assertions)]
pub const ONE_MINUTE: usize = 1;
#[cfg(not(debug_assertions))]
pub const ONE_MINUTE: usize = 60;
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
            State::LUNCH(_) => State::WORK(1),
            State::BREAK(work_count) => State::WORK(*work_count + 1),
        }
    }

    pub fn get_prev_state(state_now: &Self) -> State {
        match state_now {
            State::WORK(work_count) if *work_count == 1 => State::LUNCH(4),
            State::WORK(work_count) => State::BREAK(*work_count - 1),
            State::LUNCH(_) => State::WORK(4),
            State::BREAK(work_count) => State::WORK(*work_count),
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
    pub start_time: Option<DateTime<Local>>,
    pub limit_time: usize,
    pub on_progress: bool,
    pub state: &'a State,
    pub enhanced_graphics: bool,
    pub todos: StatefulList<TodoItem>,
    pub todo_focus: Option<TodoItem>,
    pub new_todo_string: String,
    pub status: String,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        todo_list: &TodoList,
        state: &'a State,
        status: String,
    ) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_add_todo: false,
            show_chart: false,
            progress: 0.0,
            start_time: None,
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
            Ok(None)
        }
    }

    pub fn on_delete(&mut self) {
        if self.show_add_todo {
            self.new_todo_string.pop();
        }
    }

    pub fn on_next_state(&mut self) -> Result<Option<UpdateInfo>> {
        if !self.on_progress {
            Ok(Some(UpdateInfo::MoveNextState()))
        } else {
            Ok(None)
        }
    }

    pub fn on_prev_state(&mut self) -> Result<Option<UpdateInfo>> {
        if !self.on_progress {
            Ok(Some(UpdateInfo::MovePrevState()))
        } else {
            Ok(None)
        }
    }

    pub fn on_change_finish_flag(&mut self) -> Result<Option<UpdateInfo>> {
        match self.todos.state.selected() {
            Some(ind) => Ok(Some(UpdateInfo::ChangeFinishStatus(
                self.todos.items[ind].clone(),
                false,
            ))),
            None => Ok(None),
        }
    }

    pub fn on_key(&mut self, c: char, _: (u16, u16)) -> Result<Option<UpdateInfo>> {
        if self.show_add_todo {
            self.new_todo_string.push(c);
        } else {
            match c {
                'b' => {
                    return Ok(Some(UpdateInfo::ArchiveFinishedTodo(false)));
                }
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
                    return self.on_next_state();
                }
                'h' => {
                    return self.on_prev_state();
                }
                'f' => {
                    return self.on_change_finish_flag();
                }
                ' ' => {
                    self.on_progress = !self.on_progress;
                    if self.start_time == None {
                        self.start_time = Some(Local::now());
                    }
                }
                _ => {}
            }
        }

        Ok(None)
    }

    pub fn on_tick(&mut self) -> Option<UpdateInfo> {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        if let Some(start) = self.start_time {
            if (Local::now() - start).num_seconds() as usize >= self.limit_time {
                send_notification(self.state);
                self.start_time = None;
                self.on_progress = false;

                return match &self.todo_focus {
                    // TODO!:このCloneは微妙。Lifetime付けたいが、、、
                    Some(todo) => {
                        if let State::WORK(_) = self.state {
                            Some(UpdateInfo::CountIncrement(todo.clone(), true))
                        } else {
                            Some(UpdateInfo::MoveNextState())
                        }
                    }
                    None => Some(UpdateInfo::MoveNextState()),
                };
            }
        }
        None
    }
}
