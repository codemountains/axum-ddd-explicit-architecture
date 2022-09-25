use crate::model::DateTimeRfc3339;
use todo_kernel::model::todo::Todo;

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
        let completed_at = if let Some(c) = t.completed_at {
            Some(c.into())
        } else {
            None
        };

        Self {
            id: t.id.value.to_string(),
            title: t.title,
            description: t.description,
            is_completed: t.is_completed,
            created_at: t.created_at.into(),
            updated_at: t.updated_at.into(),
            completed_at,
        }
    }
}
