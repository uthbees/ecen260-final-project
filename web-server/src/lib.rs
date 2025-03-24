use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::time::Duration;

pub mod endpoints;
pub mod server_context;

#[derive(Deserialize)]
pub struct GetEndpointsQuery {
    last_known_revision_num: i32,
}

/// Handle long polling on the server side. Calls the callback once every second for a while,
/// returning a response once completed. Once the callback returns Some(Response), the response
/// is returned, and if it takes too long to do so, an empty response is returned to avoid the
/// request timing out.
async fn handle_long_poll<F, Fut>(callback: F) -> Response
where
    F: Fn() -> Fut,
    Fut: Future<Output = Option<Response>>,
{
    let one_second = Duration::from_secs(1);

    // Wait for updates for 20 seconds
    for _ in 0..20 {
        let result = callback().await;

        if let Some(response) = result {
            return response;
        }

        tokio::time::sleep(one_second).await;
    }

    // Return an empty response to avoid the request timing out
    ().into_response()
}
