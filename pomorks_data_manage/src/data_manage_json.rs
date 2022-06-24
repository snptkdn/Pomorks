use crate::data_manage_trait::DataManage;
use crate::todo::{self, *};
use anyhow::{Context, Result};
use std::env;
use std::fs::File;
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
}
