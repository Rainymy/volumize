use std::sync::Arc;

use shared_types::{protocol::RawFrame, reader::read_frame};
use tauri::{async_runtime as rt, AppHandle, Manager};
use tokio_serial::SerialPortBuilderExt;
use tokio_util::sync::CancellationToken;

use crate::server::{serialport::find_device, RunningServer};

#[derive(Default)]
pub struct SerialState {
    server: Arc<rt::Mutex<Option<RunningServer>>>,
}

pub fn start_serial_thread(app_handle: &AppHandle) {
    let state = app_handle.state::<SerialState>();

    let cancel = CancellationToken::new();

    let new_handle = rt::spawn(async move {
        let port = match find_device() {
            Some(port) => port,
            None => return (),
        };
        let serial = tokio_serial::new(port.name, 115_200);
        let mut com = match serial.open_native_async() {
            Ok(com) => com,
            Err(e) => {
                eprintln!("Failed to open serial port: {}", e);
                return ();
            }
        };

        loop {
            match read_frame(&mut com).await {
                Ok(buffer) => {
                    let envelope = match RawFrame::decode(&buffer) {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("Failed to decode frame: {}", e);
                            return ();
                        }
                    };
                    let _ = envelope;
                }
                Err(e) => {
                    eprintln!("Failed to read frame: {}", e);
                    return ();
                }
            }
        }
    });

    // Store the server handle
    let new_server = RunningServer {
        name: "Websocket".into(),
        handle: new_handle,
        cancel,
    };
    let mut current_server = state.server.blocking_lock();
    if let Some(old) = current_server.replace(new_server) {
        rt::spawn(old.shutdown());
    }
}
