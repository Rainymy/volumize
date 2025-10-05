use std::{collections::HashMap, sync::Arc};

use futures_util::future::{select, Either};
use serde::{Deserialize, Serialize};
use tauri::async_runtime as rt;
use tauri::{AppHandle, Manager};
use tokio::{net::TcpListener, sync::mpsc, task::JoinSet};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::server::handle::handle_client;

mod handle;
mod incoming;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub address: String,
}

type ClientSender = mpsc::UnboundedSender<Message>;
type ClientMap = Arc<rt::Mutex<HashMap<String, (ClientInfo, ClientSender)>>>;

#[derive(Default)]
pub struct WebSocketServerState {
    clients: ClientMap,
    server: Arc<rt::Mutex<Option<RunningServer>>>,
}

struct RunningServer {
    handle: rt::JoinHandle<()>,
    cancel: CancellationToken,
}

impl WebSocketServerState {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(rt::Mutex::new(HashMap::new())),
            server: Arc::new(rt::Mutex::new(None)),
        }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let mut server = self.server.lock().await;

        if let Some(s) = server.take() {
            s.cancel.cancel();
            let _ = s
                .handle
                .await
                .map_err(|e| format!("Server task failed: {}", e))?;
        }

        self.clients.lock().await.clear();
        Ok(())
    }
}

pub async fn start_websocket_server(
    port: u16,
    app_handle: AppHandle,
) -> Result<String, std::io::Error> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    let state = app_handle.state::<WebSocketServerState>();
    let clients = state.clients.clone();
    let app_handle_clone = app_handle.clone();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let new_handle = rt::spawn(async move {
        let mut conns = JoinSet::new();

        loop {
            let cancelled = cancel_clone.cancelled();
            let accept = listener.accept();

            match select(Box::pin(cancelled), Box::pin(accept)).await {
                Either::Left(_) => {
                    break;
                }
                Either::Right((Ok((stream, peer_addr)), _)) => {
                    conns.spawn(handle_client(
                        stream,
                        peer_addr,
                        clients.clone(),
                        app_handle_clone.clone(),
                    ));
                }
                Either::Right((Err(_), _)) => {
                    break;
                }
            };
        }

        conns.shutdown().await;
    });

    // Store the server handle
    {
        let mut current_server = state.server.lock().await;
        let new_server = RunningServer {
            handle: new_handle,
            cancel,
        };

        if let Some(old) = current_server.replace(new_server) {
            old.cancel.cancel();
            let _ = old.handle.await;
        }
    }

    Ok(addr)
}
