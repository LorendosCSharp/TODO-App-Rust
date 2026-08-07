use std::collections::HashMap;

use crate::{data::Data, task::Task};

pub fn execute(
    data: &Data,
    id: Option<String>,
    name: Option<String>,
    till: Option<String>,
    status: Option<String>,
) {
    let mut tasks_hash: HashMap<String, Task> = data.tasks.clone();

    //Not returning the new task hash, because the actual hash can contain one task with one id, and no multiple tasks for one id
    if let Some(searched_id) = id {
        if let Some(task) = data.tasks.get(&searched_id) {
            print!("ID {} | ", &searched_id);
            task.print_task();
            return;
        }
    }

    //Same thing as in list.rs, I won't abstract the code, I will use boilerplate
    if let Some(searched_name) = name {
        tasks_hash = Data::find_suitable_task(data, |task| task.get_name() == &searched_name);
    }

    if let Some(searched_till) = till {
        let new_data = Data::new().tasks_hash(&tasks_hash);
        tasks_hash = Data::find_suitable_task(&new_data, |task| task.get_till() == &searched_till);
    }
    if let Some(searched_status) = status {
        let searched_status_bool = if searched_status == "done" || searched_status == "true" {
            true
        } else if searched_status == "not done" || searched_status == "false" {
            false
        } else {
            eprintln!("You've entered wrong status");
            eprintln!("=== ABORTING ===");
            std::process::exit(2);
        };

        let new_data = Data::new().tasks_hash(&tasks_hash);
        tasks_hash =
            Data::find_suitable_task(&new_data, |task| task.get_status() == searched_status_bool);
    }

    for (id, task) in tasks_hash.iter() {
        print!("ID {} | ", id);
        task.print_task();
    }
}
