use serde::Serialize;
use todo_app::model::todo::TodoView;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

impl From<TodoView> for JsonTodo {
    fn from(tv: TodoView) -> Self {
        let completed_at = if let Some(c) = tv.completed_at {
            Some(c.to_string())
        } else {
            None
        };

        Self {
            id: tv.id,
            title: tv.title,
            description: tv.description,
            is_completed: tv.is_completed,
            created_at: tv.created_at.to_string(),
            updated_at: tv.updated_at.to_string(),
            completed_at,
        }
    }
}
