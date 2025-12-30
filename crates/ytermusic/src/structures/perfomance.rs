use std::time::Instant;

use once_cell::sync::Lazy;

pub struct Performance {
    pub initial: Instant,
}

impl Performance {
    pub fn new() -> Self {
        Self {
            initial: Instant::now(),
        }
    }

    pub fn get_ms(&self) -> u128 {
        self.initial.elapsed().as_millis()
    }

    pub fn log(&self, label: &str) {
        log::info!("Performance - {}: {} ms", label, self.get_ms());
    }
}

pub static STARTUP_TIME: Lazy<Performance> = Lazy::new(|| Performance::new());
