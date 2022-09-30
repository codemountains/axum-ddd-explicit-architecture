use crate::model::todo::status::TodoStatus;
use async_trait::async_trait;

#[async_trait]
pub trait TodoStatusRepository {
    async fn get_by_code(&self, code: &str) -> anyhow::Result<TodoStatus>;
}
