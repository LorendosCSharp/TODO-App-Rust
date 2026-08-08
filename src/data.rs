use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::task::Task;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub tasks: HashMap<String, Task>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            tasks: HashMap::new(),
        }
    }
    pub fn tasks_hash(mut self, task_hash: &HashMap<String, Task>) -> Self {
        self.tasks = task_hash.clone();
        self
    }

    pub fn print_tasks(&self) {
        for task_hash in self.tasks.iter() {
            print!("ID {} | ", task_hash.0);
            task_hash.1.print_task();
        }
    }

    pub fn load() -> Data {
        let content = fs::read_to_string("tasks.json");

        match content {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Data::new()),
            Err(_) => Data::new(),
        }
    }

    pub fn save(data: &Data) {
        let json = serde_json::to_string_pretty(data).unwrap();

        fs::write("tasks.json", json).unwrap();
    }
}
