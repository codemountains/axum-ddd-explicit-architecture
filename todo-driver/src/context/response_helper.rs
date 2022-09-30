use crate::context::errors::AppError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tracing::log::error;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorResponse {
    error_code: String,
    errors: Vec<String>,
}

impl JsonErrorResponse {
    pub(crate) fn new(error_code: String, errors: Vec<String>) -> Self {
        Self { error_code, errors }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(validation_errors) => {
                error!("{:?}", validation_errors);

                let mut messages: Vec<String> = Vec::new();
                let errors = validation_errors.field_errors();
                for (_, v) in errors.into_iter() {
                    for validation_error in v {
                        if let Some(msg) = validation_error.clone().message {
                            messages.push(msg.to_string());
                        }
                    }
                }

                (
                    StatusCode::BAD_REQUEST,
                    Json(JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        messages,
                    )),
                )
            }
            AppError::JsonRejection(rejection) => {
                error!("{:?}", rejection);

                let messages = vec![rejection.to_string()];
                (
                    StatusCode::BAD_REQUEST,
                    Json(JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        messages,
                    )),
                )
            }
        }
        .into_response()
    }
}
