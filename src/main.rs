mod activity_manager;
mod client_id;
mod macros;
mod route;
mod sessions;

use axum::{
    Router,
    routing::{any, get, post},
};

const BIND_ENDPOINT: &str = "0.0.0.0:5579";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .with_env_filter("enoact=debug")
        .init();

    let api_router = Router::new()
        .route("/active-app-ids", get(route::api::active_app_ids))
        .route("/set-presence", post(route::api::set_presence))
        .route("/clear-presence", post(route::api::clear_presence));

    let app = Router::new()
        .route("/", get(route::index))
        .route("/ws", any(route::upgrade_handler))
        .nest("/api", api_router);

    let listener = tokio::net::TcpListener::bind(BIND_ENDPOINT).await.unwrap();
    tracing::info!("Listening on {BIND_ENDPOINT}");
    axum::serve(listener, app).await.unwrap();
}
