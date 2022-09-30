use crate::persistence::postgres::Db;
use crate::repository::DatabaseRepositoryImpl;
use todo_kernel::model::todo::status::TodoStatus;
use todo_kernel::model::todo::Todo;
use todo_kernel::repository::todo::status::TodoStatusRepository;
use todo_kernel::repository::todo::TodoRepository;

pub struct RepositoriesModule {
    todo_repository: DatabaseRepositoryImpl<Todo>,
    todo_status_repository: DatabaseRepositoryImpl<TodoStatus>,
}

pub trait RepositoriesModuleExt {
    type TodoRepo: TodoRepository;
    type TodoStatusRepo: TodoStatusRepository;

    fn todo_repository(&self) -> &Self::TodoRepo;
    fn todo_status_repository(&self) -> &Self::TodoStatusRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type TodoRepo = DatabaseRepositoryImpl<Todo>;
    type TodoStatusRepo = DatabaseRepositoryImpl<TodoStatus>;

    fn todo_repository(&self) -> &Self::TodoRepo {
        &self.todo_repository
    }

    fn todo_status_repository(&self) -> &Self::TodoStatusRepo {
        &self.todo_status_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let todo_repository = DatabaseRepositoryImpl::new(db.clone());
        let todo_status_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            todo_repository,
            todo_status_repository,
        }
    }
}
