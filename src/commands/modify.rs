use todo::{api, data::Data};

pub fn execute(
    data: &mut Data,
    id: String,
    name: Option<String>,
    description: Option<String>,
    till: Option<String>,
    status: Option<String>,
) {
    match api::modify_task(data, id, name, description, till, status) {
        Ok(value) => {
            println!("Modified a task with success");
            print!("ID {} | ", value.0);
            value.1.print_task();
        }
        Err(err_value) => {
            eprintln!("{}", err_value);
        }
    }
}
