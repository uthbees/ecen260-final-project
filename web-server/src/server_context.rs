use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type ServerContext = Arc<RwLock<ServerState>>;

pub struct ServerState {
    pub fan_temperature: Option<Temperature>,
    pub settings: Settings,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            fan_temperature: None,
            settings: Settings::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Temperature {
    /// The temperature in degrees Fahrenheit.
    value: i32,
    /// Increments every time the value is changed.
    revision_num: i32,
}

impl Temperature {
    pub fn new(value: i32) -> Self {
        Temperature {
            value,
            revision_num: 0,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn revision_num(&self) -> i32 {
        self.revision_num
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.revision_num += 1;
    }
}

#[derive(Serialize)]
pub struct Settings {
    /// The temperature in degrees Fahrenheit at which the fan will automatically turn on.
    fan_activation_temp: i32,
    fan_override: FanOverride,
    /// Increments every time a setting is changed.
    revision_num: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            // Initialize to 80 degrees.
            fan_activation_temp: 80,
            fan_override: FanOverride::None,
            revision_num: 0,
        }
    }

    pub fn fan_activation_temp(&self) -> i32 {
        self.fan_activation_temp
    }

    pub fn fan_override(&self) -> FanOverride {
        self.fan_override
    }

    pub fn revision_num(&self) -> i32 {
        self.revision_num
    }

    pub fn set_fan_activation_temp(&mut self, value: i32) {
        self.fan_activation_temp = value;
        self.revision_num += 1;
    }

    pub fn set_fan_override(&mut self, value: FanOverride) {
        self.fan_override = value;
        self.revision_num += 1;
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum FanOverride {
    None,
    On,
    Off,
}
