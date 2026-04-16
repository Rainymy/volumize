use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct ClickState {
    last_click_time: Mutex<Instant>,
    double_click_threshold: Duration,
}

impl ClickState {
    /// If parameter is None, this will default to 500ms.
    ///
    /// windows double click definition:
    /// - https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime
    pub fn new(double_click_threshold_ms: Option<Duration>) -> Self {
        let time = double_click_threshold_ms.unwrap_or(Duration::from_millis(500));

        Self {
            last_click_time: Mutex::new(Instant::now()),
            double_click_threshold: time,
        }
    }
    pub fn is_double_click(&self) -> bool {
        let mut last_click_time = match self.last_click_time.lock() {
            Ok(value) => value,
            Err(e) => e.into_inner(),
        };
        let threshold = self.double_click_threshold;
        let now = Instant::now();

        let is_double = now.saturating_duration_since(*last_click_time) < threshold;

        *last_click_time = now;
        is_double
    }
}
