use chrono::{DateTime, Utc};
use sqlx::FromRow;
use todo_kernel::model::todo::Todo;

#[derive(FromRow, Debug)]
pub struct StoredTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<StoredTodo> for Todo {
    type Error = anyhow::Error;

    fn try_from(t: StoredTodo) -> Result<Self, Self::Error> {
        Ok(Todo {
            id: t.id.try_into()?,
            title: t.title,
            description: t.description,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }
}
