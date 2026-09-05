use std::sync::Arc;

use tauri::{async_runtime as rt, AppHandle, Manager};

use crate::server::RunningServer;

#[derive(Default)]
pub struct SerialState {
    server: Arc<rt::Mutex<Option<RunningServer>>>,
}

pub fn start_serial_thread(app_handle: &AppHandle) {
    let _state = app_handle.state::<SerialState>();
}
