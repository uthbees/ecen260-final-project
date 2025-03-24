use crate::server_context::{ServerContext, Temperature};
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostSensorData {
    temperature: i32,
}

pub async fn sensor_data_post_endpoint(
    context: Extension<ServerContext>,
    Json(body): Json<PostSensorData>,
) {
    let mut state = context.write().await;

    match state.fan_temperature.as_ref() {
        None => state.fan_temperature = Some(Temperature::new(body.temperature)),
        Some(fan_temperature) => {
            fan_temperature.set_value(body.temperature);
        }
    }
}
