use crate::model::Id;

#[derive(Debug)]
pub struct TodoStatus {
    pub id: Id<TodoStatus>,
    pub code: String,
    pub name: String,
}

impl TodoStatus {
    pub fn new(id: Id<TodoStatus>, code: String, name: String) -> Self {
        Self { id, code, name }
    }
}
