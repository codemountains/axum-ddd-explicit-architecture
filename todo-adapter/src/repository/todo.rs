pub mod status;

use crate::model::todo::{InsertTodo, StoredTodo, UpdateStoredTodo};
use crate::repository::DatabaseRepositoryImpl;
use async_trait::async_trait;
use sqlx::{query, query_as};
use todo_kernel::model::todo::status::TodoStatus;
use todo_kernel::model::todo::{NewTodo, Todo, UpdateTodo};
use todo_kernel::model::Id;
use todo_kernel::repository::todo::TodoRepository;

#[async_trait]
impl TodoRepository for DatabaseRepositoryImpl<Todo> {
    async fn get(&self, id: &Id<Todo>) -> anyhow::Result<Option<Todo>> {
        let pool = self.db.0.clone();
        let sql = r#"
            select
                t.id as id,
                t.title as title,
                t.description as description,
                ts.id as status_id,
                ts.code as status_code,
                ts.name as status_name,
                t.created_at as created_at,
                t.updated_at as updated_at
            from
                todos as t
                inner join
                    todo_statuses as ts
                    on ts.id = t.status_id
            where
                t.id = $1
        "#;
        let stored_todo = query_as::<_, StoredTodo>(sql)
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_todo {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find(&self, status: Option<TodoStatus>) -> anyhow::Result<Option<Vec<Todo>>> {
        let pool = self.db.0.clone();
        let where_status = if let Some(s) = &status {
            s.id.value.to_string()
        } else {
            "".to_string()
        };

        let mut sql = r#"
            select
                t.id as id,
                t.title as title,
                t.description as description,
                ts.id as status_id,
                ts.code as status_code,
                ts.name as status_name,
                t.created_at as created_at,
                t.updated_at as updated_at
            from
                todos as t
                inner join
                    todo_statuses as ts
                    on ts.id = t.status_id
            where t.status_id in ($1)
        "#
        .to_string();

        if status.is_none() {
            sql = sql.replace("$1", "select id from todo_statuses");
        }

        let stored_todo_list = query_as::<_, StoredTodo>(&sql)
            .bind(where_status)
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
        let todo: InsertTodo = source.into();
        let id = todo.id.clone();

        let _ = query("insert into todos (id, title, description) values ($1, $2, $3)")
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.description)
            .execute(&*pool)
            .await?;

        let sql = r#"
            select
                t.id as id,
                t.title as title,
                t.description as description,
                ts.id as status_id,
                ts.code as status_code,
                ts.name as status_name,
                t.created_at as created_at,
                t.updated_at as updated_at
            from
                todos as t
                inner join
                    todo_statuses as ts
                    on ts.id = t.status_id
            where
                t.id = $1
        "#;

        let stored_todo = query_as::<_, StoredTodo>(sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_todo.try_into()?)
    }

    async fn update(&self, source: UpdateTodo) -> anyhow::Result<Todo> {
        let pool = self.db.0.clone();
        let todo: UpdateStoredTodo = source.into();
        let id = todo.id.clone();

        let update_sql = r#"
            update
                todos as target
            set
                title = case when $2 is not null then $2 else current_todo.title end,
                description = case when $3 is not null then $3 else current_todo.description end,
                status_id = case when $4 is not null then $4 else current_todo.status_id end,
                updated_at = current_timestamp
            from
                (select * from todos where id = $1) as current_todo
            where
                target.id = $1
        "#;

        let _ = query(update_sql)
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.description)
            .bind(todo.status_id)
            .execute(&*pool)
            .await?;

        let sql = r#"
            select
                t.id as id,
                t.title as title,
                t.description as description,
                ts.id as status_id,
                ts.code as status_code,
                ts.name as status_name,
                t.created_at as created_at,
                t.updated_at as updated_at
            from
                todos as t
                inner join
                    todo_statuses as ts
                    on ts.id = t.status_id
            where
                t.id = $1
        "#;

        let stored_todo = query_as::<_, StoredTodo>(sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_todo.try_into()?)
    }

    async fn delete(&self, id: &Id<Todo>) -> anyhow::Result<Option<Todo>> {
        let pool = self.db.0.clone();

        let sql = r#"
            select
                t.id as id,
                t.title as title,
                t.description as description,
                ts.id as status_id,
                ts.code as status_code,
                ts.name as status_name,
                t.created_at as created_at,
                t.updated_at as updated_at
            from
                todos as t
                inner join
                    todo_statuses as ts
                    on ts.id = t.status_id
            where
                t.id = $1
        "#;

        let stored_todo = query_as::<_, StoredTodo>(sql)
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_todo {
            Some(st) => {
                let delete_sql = r#"
                    delete from todos where id = $1
                "#;

                let _ = query(delete_sql)
                    .bind(id.value.to_string())
                    .execute(&*pool)
                    .await?;

                Ok(Some(st.try_into()?))
            }
            None => Ok(None),
        }
    }
}
