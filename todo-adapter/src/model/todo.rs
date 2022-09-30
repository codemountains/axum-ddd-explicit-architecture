pub mod status;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use todo_kernel::model::todo::status::TodoStatus;
use todo_kernel::model::todo::{NewTodo, Todo, UpdateTodo};

#[derive(FromRow, Debug)]
pub struct StoredTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status_id: String,
    pub status_code: String,
    pub status_name: String,
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
            status: TodoStatus::new(t.status_id.try_into()?, t.status_code, t.status_name),
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }
}

#[derive(FromRow, Debug)]
pub struct InsertTodo {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl From<NewTodo> for InsertTodo {
    fn from(nt: NewTodo) -> Self {
        InsertTodo {
            id: nt.id.value.to_string(),
            title: nt.title,
            description: nt.description,
        }
    }
}

pub struct UpdateStoredTodo {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_id: Option<String>,
}

impl From<UpdateTodo> for UpdateStoredTodo {
    fn from(ut: UpdateTodo) -> Self {
        let status_id = if let Some(status) = ut.status {
            Some(status.id.value.to_string())
        } else {
            None
        };

        UpdateStoredTodo {
            id: ut.id.value.to_string(),
            title: ut.title,
            description: ut.description,
            status_id,
        }
    }
}
