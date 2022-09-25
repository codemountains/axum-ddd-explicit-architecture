use crate::model::Id;
use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: Id<Todo>,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
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
