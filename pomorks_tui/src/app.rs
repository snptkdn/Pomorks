use crate::notifications::send_notification;
use crate::statefull_list::StatefulList;
use crate::tui::UpdateInfo;
use anyhow::Result;
use chrono::prelude::*;
use pomorks_data_manage::data_manage_trait::TaskLogJson;
use pomorks_data_manage::todo::{State, TodoItem, TodoList};

pub enum Tab {
    Main,
    Statistics,
}

impl Tab {
    pub fn get_next_tab(&self) -> Self {
        match self {
            Tab::Main => Tab::Statistics,
            Tab::Statistics => Tab::Main,
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
    pub todays_executed_count: i64,
    pub selected_tab: Tab,
    pub task_log: &'a Vec<TaskLogJson>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        todo_list: &TodoList,
        state: &'a State,
        status: String,
        id: &Option<String>,
        start_time: &Option<DateTime<Local>>,
        todays_executed_count: i64,
        task_log: &'a Vec<TaskLogJson>,
    ) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_add_todo: false,
            show_chart: false,
            progress: 0.0,
            start_time: if let Some(start_time) = start_time {
                Some(*start_time)
            } else {
                None
            },
            limit_time: State::get_limit_time(state),
            on_progress: false,
            state,
            todos: StatefulList::with_items(todo_list.get_vec_of_todo()),
            enhanced_graphics,
            // TODO!:分かりにくすぎる...
            todo_focus: if let Some(id) = id {
                todo_list.get_vec_of_todo().iter().find_map(|todo| {
                    if &todo.id == id {
                        Some(todo.clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            },
            new_todo_string: String::new(),
            status,
            todays_executed_count,
            selected_tab: Tab::Main,
            task_log,
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

    pub fn on_change_tab(&mut self) {
        self.selected_tab = Tab::get_next_tab(&self.selected_tab);
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
                    if self.start_time.is_none() {
                        self.start_time = Some(Local::now());
                        return Ok(Some(UpdateInfo::StartTodo(
                            self.start_time.unwrap(),
                            if let Some(focus) = &self.todo_focus {
                                focus.id.to_string()
                            } else {
                                "".to_string()
                            },
                            self.state.clone(),
                        )));
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
            if (Local::now() - start).num_seconds() as i64 >= self.limit_time as i64 {
                send_notification(self.state).expect("can't send notification.");

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
