mod app;
mod notifications;
mod statefull_list;
mod tui;
mod ui;
use anyhow::Result;
use core::panic;
use std::task;

use pomorks_data_manage::data_manage_json::{self, DataManageJson};
use pomorks_data_manage::data_manage_trait::DataManage;
use pomorks_data_manage::todo::{TodoItem, TodoList};

use crate::app::State;

fn main() -> Result<()> {
    let mut todo_list = match DataManageJson::read_all_todo()? {
        Some(todo_list) => todo_list,
        None => TodoList::new(),
    };

    println!("{:?}", todo_list);

    let mut state = app::State::WORK(1);
    let mut status = String::new();

    loop {
        match tui::launch_tui(&mut todo_list, &state, &status) {
            Ok(res) => match res {
                Some(info) => match info {
                    tui::UpdateInfo::CountIncrement(todo, is_go_next_state) => {
                        todo_list.insert_todo(TodoItem {
                            executed_count: todo.executed_count + 1,
                            ..todo
                        })?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                    }
                    tui::UpdateInfo::AddNewTodo(todo, is_go_next_state) => {
                        todo_list.add_todo(todo)?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                    }
                    tui::UpdateInfo::ChangeFinishStatus(todo, is_go_next_state) => {
                        todo_list.insert_todo(TodoItem {
                            finished: !todo.finished,
                            ..todo
                        })?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                    }
                    tui::UpdateInfo::MoveNextState() => {
                        state = State::get_next_state(&state);
                    }
                    tui::UpdateInfo::MovePrevState() => {
                        state = State::get_prev_state(&state);
                    }
                    tui::UpdateInfo::ArchiveFinishedTodo(is_go_next_state) => {
                        let finished_todo = todo_list.drain_finished_todo();
                        data_manage_json::DataManageJson::archive_todo(finished_todo)?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                    }
                },
                None => break,
            },
            Err(e) => {
                status = e.to_string();
            }
        }
    }

    DataManageJson::write_all_todo(todo_list)?;

    Ok(())
}
