use futures_util::future::{select, Either};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tauri::{
    async_runtime::{self as rt},
    AppHandle, Manager,
};
use tokio::{net::UdpSocket, sync::Mutex};
use tokio_util::sync::CancellationToken;

use super::{RunningServer, ServiceDiscovery};
use crate::tray::Discovery;

pub fn start_service_register(port: u16, app_handle: &AppHandle, policy: Discovery) {
    let state = app_handle.state::<ServiceDiscovery>();

    if matches!(policy, Discovery::TurnOff) {
        // Stop and clear existing server, then return.
        replace_server_state(&state.server, None);
        return;
    }

    let cancel = CancellationToken::new();
    let cancel_for_worker = cancel.clone();

    // If there's a duration, spawn a timer task to cancel later.
    if let Discovery::OnDuration(run_duration) = policy {
        println!(
            "[start_service_register]: Turn on register service for, {:?}",
            run_duration
        );

        let cancel_for_timer = cancel.clone();
        rt::spawn(async move {
            let deadline = tokio::time::sleep(run_duration);
            let await_cancel = cancel_for_timer.cancelled();

            // This is to make sure task/thread stops when recieved a cancellation.
            match select(Box::pin(deadline), Box::pin(await_cancel)).await {
                Either::Left(_) => cancel_for_timer.cancel(),
                Either::Right(_) => {} // it is already cancelled, no need to cancel again.
            }
        });
    }

    let new_handle = rt::spawn(async move {
        println!("[start_service_register]: Starting up...");

        // List all mDNS command: dns-sd -B _services._dns-sd._udp
        if let Err(e) = register_service(port, cancel_for_worker).await {
            println!("[start_service_register] Failed: {}", e);
        }

        println!("[start_service_register]: Closing register_service");
    });

    let new_server = RunningServer {
        name: "Service register".into(),
        handle: new_handle,
        cancel: cancel,
    };

    // Replace old server and clear its state.
    replace_server_state(&state.server, Some(new_server));
}

fn replace_server_state(
    current: &Arc<Mutex<Option<RunningServer>>>,
    new_server: Option<RunningServer>,
) {
    rt::block_on(async move {
        let mut current_server = current.lock().await;

        let old_server = match new_server {
            Some(value) => current_server.replace(value),
            None => current_server.take(),
        };

        if let Some(old) = old_server {
            let _ = old.shutdown().await;
        }
    });
}

async fn register_service(
    port: u16,
    cancel: CancellationToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let mdns = init_mdns_service(port)?;

    println!(
        "[register_service]: mDNS service registered on port {}",
        port
    );

    let result = run_udp_responder(port, cancel).await;

    println!("[register_service]: Shutting down mDNS service...");
    shutdown_mdns_service(&mdns)?;
    result
}

fn init_mdns_service(port: u16) -> Result<mdns_sd::ServiceDaemon, Box<dyn std::error::Error>> {
    let service = mdns_sd::ServiceInfo::new(
        ServiceDiscovery::MDNS_DOMAIN,
        ServiceDiscovery::MDNS_INSTANCE_NAME,
        &"volumize_server.local.", // using fixed host name.
        local_ip_address::local_ip()?,
        port,
        None,
    )?;

    let mdns = mdns_sd::ServiceDaemon::new()?;
    mdns.register(service)?;

    Ok(mdns)
}

fn bind_udp_socket(socket_addr: SocketAddr) -> Result<UdpSocket, std::io::Error> {
    use socket2::{Domain, Protocol, Type};

    let domain = match socket_addr {
        SocketAddr::V4(_) => Domain::IPV4,
        SocketAddr::V6(_) => Domain::IPV6,
    };

    let socket = socket2::Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    let _ = socket.set_reuse_address(true);
    #[cfg(unix)]
    let _ = socket.set_reuse_port(true);

    socket.bind(&socket_addr.into())?;
    socket.set_nonblocking(true)?; // Nonblocking for Tokio

    // Conversion: socket2 --> std --> tokio.
    let std_udp: std::net::UdpSocket = socket.into();
    UdpSocket::from_std(std_udp)
}

async fn run_udp_responder(
    port: u16,
    cancel: CancellationToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = bind_udp_socket(ServiceDiscovery::LISTEN_ADDRESS.into())?;
    let mut buf = [0u8; 24];

    println!("Server ready (mDNS + UDP)");

    loop {
        let cancellation = cancel.cancelled();
        let accept = socket.recv_from(&mut buf);

        let (len, addr) = match select(Box::pin(cancellation), Box::pin(accept)).await {
            Either::Left(_) => break,
            Either::Right((result, _)) => result?,
        };

        if &buf[..len] == ServiceDiscovery::DISCOVERY_MSG.as_bytes() {
            let response = format!("SERVER:{}", port);
            socket.send_to(response.as_bytes(), addr).await?;
        }
    }

    Ok(())
}

fn shutdown_mdns_service(mdns: &mdns_sd::ServiceDaemon) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match mdns.unregister(ServiceDiscovery::MDNS_DOMAIN) {
            Err(mdns_sd::Error::Again) => {
                eprintln!("mDNS failed to shutdown, trying again...");
                std::thread::sleep(Duration::from_millis(50));
            }
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
