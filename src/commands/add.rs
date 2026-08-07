use crate::data::Data;
use crate::task::Task;

pub fn execute(data: &mut Data, name: String, description: String, till: String) {
    let new_task = Task::new(&name, &description, &till, false);
    data.add_task(new_task);
}
