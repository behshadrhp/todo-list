use super::base::Base;
use crate::to_do::enums::TaskStatus;

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
