use crate::module::Modules;
use crate::routes::health_check::{hc, hc_postgres};
use crate::routes::todo::{
    create_todo, delete_todo, find_todo, get_todo, update_todo, upsert_todo,
};
use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn startup(modules: Arc<Modules>) {
    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/postgres", get(hc_postgres));

    let todo_router = Router::new()
        .route("/", get(find_todo).post(create_todo))
        .route(
            "/:id",
            get(get_todo)
                .patch(update_todo)
                .put(upsert_todo)
                .delete(delete_todo),
        );

    let app = Router::new()
        .nest("/:v/hc", hc_router)
        .nest("/:v/todos", todo_router)
        .with_state(modules);

    let addr = SocketAddr::from(init_addr());
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("TcpListener cannot bind."));
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}

fn init_addr() -> (IpAddr, u16) {
    let env_host = env::var_os("HOST").expect("HOST is undefined.");
    let ip_addr = env_host
        .into_string()
        .expect("HOST is invalid.")
        .parse::<IpAddr>()
        .expect("HOST is invalid.");

    let env_port = env::var_os("PORT").expect("PORT is undefined.");
    let port = env_port
        .into_string()
        .expect("PORT is invalid.")
        .parse::<u16>()
        .expect("PORT is invalid.");

    tracing::debug!("Init ip address.");
    (ip_addr, port)
}
