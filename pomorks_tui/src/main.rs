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

    tui::launch_tui(&mut todo_list)?;

    Ok(())
}
