use crate::data_manage_trait::DataManage;
use crate::todo::{self, *};
use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::{env, fs, vec};

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

    fn write_task_dealing(id: &String, start_time: &DateTime<Local>) -> Result<()> {
        let start_time = start_time.format("%Y/%m/%d %H:%M:%S%Z");

        // TODO!: ファイル名は引数指定
        let mut file = File::create("dealing_task.json")?;
        write!(file, "{},{}", id, start_time)?;
        file.flush()?;

        Ok(())
    }

    fn read_task_dealing() -> Result<(Option<DateTime<Local>>, Option<String>)> {
        let str = match fs::read_to_string("dealing_task.json") {
            Ok(res) => res,
            Err(e) => {
                return Ok((None, None));
            }
        };

        let vector: Vec<&str> = str.split(",").collect();
        if vector.len() != 2 {
            return Err(anyhow!("dealing_task.json is not collect."));
        }

        let id = vector[0];
        let start_time = Local.datetime_from_str(vector[1], "%Y/%m/%d %H:%M:%S%Z")?;

        Ok((Some(start_time), Some(id.to_string())))
    }
}
