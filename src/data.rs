use nanoid::nanoid;
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

    pub fn add_task(&mut self, task: Task) {
        let id = nanoid!(8);
        self.tasks.insert(id.clone(), task);
        println!("Tasks ID: {}", id);
    }

    pub fn remove_task(&mut self, id: String) {
        self.tasks.remove(&id);
        println!("Removed task with ID {}", &id)
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
