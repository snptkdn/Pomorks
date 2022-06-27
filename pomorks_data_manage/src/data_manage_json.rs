use crate::data_manage_trait::DataManage;
use crate::todo::{self, *};
use anyhow::{Context, Result};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

pub struct DataManageJson {}

impl DataManage for DataManageJson {
    fn write_all_todo(todo_list: TodoList) -> Result<()> {
        let serialized = serde_json::to_string(&todo_list)?;

        // TODO!: ファイル名は引数指定
        let mut file = File::create("task.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }

    fn read_all_todo() -> Result<Option<todo::TodoList>> {
        let todo_list_json = match File::open("task.json") {
            Ok(file) => file,
            Err(_) => File::create("task.json").context("can't create file.")?,
        };
        let todo_list: todo::TodoList = match serde_json::from_reader(todo_list_json) {
            Ok(todo_list) => todo_list,
            Err(_) => TodoList::new(),
        };

        Ok(Some(todo_list))
    }

    fn archive_todo(mut archived_todo_list: Vec<TodoItem>) -> Result<()> {
        let current_archive_json = match File::open("archive.json") {
            Ok(file) => file,
            Err(_) => File::create("archive.json")?,
        };
        let current_archive = match serde_json::from_reader(current_archive_json) {
            Ok(archive) => archive,
            Err(_) => Vec::<TodoItem>::new(),
        };

        archived_todo_list.extend(current_archive);
        let serialized = serde_json::to_string(&archived_todo_list)?;
        // TODO!: ファイル名は引数指定
        let mut file = File::create("archive.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }
}
