use crate::data_manage_trait::{DataManage, TaskDealing, TaskLogJson};
use crate::todo::*;
use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
pub const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S%Z";

pub struct DataManageFirebase {}

impl DataManage for DataManageFirebase {
    fn write_all_todo(&self, todo_list: TodoList) -> Result<()> {
        todo!()
    }

    fn read_all_todo(&self) -> Result<Option<TodoList>> {
        todo!()
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
