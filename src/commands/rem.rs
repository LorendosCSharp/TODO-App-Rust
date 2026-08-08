use todo::{api, data::Data};

pub fn execute(data: &mut Data, id: String) {
    match api::remove_task(data, id) {
        Ok(value) => {
            println!("{}", value);
        }
        Err(err_value) => {
            eprintln!("{}", err_value);
        }
    }
}
