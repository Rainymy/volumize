use std::net::SocketAddr;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tauri::AppHandle;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use uuid::Uuid;

use super::{incoming::handle_incoming_messages, ClientInfo, ClientMap};

pub async fn handle_client(
    stream: TcpStream,
    peer_addr: SocketAddr,
    clients: ClientMap,
    app_handle: AppHandle,
) {
    let client_id = Uuid::new_v4().to_string();

    match accept_async(stream).await {
        Ok(ws_stream) => {
            println!("Client {} connected from {}", client_id, peer_addr);

            let (write, read) = ws_stream.split();
            let (tx, rx) = mpsc::unbounded_channel();

            let client_info = ClientInfo {
                id: client_id.clone(),
                address: peer_addr.to_string(),
            };

            // Add client to the map
            {
                let mut clients_guard = clients.lock().await;
                clients_guard.insert(client_id.clone(), (client_info, tx));
            }

            let write_task = tokio::spawn(handle_outgoing_messages(write, rx, client_id.clone()));
            let read_task = tokio::spawn(handle_incoming_messages(
                read,
                client_id.clone(),
                clients.clone(),
                app_handle.clone(),
            ));

            // Wait for either task to complete
            tokio::select! {
                _ = write_task => {},
                _ = read_task => {},
            }
        }
        Err(e) => {
            eprintln!("Failed to accept WebSocket connection: {}", e);
        }
    }

    // Cleanup: remove client from map
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&client_id);
    }

    println!("Client {} disconnected", client_id);
}

pub async fn handle_outgoing_messages(
    mut write: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut rx: mpsc::UnboundedReceiver<Message>,
    client_id: String,
) {
    while let Some(message) = rx.recv().await {
        if let Err(e) = write.send(message).await {
            eprintln!("Error sending message to client {}: {}", client_id, e);
            break;
        }
    }
}
