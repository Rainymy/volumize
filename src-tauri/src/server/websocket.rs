use futures_util::future::{select, Either};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};
use tokio_util::sync::CancellationToken;

use tauri::{
    async_runtime::{self as rt},
    AppHandle, Manager,
};
use tokio::{net::TcpListener as TokioTcpListener, sync::mpsc::UnboundedSender, task::JoinSet};
use tokio_tungstenite::tungstenite::Message;

use crate::server::{handle::handle_client, RunningServer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub address: String,
}

type ClientSender = UnboundedSender<Message>;
pub type ClientMap = Arc<rt::Mutex<HashMap<String, (ClientInfo, ClientSender)>>>;

#[derive(Default)]
pub struct WebSocketServerState {
    pub clients: ClientMap,
    pub server: Arc<rt::Mutex<Option<RunningServer>>>,
}

impl WebSocketServerState {
    pub async fn shutdown(&self) -> Result<(), String> {
        let mut server = self.server.lock().await;
        if let Some(server) = server.take() {
            server.shutdown().await?
        }

        self.clients.lock().await.clear();
        Ok(())
    }
}

pub fn start_websocket_server(
    port: u16,
    app_handle: &AppHandle,
) -> Result<String, Box<dyn std::error::Error>> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);

    let state = app_handle.state::<WebSocketServerState>();
    let clients = state.clients.clone();
    let app_handle_clone = app_handle.clone();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let std_listener = std::net::TcpListener::bind(addr)?;
    std_listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let new_handle = rt::spawn(async move {
        let async_listener = TokioTcpListener::from_std(std_listener)
            .expect("Failed to convert tokio listener into std");
        let mut conns = JoinSet::new();

        loop {
            let cancelled = cancel_clone.cancelled();
            let accept = async_listener.accept();

            match select(Box::pin(cancelled), Box::pin(accept)).await {
                Either::Left(_) => break,
                Either::Right((Ok((stream, peer_addr)), _)) => {
                    conns.spawn(handle_client(
                        stream,
                        peer_addr,
                        clients.clone(),
                        app_handle_clone.clone(),
                    ));
                }
                Either::Right((Err(_), _)) => break,
            };
        }

        conns.shutdown().await;
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

    Ok(addr.to_string())
}
