use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    todo_list: HashMap<String, TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todo_list: HashMap::new(),
        }
    }

    pub fn add_todo(&mut self, todo: TodoItem) -> Result<()> {
        match self.todo_list.get(&todo.id) {
            Some(_) => Err(anyhow!("id is duplicated.")),
            None => {
                self.todo_list.entry(todo.id.clone()).or_insert(todo);
                return Ok(());
            }
        }
    }

    pub fn delete_todo(&mut self, todo: TodoItem) -> Result<()> {
        match self.todo_list.remove(&todo.id) {
            Some(x) => Ok(()),
            None => Err(anyhow!("selected todo is not exist.")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TodoItem {
    id: String,
    title: String,
    tag: String,
    project: String,
    estimate_count: usize,
    executed_count: usize,
    finished: bool,
}

impl TodoItem {
    pub fn new(
        id: String,
        title: String,
        tag: String,
        project: String,
        estimate_count: usize,
    ) -> Self {
        TodoItem {
            id,
            title,
            tag,
            project,
            estimate_count,
            executed_count: 0,
            finished: false,
        }
    }
}
