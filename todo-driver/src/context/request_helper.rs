use crate::context::errors::AppError;
use crate::context::validate::ValidatedRequest;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::{async_trait, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
