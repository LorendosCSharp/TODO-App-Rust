use crate::cli::sort::SortType;
use crate::data::Data;

pub fn execute(data: &Data, sort: Option<SortType>) {
    match sort {
        Some(SortType::All) | None => {
            data.print_tasks();
        }

        //I will not abstract it into a new function
        Some(SortType::Done) => {
            let tasks = Data::find_suitable_task(data, |task| task.get_status() == true);
            for (id, task) in tasks.iter() {
                print!("ID {} | ", id);
                task.print_task();
            }
        }

        Some(SortType::Undone) => {
            let tasks = Data::find_suitable_task(data, |task| task.get_status() == false);
            for (id, task) in tasks.iter() {
                print!("ID {} | ", id);
                task.print_task();
            }
        }
    }
}
