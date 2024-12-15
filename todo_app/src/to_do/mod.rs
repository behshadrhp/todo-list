pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;


pub enum ItemType {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemType {
    match status {
        TaskStatus::DONE => {
            ItemType::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemType::Pending(Pending::new(title))
        }
    }
}
