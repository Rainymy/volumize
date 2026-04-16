use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct DoubleClickState {
    last_click_time: Mutex<Instant>,
    threshold: Duration,
}

impl DoubleClickState {
    /// If parameter is None, this will default to 500ms.
    ///
    /// windows double click definition:
    /// - https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime
    pub fn new(threshold: Option<Duration>) -> Self {
        Self {
            last_click_time: Mutex::new(Instant::now()),
            threshold: threshold.unwrap_or(Duration::from_millis(500)),
        }
    }
    pub fn is_double_click(&self) -> bool {
        let mut last_click_time = match self.last_click_time.lock() {
            Ok(value) => value,
            Err(e) => e.into_inner(),
        };

        let current_time = Instant::now();
        let eclipsed = current_time.saturating_duration_since(*last_click_time);

        *last_click_time = current_time;

        eclipsed < self.threshold
    }
}
