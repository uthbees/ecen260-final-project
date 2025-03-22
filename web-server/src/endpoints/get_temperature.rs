use crate::server_context::ServerContext;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub enum EndpointResponse {
    Update(UpdateResponse),
    Empty(),
    Err(StatusCode, String),
}

impl IntoResponse for EndpointResponse {
    fn into_response(self) -> Response {
        match self {
            EndpointResponse::Update(success) => (StatusCode::OK, Json(success)).into_response(),
            EndpointResponse::Empty() => StatusCode::OK.into_response(),
            EndpointResponse::Err(status_code, error_message) => {
                (status_code, Json(error_message)).into_response()
            }
        }
    }
}

#[derive(Serialize)]
pub struct UpdateResponse {
    revision_num: i32,
    temperature: i32,
}

#[derive(Deserialize)]
pub struct EndpointQuery {
    last_known_revision_num: i32,
}

pub async fn temperature_get_endpoint(
    query: Query<EndpointQuery>,
    state: Extension<Arc<RwLock<ServerContext>>>,
) -> EndpointResponse {
    let one_second = Duration::from_secs(1);

    // Wait for updates for 20 seconds (long polling)
    for _ in 0..20 {
        match state.read().await.fan_temperature.as_ref() {
            Some(fan_temperature) => {
                if fan_temperature.revision_num() > query.last_known_revision_num {
                    return EndpointResponse::Update(UpdateResponse {
                        revision_num: fan_temperature.revision_num(),
                        temperature: fan_temperature.value(),
                    });
                }
            }
            None => {
                return EndpointResponse::Err(
                    StatusCode::SERVICE_UNAVAILABLE,
                    String::from("No fan connected to server."),
                );
            }
        };

        tokio::time::sleep(one_second).await;
    }

    // Return an empty response to avoid the request timing out
    EndpointResponse::Empty()
}
