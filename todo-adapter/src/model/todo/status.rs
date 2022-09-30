use sqlx::FromRow;
use todo_kernel::model::todo::status::TodoStatus;

#[derive(FromRow, Debug)]
pub struct StoredTodoStatus {
    pub id: String,
    pub code: String,
    pub name: String,
}

impl TryFrom<StoredTodoStatus> for TodoStatus {
    type Error = anyhow::Error;

    fn try_from(ts: StoredTodoStatus) -> Result<Self, Self::Error> {
        Ok(TodoStatus {
            id: ts.id.try_into()?,
            code: ts.code,
            name: ts.name,
        })
    }
}
