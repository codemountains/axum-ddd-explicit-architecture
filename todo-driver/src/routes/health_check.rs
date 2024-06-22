use crate::context::api_version::ApiVersion;
use crate::module::{Modules, ModulesExt};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::error;

pub async fn hc(_: ApiVersion) -> impl IntoResponse {
    tracing::debug!("Access health check endpoint.");
    StatusCode::NO_CONTENT
}

pub async fn hc_postgres(
    _: ApiVersion,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .health_check_use_case()
        .diagnose_db_conn()
        .await
        .map(|_| {
            tracing::debug!("Access postgres health check endpoint.");
            StatusCode::NO_CONTENT
        })
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}
