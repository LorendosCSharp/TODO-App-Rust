use todo::{api, data::Data};

pub fn execute(
    data: &Data,
    id: Option<String>,
    name: Option<String>,
    till: Option<String>,
    status: Option<String>,
) {
    let result = api::search_for_tasks(data, id, name, till, status);
    match result {
        Ok(value) => {
            for (id, task) in value.iter() {
                print!("ID {} | ", id);
                task.print_task();
            }
        }
        Err(value) => {
            println!("{}", value);
        }
    }
}
