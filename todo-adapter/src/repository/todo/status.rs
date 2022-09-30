use crate::model::todo::status::StoredTodoStatus;
use crate::repository::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::query_as;
use todo_kernel::model::todo::status::TodoStatus;
use todo_kernel::repository::todo::status::TodoStatusRepository;

#[async_trait]
impl TodoStatusRepository for DatabaseRepositoryImpl<TodoStatus> {
    async fn get_by_code(&self, code: &str) -> anyhow::Result<TodoStatus> {
        let pool = self.db.0.clone();
        let sql = r#"
            select
                id,
                code,
                name
            from
                todo_statuses
            where
                code = $1
        "#;

        let stored_todo_status = query_as::<_, StoredTodoStatus>(sql)
            .bind(code.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_todo_status {
            Some(todo_status) => Ok(todo_status.try_into()?),
            None => Err(anyhow!("`statusCode` is invalid.")),
        }
    }
}
