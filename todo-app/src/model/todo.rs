pub mod status;

use crate::model::todo::status::TodoStatusView;
use crate::model::DateTimeRfc3339;
use todo_kernel::model::todo::{NewTodo, Todo};
use todo_kernel::model::Id;

pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TodoStatusView,
    pub created_at: DateTimeRfc3339,
    pub updated_at: DateTimeRfc3339,
}

impl From<Todo> for TodoView {
    fn from(t: Todo) -> Self {
        Self {
            id: t.id.value.to_string(),
            title: t.title,
            description: t.description,
            status: t.status.into(),
            created_at: t.created_at.into(),
            updated_at: t.updated_at.into(),
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

pub struct UpdateTodoView {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_code: Option<String>,
}

impl UpdateTodoView {
    pub fn new(
        id: String,
        title: Option<String>,
        description: Option<String>,
        status_code: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status_code,
        }
    }
}

pub struct SearchTodoCondition {
    pub status_code: Option<String>,
}
