use crate::model::todo::Todo;
use crate::model::Id;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository {
    async fn get(&self, id: &Id<Todo>) -> anyhow::Result<Option<Todo>>;
    async fn find(&self) -> anyhow::Result<Option<Vec<Todo>>>;
}
