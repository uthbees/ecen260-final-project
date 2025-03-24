use crate::server_context::ServerContext;
use crate::{GetEndpointsQuery, handle_long_poll};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateResponse {
    revision_num: i32,
    temperature: i32,
}

pub async fn sensor_data_get_endpoint(
    query: Query<GetEndpointsQuery>,
    context: Extension<ServerContext>,
) -> Response {
    handle_long_poll(async || match &context.read().await.fan_temperature {
        Some(fan_temperature) => {
            let revision_num = fan_temperature.revision_num();

            if revision_num > query.last_known_revision_num {
                Some(
                    Json(UpdateResponse {
                        revision_num,
                        temperature: fan_temperature.value(),
                    })
                    .into_response(),
                )
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
