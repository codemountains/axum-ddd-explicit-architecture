use axum::extract::State;
use std::sync::Arc;
use todo_driver::module::Modules;
use todo_driver::startup::init_app;

pub async fn setup_state_modules() -> State<Arc<Modules>> {
    init_app();

    let modules = Modules::new().await;
    State(Arc::new(modules))
}
