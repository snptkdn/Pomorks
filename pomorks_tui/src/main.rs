mod app;
mod statefull_list;
mod tui;
mod ui;
use anyhow::Result;
use core::panic;
use std::task;

use pomorks_data_manage::data_manage_json::DataManageJson;
use pomorks_data_manage::data_manage_trait::DataManage;
use pomorks_data_manage::todo::{TodoItem, TodoList};

fn main() -> Result<()> {
    let mut todo_list = match DataManageJson::read_all_todo()? {
        Some(todo_list) => todo_list,
        None => TodoList::new(),
    };

    println!("{:?}", todo_list);

    loop {
        match tui::launch_tui(&mut todo_list)? {
            Some(info) => match info {
                tui::UpdateInfo::CountIncrement(todo) => {
                    todo_list.insert_todo(TodoItem {
                        executed_count: todo.executed_count + 1,
                        ..todo
                    })?;
                }
            },
            None => break,
        }
    }

    Ok(())
}
