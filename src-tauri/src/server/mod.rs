use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex},
};
use tokio_tungstenite::tungstenite::Message;

use crate::server::handle::handle_client;

mod handle;
mod incoming;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerMessage {
    pub client_id: String,
    pub message: String,
}

type ClientSender = mpsc::UnboundedSender<Message>;
type ClientMap = Arc<Mutex<HashMap<String, (ClientInfo, ClientSender)>>>;

#[derive(Default)]
pub struct WebSocketServerState {
    clients: ClientMap,
    server_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
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

    let server_task = tokio::spawn(async move {
        while let Ok((stream, peer_addr)) = listener.accept().await {
            tokio::spawn(handle_client(
                stream,
                peer_addr,
                clients.clone(),
                app_handle_clone.clone(),
            ));
        }
    });

    // Store the server handle
    {
        let mut handle = state.server_handle.lock().await;
        if let Some(old_task) = handle.replace(server_task) {
            old_task.abort();
        }
    }

    Ok(addr)
}
