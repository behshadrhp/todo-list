use super::base::Base;
use crate::to_do::enums::TaskStatus;

// import traits
use crate::to_do::traits::create::Create;
use crate::to_do::traits::read::Read;
use crate::to_do::traits::update::Update;

// define traits
impl Create for Pending {}
impl Read for Pending {}
impl Update for Pending {}

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base{
            title: title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base};
    }
}
