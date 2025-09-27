use std::error::Error;

use futures_util::{stream::SplitStream, StreamExt};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::{
    net::TcpStream,
    sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    types::shared::VolumeResult,
    volume_control::{VolumeCommand, VolumeCommandSender},
};

use super::{ClientMap, ServerMessage};

pub async fn handle_incoming_messages(
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    client_id: String,
    clients: ClientMap,
    app_handle: AppHandle,
) {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let data = text.to_string();

                let server_text_message = ServerMessage {
                    client_id: client_id.clone(),
                    data: data,
                };

                match parse_action(&server_text_message.data) {
                    Ok(command) => {
                        if let Err(error) =
                            handle_volume_command(command, &client_id, &clients, &app_handle).await
                        {
                            eprintln!("Failed to handle volume command: \n{:}", error)
                        }
                    }
                    Err(err) => eprintln!("{:}\n - original: {}", err, server_text_message.data),
                }
            }
            Ok(Message::Close(_frame)) => {
                println!("Client {} closed connection", client_id);
                break;
            }
            Ok(data) => {
                dbg!(data);
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
    let client = match client_lock.get(client_id) {
        Some(client) => client,
        None => return Err("No Channel Found".into()),
    };

    let state = app_handle.state::<VolumeCommandSender>();

    let result = match command {
        VolumeCommand::GetDeviceVolume(x, mut _sender) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_result_response(
                VolumeCommand::GetDeviceVolume(x, tx),
                &client.1,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetAppVolume(x, mut _sender) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_result_response(
                VolumeCommand::GetAppVolume(x, tx),
                &client.1,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetAllApplications(mut _sender) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_result_response(
                VolumeCommand::GetAllApplications(tx),
                &client.1,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetCurrentPlaybackDevice(mut _sender) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_result_response(
                VolumeCommand::GetCurrentPlaybackDevice(tx),
                &client.1,
                &state,
                rx,
            )
            .await
        }
        VolumeCommand::GetPlaybackDevices(mut _sender) => {
            let (tx, rx) = unbounded_channel();
            handle_command_with_result_response(
                VolumeCommand::GetPlaybackDevices(tx),
                &client.1,
                &state,
                rx,
            )
            .await
        }
        rest => send_command_to_volume_service(rest, &state),
    };

    result.map_err(|error| error.to_string().into())
}

fn send_command_to_volume_service(
    command: VolumeCommand,
    v_state: &VolumeCommandSender,
) -> Result<(), SendError<Message>> {
    let v_send = match v_state.tx.lock() {
        Ok(get_send) => get_send,
        Err(error) => return Err(SendError(error.to_string().into())),
    };

    v_send
        .send(command)
        .map_err(|error| SendError(error.to_string().into()))
}

async fn handle_command_with_result_response<T: serde::Serialize>(
    command: VolumeCommand,
    client_sender: &UnboundedSender<Message>,
    v_state: &VolumeCommandSender,
    mut rx: UnboundedReceiver<VolumeResult<T>>,
) -> Result<(), SendError<Message>> {
    let clone_command = command.clone();

    match send_command_to_volume_service(command, v_state) {
        Ok(msg) => msg,
        Err(error) => return Err(error),
    };

    match rx.recv().await {
        Some(result) => match result {
            Ok(data) => {
                let response = create_json_response(&clone_command, &data);
                client_sender.send(response.into())
            }
            Err(error) => Err(SendError(error.to_string().into())),
        },
        None => Ok(()),
    }
}

fn create_json_response<T: serde::Serialize>(command: &VolumeCommand, data: &T) -> String {
    json!({
        "type": command.get_name(),
        "data": data
    })
    .to_string()
}

fn parse_action(action: &str) -> Result<VolumeCommand, serde_json::Error> {
    serde_json::from_str::<VolumeCommand>(action)
}
