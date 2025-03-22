use std::sync::atomic::{AtomicI32, Ordering};

pub struct ServerContext {
    pub fan_temperature: Option<Temperature>,
}

impl Default for ServerContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerContext {
    pub fn new() -> Self {
        ServerContext {
            fan_temperature: None,
        }
    }
}

pub struct Temperature {
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
