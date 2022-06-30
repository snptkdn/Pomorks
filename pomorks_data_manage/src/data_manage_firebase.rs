use crate::data_manage_trait::{DataManage, TaskDealing, TaskLogJson};
use crate::todo::*;
use anyhow::Result;
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

        let serialized = serde_json::to_string(&todo_list)?;
        client
            .reference("/todo_list")
            .set(serialized)
            .expect("can't update todo_list where firebase");

        Ok(())
    }

    fn read_all_todo(&self) -> Result<Option<TodoList>> {
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

    fn archive_todo(&self, mut archived_todo_list: Vec<TodoItem>) -> Result<()> {
        todo!()
    }

    fn write_task_dealing(
        &self,
        id: &str,
        start_time: &DateTime<Local>,
        state: &State,
    ) -> Result<()> {
        todo!()
    }

    fn read_task_dealing(&self) -> Result<TaskDealing> {
        todo!()
    }

    fn delete_task_dealing(&self) -> Result<()> {
        todo!()
    }

    fn add_task_log(&self, id: &str, date: &DateTime<Local>) -> Result<()> {
        todo!()
    }

    fn get_executed_count_by_day(&self, date: &DateTime<Local>) -> Result<i64> {
        todo!()
    }

    fn get_log_all(&self) -> Result<Vec<TaskLogJson>> {
        todo!()
    }
}
