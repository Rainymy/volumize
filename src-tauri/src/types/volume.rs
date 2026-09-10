use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};

use tauri::async_runtime as rt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use shared_types::protocol::{
    next_request_id, Command, CommandRequest, CommandResponse, Envelope, RequestId, Response,
};

type PendingRequests = Arc<Mutex<HashMap<RequestId, oneshot::Sender<Response>>>>;

#[derive(Debug)]
pub struct CommandClient {
    sender: mpsc::UnboundedSender<Envelope>,
    pending: PendingRequests,
    cancel: CancellationToken,
    handle: rt::JoinHandle<()>,
}

impl CommandClient {
    pub fn new(
        sender: mpsc::UnboundedSender<Envelope>,
        receiver: mpsc::UnboundedReceiver<Envelope>,
    ) -> Self {
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let cancel = CancellationToken::new();

        let handle = spawn_dispatcher(receiver, pending.clone(), cancel.clone());

        Self {
            sender,
            pending,
            cancel: cancel,
            handle: handle,
        }
    }

    pub async fn request(&self, command: Command) -> Result<CommandResponse, String> {
        let id = next_request_id();
        let (tx, rx) = oneshot::channel();
        self.pending.lock().unwrap().insert(id.clone(), tx);

        let envelope = Envelope::Command(CommandRequest { id, command });
        if self.sender.send(envelope).is_err() {
            self.pending.lock().unwrap().remove(&id);
            return Err("Failed to send request".to_string());
        }

        // optional: wrap in tokio::time::timeout(...) to avoid leaking on a lost response
        match tokio::time::timeout(Duration::from_secs(15), rx).await {
            Ok(Ok(response)) => Ok(CommandResponse { id, response }),
            Ok(Err(e)) => Err(e.to_string()),
            Err(_) => Err("Timeout".to_string()),
        }
    }

    fn close_channel(&mut self) {
        let (tx, _rx) = mpsc::unbounded_channel::<Envelope>();
        let old = std::mem::replace(&mut self.sender, tx);
        drop(old);
    }

    pub fn shutdown(&mut self) {
        self.close_channel();
        self.cancel.cancel();
        self.handle.abort();
    }
}

fn spawn_dispatcher(
    mut receiver: mpsc::UnboundedReceiver<Envelope>,
    pending: PendingRequests,
    shutdown: CancellationToken,
) -> rt::JoinHandle<()> {
    fn handle_envelope(envelope: Envelope, pending: PendingRequests) {
        match envelope {
            Envelope::Response(CommandResponse { id, response }) => {
                if let Some(tx) = pending.lock().unwrap().remove(&id) {
                    let _ = tx.send(response);
                }
            }
            Envelope::Event(_event) => {
                // route to event/broadcast
            }
            Envelope::Command(_) => {
                // shouldn't arrive on this side; ignore or log
            }
        }
    }

    let handle = rt::spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                maybe_envelope = receiver.recv() => {
                    match maybe_envelope {
                        Some(envelope) => handle_envelope(envelope, pending.clone()),
                        None => break, // channel closed, sender side is gone
                    }
                }
            }
        }

        // drain: fail any requests still waiting, instead of leaving them hanging
        for (_, tx) in pending.lock().unwrap().drain() {
            let _ = tx.send(Response::Error {
                message: "dispatcher shutdown".into(),
            });
        }
    });

    handle
}

#[derive(Default)]
pub struct VolumeCommandSender {
    pub server: Arc<Mutex<Option<VolumeServer>>>,
    pub client: Arc<rt::Mutex<Option<CommandClient>>>,
}

impl VolumeCommandSender {
    pub fn send(&self, cmd: Envelope) -> Result<(), String> {
        let server = match self.server.lock() {
            Ok(server) => server,
            Err(err) => return Err(format!("Failed to lock server: {}", err)),
        };

        match &*server {
            Some(server) => server.send(cmd),
            None => Err("No server".to_string()),
        }
    }

    pub async fn request(&self, cmd: Command) -> Result<CommandResponse, String> {
        let server = self.client.lock().await;
        let response = match server.as_ref() {
            Some(server) => server.request(cmd).await,
            None => Err("No server".to_string()),
        };
        response.map_err(|_| "No response".to_string())
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let server_guard = match self.server.lock() {
            Ok(mut server) => server.take(),
            Err(err) => return Err(format!("Failed to lock server: {}", err)),
        };

        let mut client = self.client.lock().await;
        if let Some(mut old) = std::mem::replace(&mut *client, None) {
            old.shutdown();
        }

        match server_guard {
            Some(mut server) => server.shutdown(),
            None => Ok(()),
        }
    }
}

pub struct VolumeServer {
    pub tx: UnboundedSender<Envelope>,
    pub thread_handle: Option<JoinHandle<()>>,
}

impl VolumeServer {
    fn send(&self, cmd: Envelope) -> Result<(), String> {
        self.tx.send(cmd).map_err(|e| format!("Send failed: {}", e))
    }

    fn close_channel(&mut self) {
        let (new_tx, _) = unbounded_channel::<Envelope>();
        // Replace the sender with a new one and drop the original
        let old_tx = std::mem::replace(&mut self.tx, new_tx);
        drop(old_tx); // Explicitly drop the sender
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.close_channel();

        let thread = match self.thread_handle.take() {
            Some(handle) => handle.join(),
            None => return Ok(()),
        };

        thread.map_err(|e| format!("Volume thread panicked during shutdown: {:?}", e))
    }
}
