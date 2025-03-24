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
        let revision_num = settings.revision_num();

        if revision_num > query.last_known_revision_num {
            Some(
                Json(UpdateResponse {
                    revision_num,
                    fan_activation_temp: settings.fan_activation_temp(),
                    fan_override: settings.fan_override(),
                })
                .into_response(),
            )
        } else {
            None
        }
    })
    .await
}
