use todo::{api, data::Data, task::Task};

pub fn execute(data: &mut Data, name: String, description: String, till: String) {
    let new_task = Task::new(&name, &description, &till, false);

    match api::add_task(data, new_task) {
        Ok(value) => {
            println!("Added new task");
            print!("ID {} |", value.0);
            value.1.print_task();
        }
        Err(err_value) => {
            eprintln!("{}", err_value);
        }
    }
}
