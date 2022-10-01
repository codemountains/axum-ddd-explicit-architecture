pub mod status;

use crate::model::todo::status::TodoStatus;
use crate::model::Id;
use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: Id<Todo>,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewTodo {
    pub id: Id<Todo>,
    pub title: String,
    pub description: String,
}

impl NewTodo {
    pub fn new(id: Id<Todo>, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
        }
    }
}

pub struct UpdateTodo {
    pub id: Id<Todo>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
}

impl UpdateTodo {
    pub fn new(
        id: Id<Todo>,
        title: Option<String>,
        description: Option<String>,
        status: Option<TodoStatus>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }
}

pub struct UpsertTodo {
    pub id: Id<Todo>,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

impl UpsertTodo {
    pub fn new(id: Id<Todo>, title: String, description: String, status: TodoStatus) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }
}
