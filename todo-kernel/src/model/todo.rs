use crate::model::Id;
use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: Id<Todo>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
