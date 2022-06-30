use crate::todo::*;
use anyhow::Result;
use chrono::prelude::*;
use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};

pub const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S%Z";
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum TypeDataManager {
    DataManageJson,
    DataManageFirebase,
}

impl TypeDataManager {
    fn name(self) -> String {
        match self {
            Self::DataManageJson => "Json".to_string(),
            Self::DataManageFirebase => "Firebase".to_string(),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        all::<TypeDataManager>()
            .collect::<Vec<TypeDataManager>>()
            .into_iter()
            .find(|type_manager| type_manager.name() == name)
    }

    pub fn get_all_type_name_and_index() -> Vec<(usize, String)> {
        let vec_type: Vec<TypeDataManager> = all::<TypeDataManager>().collect();
        vec_type
            .into_iter()
            .enumerate()
            .map(|(ind, type_manager)| (ind, type_manager.name().to_string()))
            .collect()
    }
}

pub trait DataManage {
    fn write_all_todo(&self, todo_list: TodoList) -> Result<()>;
    fn read_all_todo(&self) -> Result<Option<TodoList>>;
    fn archive_todo(&self, archived_todo_list: Vec<TodoItem>) -> Result<()>;
    fn write_task_dealing(
        &self,
        id: &str,
        start_time: &DateTime<Local>,
        state: &State,
    ) -> Result<()>;
    fn read_task_dealing(&self) -> Result<TaskDealing>;
    fn delete_task_dealing(&self) -> Result<()>;
    fn add_task_log(&self, id: &str, date: &DateTime<Local>) -> Result<()>;
    fn get_executed_count_by_day(&self, date: &DateTime<Local>) -> Result<i64>;
    fn get_log_all(&self) -> Result<Vec<TaskLogJson>>;
}
