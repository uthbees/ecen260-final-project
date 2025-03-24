use crate::server_context::{FanOverride, ServerContext};
use crate::{GetEndpointsQuery, handle_long_poll};
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateResponse {
    revision_num: i32,
    fan_activation_temp: i32,
    fan_override: FanOverride,
}

pub async fn settings_get_endpoint(
    query: Query<GetEndpointsQuery>,
    context: Extension<ServerContext>,
) -> Response {
    handle_long_poll(async || {
        let settings = &context.read().await.settings;

        if settings.revision_num() > query.last_known_revision_num {
            Some(Json(settings).into_response())
        } else {
            None
        }
    })
    .await
}
