use crate::persistence::postgres::Db;
use crate::repository::DatabaseRepositoryImpl;
use todo_kernel::model::todo::Todo;
use todo_kernel::repository::todo::TodoRepository;

pub struct RepositoriesModule {
    todo_repository: DatabaseRepositoryImpl<Todo>,
}

pub trait RepositoriesModuleExt {
    type TodoRepo: TodoRepository;

    fn todo_repository(&self) -> &Self::TodoRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type TodoRepo = DatabaseRepositoryImpl<Todo>;

    fn todo_repository(&self) -> &Self::TodoRepo {
        &self.todo_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let todo_repository = DatabaseRepositoryImpl::new(db.clone());
        Self { todo_repository }
    }
}
