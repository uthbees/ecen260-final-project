use serde::{Serialize, Serializer};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
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

pub struct Temperature {
    /// The temperature in degrees Fahrenheit.
    value: AtomicI32,
    /// Increments every time the value is changed.
    revision_num: AtomicI32,
}

impl Temperature {
    pub fn new(value: i32) -> Self {
        Temperature {
            value: AtomicI32::from(value),
            revision_num: AtomicI32::new(0),
        }
    }

    pub fn value(&self) -> i32 {
        self.value.load(Ordering::SeqCst)
    }

    pub fn revision_num(&self) -> i32 {
        self.revision_num.load(Ordering::SeqCst)
    }

    pub fn set_value(&self, value: i32) {
        self.value.store(value, Ordering::SeqCst);
        self.revision_num
            .store(self.revision_num() + 1, Ordering::SeqCst);
    }
}

pub struct Settings {
    /// The temperature in Fahrenheit at which the fan will automatically turn on.
    fan_activation_temp: AtomicI32,
    fan_override: FanOverride,
    /// Increments every time a setting is changed.
    revision_num: AtomicI32,
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
            fan_activation_temp: AtomicI32::new(80),
            fan_override: FanOverride::None,
            revision_num: AtomicI32::new(0),
        }
    }

    pub fn fan_activation_temp(&self) -> i32 {
        self.fan_activation_temp.load(Ordering::SeqCst)
    }

    pub fn fan_override(&self) -> FanOverride {
        self.fan_override
    }

    pub fn revision_num(&self) -> i32 {
        self.revision_num.load(Ordering::SeqCst)
    }

    pub fn set_activation_temp(&self, value: i32) {
        self.fan_activation_temp.store(value, Ordering::SeqCst);
        self.revision_num
            .store(self.revision_num() + 1, Ordering::SeqCst);
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum FanOverride {
    None,
    On,
    Off,
}
