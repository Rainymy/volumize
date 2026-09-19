use std::sync::Arc;

use shared_types::{
    protocol::{CommandRequest, CommandResponse, Envelope, RawFrame, Response},
    reader::read_frame,
};
use tauri::{async_runtime as rt, AppHandle, Manager};
use tokio::{
    io::{ReadHalf, WriteHalf},
    sync::mpsc,
};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::sync::CancellationToken;

use crate::{server::serialport::find_devices, types::volume::VolumeCommandSender};

pub type SerialSender = mpsc::UnboundedSender<Envelope>;

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
    #[allow(dead_code)]
    pub serial_port: Option<String>,
}

/// This function is going to **PANIC** when serial is detected.
pub fn start_serial_thread(serial_path: Option<String>, app_handle: &AppHandle) {
    let state = app_handle.state::<SerialState>();
    let server_slot = state.server.clone();
    let app_handle_clone = app_handle.clone();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let (tx, rx) = mpsc::unbounded_channel::<Envelope>();
    let tx_clone = tx.clone();

    let new_handle = rt::spawn(async move {
        let port = match find_devices()
            .into_iter()
            .find(|d| Some(&d.name) == serial_path.as_ref())
        {
            Some(port) => port,
            None => {
                eprintln!("[start_serial_thread] Serial device not found");
                return;
            }
        };

        println!("[start_serial_thread] Found serial device: {}", port.name);

        let serial_builder = tokio_serial::new(&port.name, 115_200);
        let serial_stream = match serial_builder.open_native_async() {
            Ok(com) => com,
            Err(e) => {
                eprintln!("[start_serial_thread] Failed to open serial port: {}", e);
                return;
            }
        };

        println!("[start_serial_thread] Connected to a serial port");
        let (read_half, write_half) = tokio::io::split(serial_stream);

        let mut write_task = rt::spawn(handle_outgoing(write_half, rx));
        let mut read_task = rt::spawn(handle_incoming(
            read_half,
            tx_clone,
            app_handle_clone.clone(),
        ));

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

        println!("[start_serial_thread] Serial closed for {}", port.name);
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
    mut rx: mpsc::UnboundedReceiver<Envelope>,
) {
    use tokio::io::AsyncWriteExt;

    while let Some(data) = rx.recv().await {
        let frame = RawFrame::encode(&data).build();
        println!("[handle_outgoing] Sending: {} bytes", frame.len());

        if let Err(e) = write.write_all(&frame).await {
            eprintln!("Error writing to serial port: {}", e);
            break;
        }
    }
}

async fn handle_incoming(
    mut read: ReadHalf<SerialStream>,
    tx: mpsc::UnboundedSender<Envelope>,
    app_handle: AppHandle,
) {
    loop {
        let buffer = match read_frame(&mut read).await {
            Ok(buffer) => buffer,
            Err(e) => {
                eprintln!("Failed to read incoming buffer: {}", e);
                continue;
            }
        };

        let frame = match RawFrame::decode(&buffer) {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Failed to decode frame: {}", e);
                continue;
            }
        };

        eprintln!("[handle_incoming] Received: {:?}", frame);
        let state = app_handle.state::<VolumeCommandSender>();
        let Envelope::Command(CommandRequest { id: _id, command }) = frame else {
            eprintln!("[handle_incoming] Invalid frame: {:?}", frame);

            let _ = tx.send(Envelope::Response(CommandResponse {
                id: 0,
                response: Response::Error {
                    message: "Invalid frame".to_string(),
                },
            }));
            continue;
        };

        let mut client = state.client.lock().await;
        let result = match client.as_mut() {
            Some(client) => client.request(command).await,
            None => Err("No client connected".to_string()),
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                eprintln!("[handle_incoming] Failed to internal send frame: {}", e);
                continue;
            }
        };

        if let Err(err) = tx.send(Envelope::Response(response)) {
            eprintln!("[handle_incoming] Failed to send frame outside: {}", err);
        }
    }
}
