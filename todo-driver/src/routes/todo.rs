use crate::context::api_version::ApiVersion;
use crate::context::response_helper::JsonErrorResponse;
use crate::context::validate::ValidatedRequest;
use crate::model::todo::{
    JsonCreateTodo, JsonTodo, JsonTodoList, JsonUpdateTodoContents, JsonUpsertTodoContents,
    TodoQuery,
};
use crate::module::{Modules, ModulesExt};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::log::{error, info};

pub async fn get_todo(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
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
    _: ApiVersion,
    Query(query): Query<TodoQuery>,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let resp = modules.todo_use_case().find_todo(query.into()).await;

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

            if err.to_string() == *"`statusCode` is invalid." {
                let json =
                    JsonErrorResponse::new("invalid_request".to_string(), vec![err.to_string()]);
                Err((StatusCode::BAD_REQUEST, Json(json)))
            } else {
                let json = JsonErrorResponse::new(
                    "server_error".to_string(),
                    vec!["INTERNAL SERVER ERROR".to_string()],
                );
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json)))
            }
        }
    }
}

pub async fn create_todo(
    _: ApiVersion,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonCreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.todo_use_case().register_todo(source.into()).await;

    resp.map(|tv| {
        info!("Created todo: {}", tv.id);
        let json: JsonTodo = tv.into();
        (StatusCode::CREATED, Json(json))
    })
    .map_err(|err| {
        error!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn update_todo(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonUpdateTodoContents>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match source.validate(id) {
        Ok(todo) => {
            let resp = modules.todo_use_case().update_todo(todo).await;

            resp.map(|tv| {
                info!("Updated todo {}", tv.id);
                let json: JsonTodo = tv.into();
                (StatusCode::OK, Json(json))
            })
            .map_err(|err| {
                error!("{:?}", err);

                if err.to_string() == *"`statusCode` is invalid." {
                    let json = JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        vec![err.to_string()],
                    );
                    (StatusCode::BAD_REQUEST, Json(json))
                } else {
                    let json = JsonErrorResponse::new(
                        "server_error".to_string(),
                        vec!["INTERNAL SERVER ERROR".to_string()],
                    );
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
                }
            })
        }
        Err(errors) => {
            let json = JsonErrorResponse::new("invalid_request".to_string(), errors);
            Err((StatusCode::BAD_REQUEST, Json(json)))
        }
    }
}

pub async fn upsert_todo(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonUpsertTodoContents>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let resp = modules
        .todo_use_case()
        .upsert_todo(source.to_view(id))
        .await;

    resp.map(|tv| {
        info!("Created or Updated todo {}", tv.id);
        let json: JsonTodo = tv.into();
        (StatusCode::OK, Json(json))
    })
    .map_err(|err| {
        error!("{:?}", err);

        if err.to_string() == *"`statusCode` is invalid." {
            let json = JsonErrorResponse::new("invalid_request".to_string(), vec![err.to_string()]);
            (StatusCode::BAD_REQUEST, Json(json))
        } else {
            let json = JsonErrorResponse::new(
                "server_error".to_string(),
                vec!["INTERNAL SERVER ERROR".to_string()],
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
        }
    })
}

pub async fn delete_todo(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.todo_use_case().delete_todo(id).await;

    match resp {
        Ok(tv) => tv
            .map(|tv| {
                info!("Deleted todo `{}`.", tv.id);
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
