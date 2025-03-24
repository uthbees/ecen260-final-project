use crate::server_context::{FanOverride, ServerContext};
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostSettings {
    fan_activation_temp: Option<i32>,
    fan_override: Option<FanOverride>,
}

pub async fn settings_post_endpoint(
    context: Extension<ServerContext>,
    Json(body): Json<PostSettings>,
) {
    let mut state = context.write().await;

    if let Some(fan_activation_temp) = body.fan_activation_temp {
        state.settings.set_fan_activation_temp(fan_activation_temp);
    }

    if let Some(fan_override) = body.fan_override {
        state.settings.set_fan_override(fan_override);
    }
}
