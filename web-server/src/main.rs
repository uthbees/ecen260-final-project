use axum::routing::{get, post};
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use web_server::endpoints::get_sensor_data::sensor_data_get_endpoint;
use web_server::endpoints::get_settings::settings_get_endpoint;
use web_server::endpoints::post_sensor_data::sensor_data_post_endpoint;
use web_server::endpoints::post_settings::settings_post_endpoint;
use web_server::server_context::ServerState;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sensor_data", get(sensor_data_get_endpoint))
        .route("/sensor_data", post(sensor_data_post_endpoint))
        .route("/settings", get(settings_get_endpoint))
        .route("/settings", post(settings_post_endpoint))
        .layer(Extension(Arc::new(RwLock::new(ServerState::new()))));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
