use crate::data_manage_trait::{DataManage, TaskDealing, TaskLogJson, DATE_FORMAT};
use crate::todo::*;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use firerust::FirebaseClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct FirebaseInfo {
    url: String,
    api_key: String,
}

impl FirebaseInfo {
    fn get_firebase_info() -> Result<FirebaseInfo> {
        let info_json = File::open("firebase_info.json")?;
        Ok(serde_json::from_reader(info_json)?)
    }

    pub fn get_client() -> Result<FirebaseClient> {
        let info = Self::get_firebase_info()?;

        let mut client = FirebaseClient::new(info.url).expect("Firebase connection is failure");
        client.auth(info.api_key);

        Ok(client)
    }
}
pub struct DataManageFirebase {}

impl DataManage for DataManageFirebase {
    fn write_all_todo(&self, todo_list: TodoList) -> Result<()> {
        let client = FirebaseInfo::get_client()?;

        let serialized = serde_json::to_value(&todo_list)?;
        client
            .reference("/todo_list")
            .set(serialized)
            .expect("can't update todo_list where firebase");

        Ok(())
    }

    fn read_all_todo(&self) -> Result<Option<TodoList>> {
        println!("read_todo");
        let client = FirebaseInfo::get_client()?;

        let todo_list_json: Value = client
            .reference("/todo_list")
            .get()
            .expect("can't get todo_list from firebase");

        let todo_list: TodoList = match serde_json::from_value(todo_list_json) {
            Ok(todo_list) => todo_list,
            Err(_) => TodoList::new(),
        };

        Ok(Some(todo_list))
    }

    fn archive_todo(&self, archived_todo_list: Vec<TodoItem>) -> Result<()> {
        println!("archive");
        let client = FirebaseInfo::get_client()?;

        let serialized = serde_json::to_value(&archived_todo_list)?;
        match client.reference("/archive").update(&serialized) {
            Ok(_) => (),
            Err(_) => client
                .reference("/archive")
                .set(serialized)
                .expect("can't update archive"),
        }

        Ok(())
    }

    fn write_task_dealing(
        &self,
        id: &str,
        start_time: &DateTime<Local>,
        state: &State,
    ) -> Result<()> {
        println!("write_deal");
        let client = FirebaseInfo::get_client()?;

        let task_dealing = TaskDealing {
            id: Some(id.to_string()),
            date: Some(*start_time),
            state: Some(state.clone()),
        };

        let serialized = serde_json::to_value(&task_dealing)?;

        client
            .reference("/task_dealing")
            .set(serialized)
            .expect("can't update task dealing where firebase");

        Ok(())
    }

    fn read_task_dealing(&self) -> Result<TaskDealing> {
        println!("read_dealong");
        let client = FirebaseInfo::get_client()?;

        let task_dealing_json: Value = client
            .reference("/task_dealing")
            .get()
            .expect("can't get task_dealing from firebase");

        let task_dealing = match serde_json::from_value(task_dealing_json) {
            Ok(task_dealing) => task_dealing,
            Err(_) => TaskDealing {
                id: None,
                date: None,
                state: None,
            },
        };

        Ok(task_dealing)
    }

    fn delete_task_dealing(&self) -> Result<()> {
        println!("delete_log");
        let client = FirebaseInfo::get_client()?;

        let task_dealing = TaskDealing {
            id: None,
            date: None,
            state: None,
        };

        let serialized = serde_json::to_value(&task_dealing)?;

        client
            .reference("/task_dealing")
            .set(serialized)
            .expect("can't update task dealing where firebase");

        Ok(())
    }

    fn add_task_log(&self, id: &str, date: &DateTime<Local>) -> Result<()> {
        println!("add_log");
        let client = FirebaseInfo::get_client()?;
        let date = date.format(DATE_FORMAT).to_string();

        let serialized = serde_json::to_value(&TaskLogJson {
            id: id.to_string(),
            date,
        })?;

        client
            .reference("/task_log")
            .update(serialized)
            .expect("can't update task dealing where firebase");

        Ok(())
    }

    fn get_executed_count_by_day(&self, date: &DateTime<Local>) -> Result<i64> {
        println!("get_executed");
        let client = FirebaseInfo::get_client()?;

        let task_log_json: Value = client
            .reference("/task_log")
            .get()
            .expect("can't get task_log from firebase");

        let task_log: Vec<TaskLogJson> = match serde_json::from_value(task_log_json) {
            Ok(task_log) => task_log,
            // TODO:名前おかしい
            Err(_) => vec![],
        };

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

    fn get_log_all(&self) -> Result<Vec<TaskLogJson>> {
        println!("get_log_all");
        let client = FirebaseInfo::get_client()?;
        let task_log_json: Value = client
            .reference("/task_log")
            .get()
            .expect("can't get task_log from firebase");

        match serde_json::from_value(task_log_json) {
            Ok(task_log) => Ok(task_log),
            // TODO:名前おかしい
            Err(e) => Ok(vec![]),
        }
    }
}
