use crate::data::Data;
use crate::task::Task;
use nanoid::nanoid;
use std::collections::HashMap;

pub fn add_task(data: &mut Data, task: Task) -> Result<(String, Task), String> {
    let mut id = nanoid!(8);
    //I could use other loops variants, but I want use this one. IDK why but it seems more interesting then the others.
    let mut tries = 0;
    loop {
        if tries >= 5 {
            return Err(
                "Error while generating new ID for a task. Couldn't generate new ID".to_string(),
            );
        }

        if data.tasks.contains_key(&id) {
            id = nanoid!(8);
            tries += 1;
        } else {
            break;
        }
    }
    data.tasks.insert(id.clone(), task.clone());
    Ok((id, task))
}

pub fn remove_task(data: &mut Data, id: String) -> Result<String, String> {
    if data.tasks.remove(&id).is_some() {
        Ok(format!("Removed task with ID {}", &id))
    } else {
        Err(format!(
            "Couldn't find the task with ID {} while removing a task",
            &id
        ))
    }
}

pub fn done_task(data: &mut Data, id: String) -> Result<(String, Task), String> {
    if let Some(task) = data.tasks.get_mut(&id) {
        task.change_status(true);
        Ok(("Task was done".to_string(), task.clone()))
    } else {
        Err(format!(
            "Couldn't find the task with ID {} while finishing a task",
            &id
        ))
    }
}

pub fn modify_task(
    data: &mut Data,
    id: String,
    name: Option<String>,
    description: Option<String>,
    till: Option<String>,
    status: Option<String>,
) -> Result<(String, Task), String> {
    if let Some(task) = data.tasks.get_mut(&id) {
        if let Some(new_name) = name {
            task.change_name(new_name);
        }
        if let Some(new_description) = description {
            task.change_description(new_description);
        }
        if let Some(new_till) = till {
            task.change_till(new_till);
        }
        if let Some(new_status) = status {
            task.change_status(if new_status == "done" { true } else { false });
        }
        return Ok((id, task.clone()));
    } else {
        return Err(format!("Couldn't find the task with ID {}", id));
    }
}

pub fn search_for_tasks(
    data: &Data,
    id: Option<String>,
    name: Option<String>,
    till: Option<String>,
    status: Option<String>,
) -> Result<HashMap<String, Task>, String> {
    let mut tasks_hash: HashMap<String, Task> = data.tasks.clone();

    //Not returning the new task hash, because the actual hash can contain one task with one id, and no multiple tasks for one id
    if let Some(searched_id) = id {
        if let Some(task) = data.tasks.get(&searched_id) {
            tasks_hash = HashMap::new();
            tasks_hash.insert(searched_id, task.clone());
            return Ok(tasks_hash);
        }
        return Err(format!(
            "Couldn't find the task with ID {} while finishing a task",
            searched_id
        ));
    }

    //Same thing as in list.rs, I won't abstract the code, I will use boilerplate
    if let Some(searched_name) = name {
        tasks_hash = find_suitable_task(data, |task| task.get_name() == &searched_name);
    }

    if let Some(searched_till) = till {
        let new_data = Data::new().tasks_hash(&tasks_hash);
        tasks_hash = find_suitable_task(&new_data, |task| task.get_till() == &searched_till);
    }
    if let Some(searched_status) = status {
        // Here I have sprinkled some magic, so if wrong status is provided it would skip the filter
        let searched_status_bool = match searched_status.as_str() {
            "done" | "true" => Some(true),
            "not done" | "false" => Some(false),
            _ => None,
        };

        if let Some(searched_status_bool) = searched_status_bool {
            let new_data = Data::new().tasks_hash(&tasks_hash);
            tasks_hash =
                find_suitable_task(&new_data, |task| task.get_status() == searched_status_bool);
        }
    }
    return Ok(tasks_hash);
}

// This is some mindblowing thing for me
pub fn find_suitable_task(data: &Data, condition: impl Fn(&Task) -> bool) -> HashMap<String, Task> {
    let mut new_tasks: HashMap<String, Task> = HashMap::new();
    for (id, task) in data.tasks.iter().filter(|(_, task)| condition(task)) {
        new_tasks.insert(id.clone(), task.clone());
    }
    return new_tasks;
}
