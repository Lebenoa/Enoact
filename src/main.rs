mod activity_manager;
mod client_id;
mod macros;
mod route;

use axum::{
    Router,
    routing::{any, get},
};

const BIND_ENDPOINT: &str = "0.0.0.0:5579";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .with_env_filter("enoact=debug")
        .init();

    let app = Router::new()
        .route("/", get(route::index))
        .route("/ws", any(route::upgrade_handler));

    let listener = tokio::net::TcpListener::bind(BIND_ENDPOINT).await.unwrap();
    tracing::info!("Listening on {BIND_ENDPOINT}");
    axum::serve(listener, app).await.unwrap();
}
