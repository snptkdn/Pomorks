mod app;
mod date_manage;
mod notifications;
mod statefull_list;
mod tui;
mod ui;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use pomorks_data_manage::data_manage_firebase::DataManageFirebase;
use std::io;

use pomorks_data_manage::data_manage_json::DataManageJson;
use pomorks_data_manage::data_manage_trait::{DataManage, TypeDataManager};
use pomorks_data_manage::todo::{State, TodoItem, TodoList};

fn main() -> Result<()> {
    let selected_data_manager = input_selected_data_manager()?;
    let data_manager: &dyn DataManage = match selected_data_manager {
        TypeDataManager::DataManageJson => &DataManageJson {},
        TypeDataManager::DataManageFirebase => &DataManageFirebase {},
    };

    let mut todo_list = match data_manager.read_all_todo()? {
        Some(todo_list) => todo_list,
        None => TodoList::new(),
    };

    println!("{:?}", todo_list);

    let mut status = String::new();
    let mut task_dealing = data_manager.read_task_dealing()?;

    let mut state = State::WORK(1);
    if let Some(state_first) = task_dealing.state {
        state = state_first;
    }

    let mut todays_executed_count = data_manager.get_executed_count_by_day(&Local::now())?;
    let task_log = data_manager.get_log_all()?;

    loop {
        // TODO:start_timeの制御が各フローに散ってるの良くないが、、、?
        match tui::launch_tui(
            &mut todo_list,
            &state,
            &status,
            &task_dealing.id,
            &task_dealing.date,
            todays_executed_count,
            &task_log,
        ) {
            Ok(res) => match res {
                Some(info) => match info {
                    tui::UpdateInfo::CountIncrement(todo, is_go_next_state) => {
                        todo_list.insert_todo(TodoItem {
                            executed_count: todo.executed_count + 1,
                            ..todo.clone()
                        })?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                        task_dealing.date = None;
                        data_manager.add_task_log(&todo.id, &Local::now())?;
                        todays_executed_count =
                            data_manager.get_executed_count_by_day(&Local::now())?;
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
                        task_dealing.date = None;
                    }
                    tui::UpdateInfo::MovePrevState() => {
                        state = State::get_prev_state(&state);
                        task_dealing.date = None;
                    }
                    tui::UpdateInfo::ArchiveFinishedTodo(is_go_next_state) => {
                        let finished_todo = todo_list.drain_finished_todo();
                        data_manager.archive_todo(finished_todo)?;
                        if is_go_next_state {
                            state = State::get_next_state(&state);
                        }
                    }
                    tui::UpdateInfo::StartTodo(_start_time, _id, _state) => {
                        data_manager.write_task_dealing(&_id, &_start_time, &_state)?;
                        task_dealing.id = Some(_id.clone());
                        task_dealing.date = Some(_start_time);
                        state = _state;
                    }
                },
                None => break,
            },
            Err(e) => {
                status = e.to_string();
            }
        }
    }

    if task_dealing.date.is_none() {
        data_manager.delete_task_dealing()?;
    }
    data_manager.write_all_todo(todo_list)?;

    Ok(())
}

fn input_selected_data_manager() -> Result<TypeDataManager> {
    println!("Please Select DataManager");

    let vec_name_and_index = TypeDataManager::get_all_type_name_and_index();

    vec_name_and_index.iter().for_each(|(ind, name)| {
        println!("{}:{}", ind, name);
    });

    let mut result = String::new();

    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line.");

    let selected_name = &vec_name_and_index[result.trim().parse::<usize>()?];

    match TypeDataManager::from_name(&selected_name.1) {
        Some(type_manager) => Ok(type_manager),
        None => Err(anyhow!("input number is incorrect")),
    }
}
