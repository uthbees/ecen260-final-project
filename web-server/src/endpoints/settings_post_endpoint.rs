use crate::server_context::ServerContext;
use axum::Extension;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn settings_post_endpoint(state: Extension<Arc<RwLock<ServerContext>>>) {
    todo!()
}
