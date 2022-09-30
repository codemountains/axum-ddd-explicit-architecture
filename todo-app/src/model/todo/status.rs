use todo_kernel::model::todo::status::TodoStatus;

pub struct TodoStatusView {
    pub id: String,
    pub code: String,
    pub name: String,
}

impl From<TodoStatus> for TodoStatusView {
    fn from(ts: TodoStatus) -> Self {
        Self {
            id: ts.id.value.to_string(),
            code: ts.code,
            name: ts.name,
        }
    }
}
