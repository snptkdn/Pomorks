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
        self.todo_list.insert(todo.id.clone(), todo);

        Ok(())
    }

    pub fn get_vec_of_todo(&self) -> Vec<TodoItem> {
        self.todo_list
            .iter()
            .map(|(_, todo)| todo.clone())
            .collect()
    }

    pub fn drain_finished_todo(&mut self) -> Vec<TodoItem> {
        self.todo_list
            .drain_filter(|_id, todo| todo.finished)
            .map(|(k, v)| v)
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

#[cfg(debug_assertions)]
pub const ONE_MINUTE: usize = 1;
#[cfg(not(debug_assertions))]
pub const ONE_MINUTE: usize = 60;
type WorkCount = usize;

#[derive(Clone, Serialize, Deserialize)]
pub enum State {
    WORK(WorkCount),
    BREAK(WorkCount),
    LUNCH(WorkCount),
}

impl State {
    pub fn get_next_state(state_now: &Self) -> State {
        match state_now {
            State::WORK(work_count) if *work_count == 4 => State::LUNCH(*work_count),
            State::WORK(work_count) => State::BREAK(*work_count),
            State::LUNCH(_) => State::WORK(1),
            State::BREAK(work_count) => State::WORK(*work_count + 1),
        }
    }

    pub fn get_prev_state(state_now: &Self) -> State {
        match state_now {
            State::WORK(work_count) if *work_count == 1 => State::LUNCH(4),
            State::WORK(work_count) => State::BREAK(*work_count - 1),
            State::LUNCH(_) => State::WORK(4),
            State::BREAK(work_count) => State::WORK(*work_count),
        }
    }

    pub fn get_state_name(state: &Self) -> String {
        match state {
            State::WORK(work_count) => format!("WORK_{}", work_count),
            State::BREAK(_) => format!("BREAK"),
            State::LUNCH(_) => format!("LUNCH"),
        }
    }

    pub fn get_limit_time(state: &Self) -> usize {
        match state {
            State::WORK(_) => 25 * ONE_MINUTE,
            State::BREAK(_) => 5 * ONE_MINUTE,
            State::LUNCH(_) => 30 * ONE_MINUTE,
        }
    }
}
