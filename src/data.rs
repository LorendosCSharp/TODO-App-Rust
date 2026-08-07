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
    pub fn tasks_hash(mut self, task_hash: &HashMap<String, Task>) -> Self {
        self.tasks = task_hash.clone();
        self
    }

    pub fn add_task(&mut self, task: Task) {
        let mut id = nanoid!(8);
        let mut tries = 0;
        loop {
            if tries >= 5 {
                eprintln!("Couldn't generate id that is not in the list.");
                eprintln!("=== ABORTING ===");
                std::process::exit(1);
            }

            if self.tasks.contains_key(&id) {
                id = nanoid!(8);
                tries += 1;
            } else {
                break;
            }
        }
        self.tasks.insert(id.clone(), task);
        println!("Tasks ID: {}", id);
    }

    pub fn remove_task(&mut self, id: String) {
        if self.tasks.remove(&id).is_some() {
            println!("Removed task with ID {}", &id)
        } else {
            eprintln!("Couldn't find the task with ID {}", &id);
        }
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

    // This is some mindblowing thing for me
    pub fn find_suitable_task(
        data: &Data,
        condition: impl Fn(&Task) -> bool,
    ) -> HashMap<String, Task> {
        let mut new_tasks: HashMap<String, Task> = HashMap::new();
        for (id, task) in data.tasks.iter().filter(|(_, task)| condition(task)) {
            new_tasks.insert(id.clone(), task.clone());
        }
        return new_tasks;
    }
}
