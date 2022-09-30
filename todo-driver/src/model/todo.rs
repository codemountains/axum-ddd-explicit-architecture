pub mod status;

use crate::model::todo::status::JsonTodoStatus;
use serde::{Deserialize, Serialize};
use todo_app::model::todo::{CreateTodo, TodoView, UpdateTodoView};
use validator::Validate;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTodo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: JsonTodoStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TodoView> for JsonTodo {
    fn from(tv: TodoView) -> Self {
        Self {
            id: tv.id,
            title: tv.title,
            description: tv.description,
            status: tv.status.into(),
            created_at: tv.created_at.to_string(),
            updated_at: tv.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTodoList {
    pub todos: Vec<JsonTodo>,
}

impl JsonTodoList {
    pub fn new(todos: Vec<JsonTodo>) -> Self {
        Self { todos }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreateTodo {
    #[validate(
        length(min = 1, message = "`title` is empty."),
        required(message = "`title` is null.")
    )]
    pub title: Option<String>,
    #[validate(required(message = "`description` is null."))]
    pub description: Option<String>,
}

impl From<JsonCreateTodo> for CreateTodo {
    fn from(jc: JsonCreateTodo) -> Self {
        CreateTodo {
            title: jc.title.unwrap(),
            description: jc.description.unwrap(),
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_code: Option<String>,
}

impl JsonUpdateTodo {
    pub fn validate(self, id: String) -> Result<UpdateTodoView, Vec<String>> {
        let mut errors: Vec<String> = vec![];

        if let Some(title) = &self.title {
            if title.is_empty() {
                errors.push("`title` is empty.".to_string());
            }
        }

        if let Some(status_code) = &self.status_code {
            if status_code.is_empty() {
                errors.push("`statusCode` is empty.".to_string());
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(UpdateTodoView::new(
            id,
            self.title,
            self.description,
            self.status_code,
        ))
    }
}
