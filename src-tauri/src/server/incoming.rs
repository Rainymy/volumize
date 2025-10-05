use std::error::Error;

use futures_util::{stream::SplitStream, StreamExt};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    types::shared::VolumeResult,
    volume_control::{VolumeCommand, VolumeCommandSender},
};

use super::ClientMap;

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
    command: VolumeCommand,
    client_id: &str,
    clients: &ClientMap,
    app_handle: &AppHandle,
) -> Result<(), Box<dyn Error>> {
    let client_lock = clients.lock().await;
    let (_, client_sender) = client_lock.get(client_id).ok_or("Client not found")?;

    let state = app_handle.state::<VolumeCommandSender>();

    match command {
        VolumeCommand::GetDeviceVolume(x, _) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_response(
                VolumeCommand::GetDeviceVolume(x, tx),
                &client_sender,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetAppVolume(x, _) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_response(
                VolumeCommand::GetAppVolume(x, tx),
                &client_sender,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetAllApplications(_) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_response(
                VolumeCommand::GetAllApplications(tx),
                &client_sender,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetCurrentPlaybackDevice(_) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_response(
                VolumeCommand::GetCurrentPlaybackDevice(tx),
                &client_sender,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetPlaybackDevices(_) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_response(
                VolumeCommand::GetPlaybackDevices(tx),
                &client_sender,
                &state,
                rx,
            )
            .await
        }
        rest => send_command(rest, &state),
    }
}

fn send_command(
    command: VolumeCommand,
    v_state: &VolumeCommandSender,
) -> Result<(), Box<dyn Error>> {
    let tx = v_state
        .tx
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    tx.send(command)
        .map_err(|e| format!("Send error: {}", e).into())
}

async fn handle_command_with_response<T: serde::Serialize>(
    command: VolumeCommand,
    client_sender: &UnboundedSender<Message>,
    v_state: &VolumeCommandSender,
    mut rx: UnboundedReceiver<VolumeResult<T>>,
) -> Result<(), Box<dyn Error>> {
    let command_name = command.get_name();

    send_command(command, v_state)?;

    match rx.recv().await {
        Some(Ok(result)) => {
            let respons = create_json_response(&command_name, &result);
            client_sender
                .send(respons.into())
                .map_err(|e| e.to_string().into())
        }
        Some(Err(err)) => Err(err.into()),
        None => Err("Response channel closed".into()),
    }
}

fn create_json_response<T: serde::Serialize>(name: &str, data: &T) -> String {
    json!({
        "type": name,
        "data": data
    })
    .to_string()
}

fn parse_action(action: &str) -> Result<VolumeCommand, serde_json::Error> {
    serde_json::from_str::<VolumeCommand>(action)
}
