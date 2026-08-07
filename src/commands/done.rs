use crate::data::Data;

pub fn execute(data: &mut Data, id: String) {
    if let Some(task) = data.tasks.get_mut(&id) {
        task.change_status(true);
        task.print_task();
    }
}
