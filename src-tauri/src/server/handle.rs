use std::net::SocketAddr;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tauri::async_runtime as rt;
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

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to accept WebSocket connection: {}", e);
            return;
        }
    };

    println!("Client {} connected from {}", client_id, peer_addr);

    let (write, read) = ws_stream.split();
    let (tx, rx) = mpsc::unbounded_channel();

    // Add client to the map
    clients.lock().await.insert(
        client_id.clone(),
        (
            ClientInfo {
                id: client_id.clone(),
                address: peer_addr.to_string(),
            },
            tx,
        ),
    );

    let mut write_task = rt::spawn(handle_outgoing_messages(write, rx, client_id.clone()));
    let mut read_task = rt::spawn(handle_incoming_messages(
        read,
        client_id.clone(),
        clients.clone(),
        app_handle,
    ));

    let _ = tokio::select! {
        _ = &mut write_task => { read_task.abort();  read_task.await },
        _ = &mut read_task  => { write_task.abort(); write_task.await },
    };

    // Cleanup: remove client from map
    clients.lock().await.remove(&client_id);
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
