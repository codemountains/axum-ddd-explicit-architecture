use crate::model::todo::TodoView;
use std::sync::Arc;
use todo_adapter::modules::RepositoriesModuleExt;
use todo_kernel::repository::todo::TodoRepository;

pub struct TodoUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get_todo(&self, id: String) -> anyhow::Result<Option<TodoView>> {
        let resp = self
            .repositories
            .todo_repository()
            .get(&id.try_into()?)
            .await?;

        match resp {
            Some(t) => Ok(Some(t.into())),
            None => Ok(None),
        }
    }

    pub async fn find_todo(&self) -> anyhow::Result<Option<Vec<TodoView>>> {
        let resp = self.repositories.todo_repository().find().await?;

        match resp {
            Some(todos) => {
                let tv_list = todos.into_iter().map(|t| t.into()).collect();
                Ok(Some(tv_list))
            }
            None => Ok(None),
        }
    }
}
