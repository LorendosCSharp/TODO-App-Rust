use todo::{api, data::Data};

pub fn execute(data: &mut Data, id: String) {
    match api::done_task(data, id) {
        Ok(value) => {
            println!("Finished task");
            print!("ID {}", value.0);
            value.1.print_task();
        }
        Err(err_value) => {
            eprintln!("{}", err_value)
        }
    }
}
