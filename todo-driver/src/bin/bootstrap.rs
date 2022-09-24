use std::sync::Arc;
use todo_driver::module::Modules;
use todo_driver::startup::{init_app, startup};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}
