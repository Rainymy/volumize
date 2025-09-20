use futures_util::{stream::SplitStream, StreamExt};
use tauri::AppHandle;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::volume_control::VolumeCommand;

use super::{ClientMap, ServerMessage};

pub async fn handle_incoming_messages(
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    client_id: String,
    clients: ClientMap,
    _app_handle: AppHandle,
) {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let data = text.to_string();

                let _server_text_message = ServerMessage {
                    client_id: client_id.clone(),
                    data: data,
                };

                match parse_action(&_server_text_message.data) {
                    Ok(_command) => {
                        // dbg!(command);
                    }
                    Err(err) => eprintln!("{:}\n - original: {}", err, _server_text_message.data),
                }

                if let Some(this_client) = clients.lock().await.get(&client_id) {
                    let _send = this_client.1.send("hello client, i'm server".into());
                }
            }
            Ok(Message::Close(_frame)) => {
                println!("Client {} closed connection", client_id);
                break;
            }
            Ok(data) => {
                dbg!(data);
            }
            Err(e) => {
                eprintln!("WebSocket error for client {}: {}", client_id, e);
                break;
            }
        }
    }
}

fn parse_action(action: &str) -> Result<VolumeCommand, serde_json::Error> {
    serde_json::from_str::<VolumeCommand>(action)
}
