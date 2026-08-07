use crate::data::Data;

pub fn execute(
    data: &mut Data,
    id: String,
    name: Option<String>,
    description: Option<String>,
    till: Option<String>,
    status: Option<String>,
) {
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
    } else {
        eprintln!("Couldn't find the task with ID {}", id);
    }
}
