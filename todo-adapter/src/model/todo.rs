use chrono::{DateTime, Utc};
use sqlx::FromRow;
use todo_kernel::model::todo::Todo;

#[derive(FromRow, Debug)]
pub struct StoredTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl TryFrom<StoredTodo> for Todo {
    type Error = anyhow::Error;

    fn try_from(t: StoredTodo) -> Result<Self, Self::Error> {
        Ok(Todo {
            id: t.id.try_into()?,
            title: t.title,
            description: t.description,
            is_completed: t.is_completed,
            created_at: t.created_at,
            updated_at: t.updated_at,
            completed_at: t.completed_at,
        })
    }
}
