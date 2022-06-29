use crate::todo::*;
use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskLogJson {
    pub id: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDealing {
    pub id: Option<String>,
    pub date: Option<DateTime<Local>>,
    pub state: Option<State>,
}

pub trait DataManage {
    fn write_all_todo(todo_list: TodoList) -> Result<()>;
    fn read_all_todo() -> Result<Option<TodoList>>;
    fn archive_todo(archived_todo_list: Vec<TodoItem>) -> Result<()>;
    fn write_task_dealing(id: &str, start_time: &DateTime<Local>, state: &State) -> Result<()>;
    fn read_task_dealing() -> Result<TaskDealing>;
    fn delete_task_dealing() -> Result<()>;
    fn add_task_log(id: &str, date: &DateTime<Local>) -> Result<()>;
    fn get_executed_count_by_day(date: &DateTime<Local>) -> Result<i64>;
    fn get_log_all() -> Result<Vec<TaskLogJson>>;
}
