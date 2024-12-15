mod to_do;

use to_do::enums::TaskStatus;
use to_do::ItemType;
use to_do::to_do_factory;

// import traits
use crate::to_do::traits::read::Read;
use crate::to_do::traits::update::Update;
use crate::to_do::traits::delete::Delete;


fn main() {
    println!("--- To DO List ---");

    let to_do_item = to_do_factory("Washing", TaskStatus::DONE);

    match to_do_item {
        ItemType::Pending(item) => {
            item.read(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        },
        ItemType::Done(item) => {
            item.read(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
    }
}
