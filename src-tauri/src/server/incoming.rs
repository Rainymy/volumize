use std::error::Error;

use futures_util::{stream::SplitStream, StreamExt};
use serde_json::json;
use shared_types::protocol::{CommandRequest, CommandResponse, Envelope};
use tauri::{AppHandle, Manager};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tokio_util::bytes::Bytes;

use crate::server::websocket::ClientMap;
use crate::types::volume::VolumeCommandSender;

pub async fn handle_incoming_messages(
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    client_id: String,
    clients: ClientMap,
    app_handle: AppHandle,
) {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => match parse_action(&text) {
                Ok(command) => {
                    if let Err(error) =
                        handle_volume_command(command, &client_id, &clients, &app_handle).await
                    {
                        eprintln!("Failed to handle volume command: {}", error)
                    }
                }
                Err(err) => eprintln!("Parse error: {}\n - Original: {}", err, text),
            },
            Ok(Message::Close(_)) => {
                println!("Client {} closed connection", client_id);
                break;
            }
            Ok(Message::Ping(_)) => {
                let client_lock = clients.lock().await;
                if let Some((_, client_sender)) = client_lock.get(&client_id) {
                    let _ = client_sender.send(Message::Pong(Bytes::new()));
                }
            }
            Ok(data) => {
                eprintln!("Unexpected message type from {}: {:?}", client_id, data);
                break;
            }
            Err(e) => {
                eprintln!("WebSocket error for client {}: {}", client_id, e);
                break;
            }
        }
    }
}

async fn handle_volume_command(
    envelope: Envelope,
    client_id: &str,
    clients: &ClientMap,
    app_handle: &AppHandle,
) -> Result<(), Box<dyn Error>> {
    let Envelope::Command(CommandRequest { id, command }) = envelope else {
        return Ok(());
    };

    let state = app_handle.state::<VolumeCommandSender>();
    let response = state.request(&command).await?;

    let client_lock = clients.lock().await;
    let (_, client_sender) = client_lock.get(client_id).ok_or("Client not found")?;

    let outgoing = Envelope::Response(CommandResponse { id, response });

    let respons = create_json_response("volume", &outgoing);
    client_sender.send(respons.into()).map_err(|e| e.into())
}

fn create_json_response<T: serde::Serialize>(name: &str, data: &T) -> String {
    json!({
        "type": name,
        "data": data
    })
    .to_string()
}

fn parse_action(action: &str) -> Result<Envelope, serde_json::Error> {
    println!("Parsing action: {}", action);
    serde_json::from_str::<Envelope>(action)
}
