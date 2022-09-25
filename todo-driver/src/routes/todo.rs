use crate::model::todo::{JsonTodo, JsonTodoList};
use crate::module::{Modules, ModulesExt};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use tracing::log::{error, info};

pub async fn get_todo(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.todo_use_case().get_todo(id).await;

    match resp {
        Ok(tv) => tv
            .map(|tv| {
                info!("Found todo `{}`.", tv.id);
                let json: JsonTodo = tv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("Todo is not found.");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn find_todo(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.todo_use_case().find_todo().await;

    match resp {
        Ok(tv_list) => match tv_list {
            Some(tv) => {
                let todos = tv.into_iter().map(|t| t.into()).collect();
                let json = JsonTodoList::new(todos);
                Ok((StatusCode::OK, Json(json)))
            }
            None => {
                let json = JsonTodoList::new(vec![]);
                Ok((StatusCode::OK, Json(json)))
            }
        },
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
