use crate::data_manage_trait::{DataManage, TaskLogJson};
use crate::todo::*;
use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
pub const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S%Z";

pub struct DataManageJson {}

#[derive(Serialize, Deserialize)]
struct TaskDealing {
    id: Option<String>,
    date: Option<DateTime<Local>>,
    state: Option<State>,
}

impl DataManage for DataManageJson {
    fn write_all_todo(todo_list: TodoList) -> Result<()> {
        let serialized = serde_json::to_string(&todo_list)?;

        // TODO!: ファイル名�?�引数�?�?
        let mut file = File::create("task.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }

    fn read_all_todo() -> Result<Option<TodoList>> {
        let todo_list_json = match File::open("task.json") {
            Ok(file) => file,
            Err(_) => File::create("task.json").context("can't create file.")?,
        };
        let todo_list: TodoList = match serde_json::from_reader(todo_list_json) {
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
        // TODO!: ファイル名�?�引数�?�?
        let mut file = File::create("archive.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }

    fn write_task_dealing(id: &String, start_time: &DateTime<Local>, state: &State) -> Result<()> {
        let task_dealing = TaskDealing {
            id: Some(id.to_string()),
            date: Some(*start_time),
            state: Some(state.clone()),
        };

        let serialized = serde_json::to_string(&task_dealing)?;

        let mut file = File::create("dealing_task.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }

    fn read_task_dealing() -> Result<(Option<DateTime<Local>>, Option<String>, Option<State>)> {
        let task_dealing_json = match File::open("dealing_task.json") {
            Ok(file) => file,
            Err(_) => File::create("dealing_task.json").context("can't create file.")?,
        };
        let task_dealing = match serde_json::from_reader(task_dealing_json) {
            Ok(task_dealing) => task_dealing,
            Err(_) => TaskDealing {
                id: None,
                date: None,
                state: None,
            },
        };

        Ok((task_dealing.date, task_dealing.id, task_dealing.state))
    }

    fn delete_task_dealing() -> Result<()> {
        fs::remove_file("dealing_task.json")?;
        Ok(())
    }

    fn add_task_log(id: &String, date: &DateTime<Local>) -> Result<()> {
        let task_log_json = match File::open("task_log.json") {
            Ok(file) => file,
            Err(_) => File::create("task_log.json").context("can't create file.")?,
        };
        let mut task_log: Vec<TaskLogJson> = match serde_json::from_reader(task_log_json) {
            Ok(task_log) => task_log,
            Err(_) => Vec::new(),
        };

        let date = date.format(DATE_FORMAT).to_string();

        task_log.push(TaskLogJson {
            id: id.to_string(),
            date,
        });

        let serialized = serde_json::to_string(&task_log)?;
        let mut file = File::create("task_log.json")?;
        write!(file, "{}", serialized)?;
        file.flush()?;

        Ok(())
    }

    fn get_executed_count_by_day(date: &DateTime<Local>) -> Result<i64> {
        let task_log_json = File::open("task_log.json")?;
        let task_log: Vec<TaskLogJson> = serde_json::from_reader(task_log_json)?;

        let count = task_log.iter().fold(0, |acc, log| {
            let date_each = match Local.datetime_from_str(&log.date, DATE_FORMAT) {
                Ok(res) => res,
                Err(_) => Local
                    .datetime_from_str("1800/02/02 00:00:00+09:00", DATE_FORMAT)
                    .unwrap(),
            };
            if date_each.day() == date.day() {
                acc + 1
            } else {
                acc
            }
        });

        Ok(count)
    }

    fn get_log_all() -> Result<Vec<TaskLogJson>> {
        let task_log_json = File::open("task_log.json")?;
        Ok(serde_json::from_reader(task_log_json)?)
    }
}
