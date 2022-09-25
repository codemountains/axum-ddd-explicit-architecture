use crate::model::todo::{NewTodo, Todo};
use crate::model::Id;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository {
    async fn get(&self, id: &Id<Todo>) -> anyhow::Result<Option<Todo>>;
    async fn find(&self) -> anyhow::Result<Option<Vec<Todo>>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<Todo>;
}
