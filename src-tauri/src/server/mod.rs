use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, Mutex},
};

use tauri::async_runtime::{self as rt};
use tokio_util::sync::CancellationToken;

mod handle;
mod incoming;
pub mod serial;
pub mod serialport;
pub mod service_discovery;
pub mod service_register;
pub mod volume_control;
pub mod websocket;

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

#[derive(Default)]
pub struct ServiceDiscovery {
    server: Arc<Mutex<Option<RunningServer>>>,
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

    pub async fn shutdown(&self) -> Result<(), String> {
        let mut server = match self.server.lock() {
            Ok(server) => server,
            Err(e) => return Err(format!("Failed to lock server: {}", e)),
        };
        match server.take() {
            Some(server) => server.shutdown().await,
            None => Ok(()),
        }
    }
}
