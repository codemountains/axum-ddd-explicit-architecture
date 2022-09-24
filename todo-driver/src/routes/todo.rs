use crate::model::todo::JsonTodo;
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
