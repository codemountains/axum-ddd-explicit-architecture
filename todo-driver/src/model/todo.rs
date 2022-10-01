pub mod status;

use crate::model::todo::status::JsonTodoStatus;
use serde::{Deserialize, Serialize};
use todo_app::model::todo::{
    CreateTodo, SearchTodoCondition, TodoView, UpdateTodoView, UpsertTodoView,
};
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
pub struct JsonUpdateTodoContents {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_code: Option<String>,
}

impl JsonUpdateTodoContents {
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

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUpsertTodoContents {
    #[validate(
        length(min = 1, message = "`title` is empty."),
        required(message = "`title` is null.")
    )]
    pub title: Option<String>,
    #[validate(required(message = "`description` is null."))]
    pub description: Option<String>,
    #[validate(
        length(min = 1, message = "`statusCode` is empty."),
        required(message = "`statusCode` is null.")
    )]
    pub status_code: Option<String>,
}

impl JsonUpsertTodoContents {
    pub fn to_view(self, id: String) -> UpsertTodoView {
        UpsertTodoView::new(
            id,
            self.title.unwrap(),
            self.description.unwrap(),
            self.status_code.unwrap(),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoQuery {
    pub status: Option<String>,
}

impl From<TodoQuery> for SearchTodoCondition {
    fn from(tq: TodoQuery) -> Self {
        Self {
            status_code: tq.status,
        }
    }
}
