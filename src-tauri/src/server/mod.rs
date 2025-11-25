use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use futures_util::future::{select, Either};
use serde::{Deserialize, Serialize};
use tauri::{
    async_runtime::{self as rt},
    AppHandle, Manager,
};
use tokio::{net::TcpListener as TokioTcpListener, sync::mpsc::UnboundedSender, task::JoinSet};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

mod handle;
mod incoming;
pub mod service_discovery;
pub mod service_register;
pub mod volume_control;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub address: String,
}

type ClientSender = UnboundedSender<Message>;
type ClientMap = Arc<rt::Mutex<HashMap<String, (ClientInfo, ClientSender)>>>;

#[derive(Default)]
pub struct WebSocketServerState {
    clients: ClientMap,
    server: Arc<rt::Mutex<Option<RunningServer>>>,
}

pub struct RunningServer {
    pub name: String,
    handle: rt::JoinHandle<()>,
    cancel: CancellationToken,
}

impl RunningServer {
    pub async fn shutdown(self) -> Result<(), String> {
        self.cancel.cancel();
        self.handle
            .await
            .map_err(|e| format!("[{}] shutdown failed: {}", self.name, e))
    }
}

pub struct ServiceDiscovery {
    server: Arc<rt::Mutex<Option<RunningServer>>>,
}

impl ServiceDiscovery {
    const LISTEN_PORT: u16 = 31280;
    pub const MDNS_DOMAIN: &str = "_volume-service._tcp.local.";
    pub const MDNS_INSTANCE_NAME: &str = "volumize";
    pub const DISCOVERY_MSG: &str = "DISCOVER_VOLUMIZE";
    pub const BROADCAST_ADDRESS: SocketAddrV4 =
        SocketAddrV4::new(Ipv4Addr::BROADCAST, Self::LISTEN_PORT);
    pub const LISTEN_ADDRESS: SocketAddrV4 =
        SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, Self::LISTEN_PORT);

    pub fn new() -> Self {
        Self {
            server: Default::default(),
        }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let mut server = self.server.lock().await;
        match server.take() {
            Some(server) => server.shutdown().await,
            None => Ok(()),
        }
    }
}

impl WebSocketServerState {
    pub fn new() -> Self {
        Self {
            clients: Default::default(),
            server: Default::default(),
        }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let mut server = self.server.lock().await;
        if let Some(server) = server.take() {
            server.shutdown().await?
        }

        self.clients.lock().await.clear();
        Ok(())
    }
}

pub fn start_websocket_server(port: u16, app_handle: &AppHandle) -> String {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);

    let state = app_handle.state::<WebSocketServerState>();
    let clients = state.clients.clone();
    let app_handle_clone = app_handle.clone();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let std_listener = std::net::TcpListener::bind(addr).unwrap();
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
                    conns.spawn(handle::handle_client(
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
    rt::block_on(async {
        let new_server = RunningServer {
            name: "Websocket".into(),
            handle: new_handle,
            cancel,
        };

        let mut current_server = state.server.lock().await;
        if let Some(old) = current_server.replace(new_server) {
            let _ = old.shutdown().await;
        }
    });

    addr.to_string()
}
