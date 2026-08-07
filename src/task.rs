use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    till: String,
    status: bool,
}

impl Task {
    pub fn new(name: &str, description: &str, till: &str, status: bool) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            till: till.to_string(),
            status: status,
        }
    }

    pub fn get_status(&self) -> bool {
        self.status
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

    pub fn change_status(&mut self, status: bool) {
        self.status = status;
    }
}
