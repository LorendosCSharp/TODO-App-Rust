use crate::cli::sort::SortType;
use crate::data::Data;

pub fn execute(data: &Data, sort: Option<SortType>) {
    match sort {
        Some(SortType::All) | None => {
            data.print_tasks();
        }

        Some(SortType::Done) => {
            find_suitable_tasks(data, true);
        }

        Some(SortType::Undone) => {
            find_suitable_tasks(data, false);
        }
    }
}

fn find_suitable_tasks(data: &Data, status: bool) {
    for (id, task) in data
        .tasks
        .iter()
        .filter(|(_, task)| task.get_status() == status)
    {
        print!("ID {} | ", id);
        task.print_task();
    }
}
