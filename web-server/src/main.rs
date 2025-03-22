use axum::routing::{get, post};
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use web_server::endpoints::get_temperature::temperature_get_endpoint;
use web_server::endpoints::post_temperature::temperature_post_endpoint;
use web_server::server_context::ServerContext;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/temperature", get(temperature_get_endpoint))
        .route("/temperature", post(temperature_post_endpoint))
        .layer(Extension(Arc::new(RwLock::new(ServerContext::new()))));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
