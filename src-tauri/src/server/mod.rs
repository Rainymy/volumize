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
mod service_discovery;
mod service_register;
mod volume_control;

pub use service_discovery::discover_server;
#[allow(unused_imports)]
pub use service_register::start_service_register;
#[allow(unused_imports)]
pub use volume_control::*;

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
    pub const MDNS_DOMAIN: &str = "_volume-service._tcp.local.";
    pub const MDNS_INSTANCE_NAME: &str = "volumize";
    pub const DISCOVERY_MSG: &str = "DISCOVER_VOLUMIZE";
    pub const BROADCAST_ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::BROADCAST, 51280);
    pub const LISTEN_ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 51280);

    pub fn new() -> Self {
        Self {
            server: Arc::new(rt::Mutex::new(None)),
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
            clients: Arc::new(rt::Mutex::new(HashMap::new())),
            server: Arc::new(rt::Mutex::new(None)),
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
    std_listener.set_nonblocking(true)?;

    let new_handle = rt::spawn(async move {
        let async_listener = TokioTcpListener::from_std(std_listener)
            .expect("failed to convert std listener to tokio");
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

    Ok(addr.to_string())
}
