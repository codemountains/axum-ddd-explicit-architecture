use crate::context::errors::AppError;
use crate::context::validate::ValidatedRequest;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::{BoxError, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedRequest<T>
    where
        T: DeserializeOwned + Validate,
        B: http_body::Body + Send,
        B::Data: Send,
        B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
