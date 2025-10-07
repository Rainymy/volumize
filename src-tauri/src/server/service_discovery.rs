use std::time::Duration;

use futures_util::future::{select, Either};
use mdns_sd::{ServiceDaemon, ServiceEvent};

use super::ServiceDiscovery;

pub async fn mdns_discover(timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse(&ServiceDiscovery::MDNS_DOMAIN)?;

    loop {
        let deadline = tokio::time::sleep(timeout);

        match select(Box::pin(deadline), receiver.recv_async()).await {
            Either::Left(_) => return Err("mDNS timeout".into()),
            Either::Right((Ok(service), _)) => {
                if let ServiceEvent::ServiceResolved(info) = &service {
                    if let Some(addr) = info.get_addresses().iter().next() {
                        let port = info.get_port();
                        return Ok(format!("{}:{}", addr, port));
                    }
                }
            }
            Either::Right((Err(err), _)) => return Err(err.into()),
        }
    }
}

pub async fn broadcast_discover(timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    socket
        .send_to(
            ServiceDiscovery::DISCOVERY_MSG.as_bytes(),
            ServiceDiscovery::BROADCAST_ADDRESS,
        )
        .await?;

    let mut buf = [0u8; 32];

    let deadline = tokio::time::sleep(timeout);
    let accept = socket.recv_from(&mut buf);

    let (len, addr) = match select(Box::pin(deadline), Box::pin(accept)).await {
        Either::Left(_) => return Err("Broadcast timeout".into()),
        Either::Right((result, _)) => result?,
    };

    let response = String::from_utf8_lossy(&buf[..len]);
    // Parse "SERVER:8080" format
    let port = response
        .split(":")
        .nth(1)
        .ok_or("Invalid server response format")?
        .parse::<u32>()
        .unwrap_or(0);

    Ok(format!("{}:{}", addr.ip(), port))
}

pub async fn discover_server() -> Result<String, Box<dyn std::error::Error>> {
    println!("Trying mDNS discovery...");
    match mdns_discover(Duration::from_secs(3)).await {
        Ok(addr) => {
            println!("Found via mDNS: {}", addr);
            return Ok(addr);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    println!("Trying UDP broadcast...");
    match broadcast_discover(Duration::from_secs(3)).await {
        Ok(addr) => {
            println!("Found via broadcast: {}", addr);
            return Ok(addr);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    Err("Could not discover server. Please enter IP manually.".into())
}
