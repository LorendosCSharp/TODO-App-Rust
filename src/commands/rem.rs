use crate::data::Data;

pub fn execute(data: &mut Data, id: String) {
    data.remove_task(id);
}
