use crate::model::todo::{InsertTodo, StoredTodo};
use crate::repository::DatabaseRepositoryImpl;
use async_trait::async_trait;
use sqlx::{query, query_as};
use todo_kernel::model::todo::{NewTodo, Todo};
use todo_kernel::model::Id;
use todo_kernel::repository::todo::TodoRepository;

#[async_trait]
impl TodoRepository for DatabaseRepositoryImpl<Todo> {
    async fn get(&self, id: &Id<Todo>) -> anyhow::Result<Option<Todo>> {
        let pool = self.db.0.clone();
        let stored_todo = query_as::<_, StoredTodo>("select * from todos where id = $1")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_todo {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find(&self) -> anyhow::Result<Option<Vec<Todo>>> {
        let pool = self.db.0.clone();
        let stored_todo_list = query_as::<_, StoredTodo>("select * from todos")
            .fetch_all(&*pool)
            .await
            .ok();

        match stored_todo_list {
            Some(todo_list) => {
                let todos = todo_list.into_iter().flat_map(|st| st.try_into()).collect();
                Ok(Some(todos))
            }
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewTodo) -> anyhow::Result<Todo> {
        let pool = self.db.0.clone();
        let todo: InsertTodo = source.try_into()?;
        let id = todo.id.clone();

        let _ = query("insert into todos (id, title, description) values ($1, $2, $3)")
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.description)
            .execute(&*pool)
            .await?;

        let stored_todo = query_as::<_, StoredTodo>("select * from todos where id = $1")
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_todo.try_into()?)
    }
}
