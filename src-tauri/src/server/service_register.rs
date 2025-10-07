use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};
use tauri::{
    async_runtime::{self as rt},
    AppHandle, Manager,
};
use tokio::net::UdpSocket;
use tokio_util::sync::CancellationToken;

use super::{RunningServer, ServiceDiscovery};

// Later in devlopment. Make it so that user can eiter choice to have it:
// - [always on, off, on some amount of time]
pub async fn start_service_register(port: u16, app_handle: AppHandle) {
    let state = app_handle.state::<ServiceDiscovery>();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let new_handle = rt::spawn(async move {
        // List all mDNS command: dns-sd -B _services._dns-sd._udp
        if let Err(e) = register_service(port, cancel_clone).await {
            println!("{}", e);
        }
    });

    let new_server = RunningServer {
        name: "Service register".into(),
        handle: new_handle,
        cancel: cancel,
    };

    let mut current_server = state.server.lock().await;
    if let Some(old) = current_server.replace(new_server) {
        let _ = old.shutdown().await;
    }
}

async fn register_service(
    port: u16,
    cancel: CancellationToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let mdns = init_mdns_service(port)?;
    let result = run_udp_responder(port, cancel).await;
    shutdown_mdns_service(&mdns)?;
    result
}

fn init_mdns_service(port: u16) -> Result<mdns_sd::ServiceDaemon, Box<dyn std::error::Error>> {
    let mdns = mdns_sd::ServiceDaemon::new()?;
    let hostname = "volumize_server.local."; // using fixed host name.

    // let properties = [("version", "1.0"), ("api", "v2")];
    let service = mdns_sd::ServiceInfo::new(
        ServiceDiscovery::SERVICE_MDNS_DOMAIN,
        ServiceDiscovery::SERVICE_MDNS_INSTANCE_NAME,
        &hostname,
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 115)),
        port,
        None, // &properties[..],
    )?;

    mdns.register(service)?;
    println!("mDNS service registered on port {}", port);

    Ok(mdns)
}

async fn run_udp_responder(
    port: u16,
    cancel: CancellationToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:51820").await?;
    let mut buf = [0u8; 24];

    println!("Server ready (mDNS + UDP fallback)");

    use futures_util::future::{select, Either};

    loop {
        let cancellation = cancel.cancelled();
        let accept = socket.recv_from(&mut buf);

        let (len, addr) = match select(Box::pin(cancellation), Box::pin(accept)).await {
            Either::Left(_) => break,
            Either::Right((result, _)) => result?,
        };

        if &buf[..len] == ServiceDiscovery::SERVICE_DISCOVERY_MSG {
            let response = format!("SERVER:{}", port);
            socket.send_to(response.as_bytes(), addr).await?;
        }
        println!("looping...");
    }

    Ok(())
}

fn shutdown_mdns_service(mdns: &mdns_sd::ServiceDaemon) -> Result<(), String> {
    println!("Shutting down mDNS service...");
    loop {
        match mdns.unregister(ServiceDiscovery::SERVICE_MDNS_DOMAIN) {
            Err(mdns_sd::Error::Again) => {
                eprintln!("mDNS failed to shutdown, trying again...");
                std::thread::sleep(Duration::from_millis(50));
            }
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(format!("mDNS shutdown error: {}", e));
            }
        }
    }
}
