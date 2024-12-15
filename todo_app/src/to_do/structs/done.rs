use super::base::Base;
use crate::to_do::enums::TaskStatus;

// import traits
use crate::to_do::traits::create::Create;
use crate::to_do::traits::read::Read;
use crate::to_do::traits::update::Update;
use crate::to_do::traits::delete::Delete;

// define traits
impl Create for Done {}
impl Read for Done {}
impl Update for Done {}
impl Delete for Done {}

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base = Base{
            title: title.to_string(),
            status: TaskStatus::DONE
        };
        return Done{super_struct: base};
    }
}
