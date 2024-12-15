mod to_do;

use to_do::enums::TaskStatus;
use to_do::ItemType;
use to_do::to_do_factory;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;


fn main() {
    println!("--- To DO List ---");

    let to_do_item = to_do_factory("Washing", TaskStatus::DONE);

    match to_do_item {
        ItemType::Pending(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        },
        ItemType::Done(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
    }
}
