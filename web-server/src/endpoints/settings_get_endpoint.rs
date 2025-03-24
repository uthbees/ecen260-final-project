use crate::endpoints::get_sensor_data::EndpointQuery;
use crate::server_context::ServerContext;
use axum::Extension;
use axum::extract::Query;
use axum::response::Response;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn settings_get_endpoint(
    query: Query<EndpointQuery>,
    state: Extension<Arc<RwLock<ServerContext>>>,
) -> Response {
    todo!();
}
