use crate::server_context::{ServerContext, Temperature};
use axum::Extension;
use rand::{Rng, rng};

pub async fn sensor_data_post_endpoint(context: Extension<ServerContext>) {
    let mut state = context.write().await;

    // TODO: Instead of assigning a random value like this, read the request body and assign the value from there.
    let mut rng = rng();
    let new_value = rng.random_range(1..=10);

    match state.fan_temperature.as_ref() {
        None => state.fan_temperature = Some(Temperature::new(new_value)),
        Some(fan_temperature) => {
            fan_temperature.set_value(new_value);
        }
    }
}
