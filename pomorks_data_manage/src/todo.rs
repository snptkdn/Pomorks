use anyhow::{anyhow, Result};
use rand::Rng;
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

    pub fn insert_todo(&mut self, todo: TodoItem) -> Result<()> {
        //TODO!:リザルト処理
        self.todo_list.insert(todo.id.clone(), todo);

        Ok(())
    }

    pub fn get_vec_of_todo(&self) -> Vec<TodoItem> {
        self.todo_list
            .iter()
            .map(|(_, todo)| todo.clone())
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub tag: String,
    pub project: String,
    pub estimate_count: usize,
    pub executed_count: usize,
    pub finished: bool,
    pub detail: String,
}

impl TodoItem {
    pub fn new(
        id: String,
        title: String,
        tag: String,
        project: String,
        estimate_count: usize,
        executed_count: usize,
        detail: String,
    ) -> Self {
        TodoItem {
            id,
            title,
            tag,
            project,
            estimate_count,
            executed_count,
            finished: false,
            detail,
        }
    }

    pub fn from_str(str: &String) -> Result<Self> {
        let spl: Vec<&str> = str.split(" ").collect();

        if spl.len() != 4 {
            return Err(anyhow!("Todo String Parse Error."));
        }

        let mut id: Vec<char> = vec![];
        for _num in 1..11 {
            let rand_num = rand::thread_rng().gen_range(97..123);
            if let Some(rand_num) = std::char::from_u32(rand_num) {
                id.push(rand_num);
            }
        }

        Ok(TodoItem {
            id: id.iter().collect(),
            title: spl[0].to_string(),
            tag: spl[1].to_string(),
            project: spl[2].to_string(),
            estimate_count: spl[3].to_string().parse()?,
            executed_count: 0,
            finished: false,
            detail: String::new(),
        })
    }
}
