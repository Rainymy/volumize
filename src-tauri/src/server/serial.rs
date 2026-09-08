use std::sync::Arc;

use shared_types::{protocol::RawFrame, reader::read_frame};
use tauri::{async_runtime as rt, AppHandle, Manager};
use tokio::{
    io::{ReadHalf, WriteHalf},
    sync::mpsc,
};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::sync::CancellationToken;

use crate::{server::serialport::find_devices, types::volume::VolumeCommandSender};

pub type SerialSender = mpsc::UnboundedSender<Vec<u8>>;

#[allow(dead_code)]
pub struct RunningSerial {
    pub name: String,
    pub handle: rt::JoinHandle<()>,
    pub cancel: CancellationToken,
    pub sender: SerialSender,
}

impl RunningSerial {
    pub async fn shutdown(self) {
        self.cancel.cancel();
        let _ = self.handle.await;
    }
}

// TODO: server field can be std::sync::Mutex. instead of async Mutex.
#[derive(Default)]
pub struct SerialState {
    pub server: Arc<rt::Mutex<Option<RunningSerial>>>,
    pub serial_port: Option<String>,
}

/// This function is going to **PANIC** when serial is detected.
pub fn start_serial_thread(serial_path: Option<String>, app_handle: &AppHandle) {
    let state = app_handle.state::<SerialState>();
    let server_slot = state.server.clone();
    let app_handle_clone = app_handle.clone();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let (tx, rx) = mpsc::unbounded_channel::<Vec<u8>>();

    let new_handle = rt::spawn(async move {
        let port = match find_devices()
            .into_iter()
            .find(|d| Some(d.name.clone()) == serial_path)
        {
            Some(port) => port,
            None => {
                eprintln!("Serial device not found");
                return;
            }
        };

        let serial = tokio_serial::new(port.name, 115_200);
        let com = match serial.open_native_async() {
            Ok(com) => com,
            Err(e) => {
                eprintln!("Failed to open serial port: {}", e);
                return;
            }
        };

        let (read_half, write_half) = tokio::io::split(com);

        let mut write_task = rt::spawn(handle_outgoing(write_half, rx));
        let mut read_task = rt::spawn(handle_incoming(read_half, app_handle_clone.clone()));

        tokio::select! {
            _ = cancel_clone.cancelled() => {
                write_task.abort();
                read_task.abort();
            }
            _ = &mut write_task => {
                read_task.abort();
                let _ = read_task.await;
            }
            _ = &mut read_task => { // (device unplugged, decode error, etc.)
                write_task.abort();
                let _ = write_task.await;
            }
        }

        println!("Serial thread finished for {:?}", serial_path);
    });

    let new_server = RunningSerial {
        name: "Serial port".into(),
        handle: new_handle,
        cancel: cancel,
        sender: tx,
    };

    let mut current = server_slot.blocking_lock();
    if let Some(old) = current.replace(new_server) {
        rt::block_on(old.shutdown());
    }
}

async fn handle_outgoing(
    mut write: WriteHalf<SerialStream>,
    mut rx: mpsc::UnboundedReceiver<Vec<u8>>,
) {
    use tokio::io::AsyncWriteExt;

    while let Some(data) = rx.recv().await {
        if let Err(e) = write.write_all(&data).await {
            eprintln!("Error writing to serial port: {}", e);
            break;
        }
    }
}

// #[derive(Clone, Serialize)]
// struct SerialErrorPayload {
//     message: String,
// }

async fn handle_incoming(mut read: ReadHalf<SerialStream>, app_handle: AppHandle) {
    // use tauri::Emitter;
    loop {
        match read_frame(&mut read).await {
            Ok(buffer) => match RawFrame::decode(&buffer) {
                Ok(frame) => {
                    let state = app_handle.state::<VolumeCommandSender>();
                    let _ = state.send(frame);
                }
                Err(e) => {
                    eprintln!("Failed to decode frame: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to read frame: {}", e);
                break;
            }
        }
    }
}
