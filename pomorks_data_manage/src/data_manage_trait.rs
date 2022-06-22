use crate::todo::*;
use anyhow::Result;

pub trait DataManage {
    fn write_all_todo(todo_list: TodoList) -> Result<()>;
    fn read_all_todo() -> Result<Option<Vec<TodoItem>>>;
}
