use serde::Serialize;
use todo_app::model::todo::TodoView;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TodoView> for JsonTodo {
    fn from(tv: TodoView) -> Self {
        Self {
            id: tv.id,
            title: tv.title,
            description: tv.description,
            created_at: tv.created_at.to_string(),
            updated_at: tv.updated_at.to_string(),
        }
    }
}
