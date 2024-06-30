use crate::shared::setup_state_modules;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use todo_driver::context::api_version::ApiVersion;
use todo_driver::routes::health_check::{hc, hc_postgres};

mod shared;

#[tokio::test]
async fn test_hc() {
    let resp = hc(ApiVersion::V1).await;
    assert_eq!(resp.into_response().status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_hc_postgres() {
    let modules = setup_state_modules().await;

    match hc_postgres(ApiVersion::V1, modules).await {
        Ok(resp) => {
            assert_eq!(resp.into_response().status(), StatusCode::NO_CONTENT)
        }
        Err(_) => {
            assert!(false);
        }
    }
}
