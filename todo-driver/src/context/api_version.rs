use crate::context::errors::AppError;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiVersion {
    V1,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts.extract().await.map_err(|err| err)?;

        let version = params
            .get("v")
            .ok_or_else(|| AppError::UnknownApiVerRejection("missing version param".to_string()))?;

        match version.as_str() {
            "v1" => Ok(ApiVersion::V1),
            _ => Err(AppError::UnknownApiVerRejection(version.to_string())),
        }
    }
}
