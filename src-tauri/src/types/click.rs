use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

pub struct ClickState {
    pub last_click_time: Arc<Mutex<Instant>>,
    double_click_threshold_ms: u64,
}

impl ClickState {
    /// If parameter is None, this will default to 500ms.
    ///
    /// windows double click definition:
    /// - https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime
    pub fn new(double_click_threshold_ms: Option<u64>) -> Self {
        Self {
            last_click_time: Arc::new(Mutex::new(Instant::now())),
            double_click_threshold_ms: double_click_threshold_ms.unwrap_or(500),
        }
    }
    pub fn is_double_click(&self) -> bool {
        let mut last_click_time = match self.last_click_time.lock() {
            Ok(value) => value,
            Err(e) => e.into_inner(),
        };
        let threshold = Duration::from_millis(self.double_click_threshold_ms);
        let now = Instant::now();

        let is_double = now.saturating_duration_since(*last_click_time) < threshold;

        *last_click_time = now;
        is_double
    }
}
