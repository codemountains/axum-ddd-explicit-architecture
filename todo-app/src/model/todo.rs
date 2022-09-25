use crate::model::DateTimeRfc3339;
use todo_kernel::model::todo::{NewTodo, Todo};
use todo_kernel::model::Id;

pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTimeRfc3339,
    pub updated_at: DateTimeRfc3339,
    pub completed_at: Option<DateTimeRfc3339>,
}

impl From<Todo> for TodoView {
    fn from(t: Todo) -> Self {
        Self {
            id: t.id.value.to_string(),
            title: t.title,
            description: t.description,
            is_completed: t.is_completed,
            created_at: t.created_at.into(),
            updated_at: t.updated_at.into(),
            completed_at: t.completed_at.map(|c| c.into()),
        }
    }
}

pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

impl CreateTodo {
    pub fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}

impl TryFrom<CreateTodo> for NewTodo {
    type Error = anyhow::Error;

    fn try_from(ct: CreateTodo) -> Result<Self, Self::Error> {
        Ok(NewTodo::new(Id::gen(), ct.title, ct.description))
    }
}
