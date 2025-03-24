use crate::server_context::ServerContext;
use crate::{GetEndpointsQuery, handle_long_poll};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

pub async fn sensor_data_get_endpoint(
    query: Query<GetEndpointsQuery>,
    context: Extension<ServerContext>,
) -> Response {
    handle_long_poll(async || match &context.read().await.fan_temperature {
        Some(fan_temperature) => {
            if fan_temperature.revision_num() > query.last_known_revision_num {
                Some(Json(fan_temperature).into_response())
            } else {
                None
            }
        }
        None => Some(
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(String::from("No fan connected to server.")),
            )
                .into_response(),
        ),
    })
    .await
}
