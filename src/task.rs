use clap::builder::Str;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    till: String,
    status: bool,
}

impl Task {
    pub fn new(name: &str, description: &str, till: &str, status: bool) -> Task {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            till: till.to_string(),
            status: status,
        }
    }

    pub fn print_task(&self) {
        let output = format!(
            "{} | {} is due to {} and is {}",
            self.name,
            self.description,
            self.till,
            if self.status { "done" } else { "not done" }
        );
        println!("{}", output);
    }
    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn change_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn change_till(&mut self, till: String) {
        self.till = till;
    }
    pub fn change_status(&mut self, status: bool) {
        self.status = status;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn get_till(&self) -> &String {
        &self.till
    }
    pub fn get_status(&self) -> bool {
        self.status
    }
}
