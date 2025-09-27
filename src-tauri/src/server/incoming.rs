use futures_util::{stream::SplitStream, StreamExt};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::{net::TcpStream, sync::mpsc::unbounded_channel};
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
    _app_handle: AppHandle,
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
                        let v_state = _app_handle.state::<VolumeCommandSender>();
                        if let Some(c_client) = clients.lock().await.get(&client_id) {
                            match command {
                                VolumeCommand::GetAllApplications(mut sender) => {
                                    let (tx, mut rx) = unbounded_channel::<VolumeResult<_>>();
                                    sender.clone_from(&tx);
                                    let temp = VolumeCommand::GetAllApplications(sender);
                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(temp);
                                    };

                                    let temp2 = VolumeCommand::GetAllApplications(tx.clone());
                                    if let Some(r_value) = rx.recv().await {
                                        let value = json!({
                                            "type": temp2.get_name(),
                                            "data": r_value.unwrap_or_default()
                                        })
                                        .to_string();
                                        let _ = c_client.1.send(Message::Text(value.into()));
                                    };
                                }
                                VolumeCommand::GetDeviceVolume(a, mut sender) => {
                                    let (tx, mut rx) = unbounded_channel::<VolumeResult<_>>();
                                    sender.clone_from(&tx);

                                    let temp = VolumeCommand::GetDeviceVolume(a.clone(), sender);

                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(temp);
                                    };

                                    let temp2 = VolumeCommand::GetDeviceVolume(a, tx.clone());
                                    if let Some(r_value) = rx.recv().await {
                                        if let Ok(value) = r_value {
                                            let value = json!({
                                                "type": temp2.get_name(),
                                                "data": value
                                            })
                                            .to_string();

                                            let _ = c_client.1.send(value.into());
                                        }
                                    };
                                }
                                VolumeCommand::GetAppVolume(a, mut unbounded_sender) => {
                                    let (tx, mut rx) = unbounded_channel::<_>();
                                    unbounded_sender.clone_from(&tx);
                                    let temp = VolumeCommand::GetAppVolume(a, unbounded_sender);
                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(temp);
                                    };

                                    let temp2 = VolumeCommand::GetAppVolume(a, tx.clone());
                                    let value = rx.recv().await;
                                    let s_value = json!({
                                        "type": temp2.get_name(),
                                        "data": value.unwrap_or_default()
                                    })
                                    .to_string();

                                    let _ = c_client.1.send(s_value.into());
                                }
                                VolumeCommand::GetCurrentPlaybackDevice(mut sender) => {
                                    let (tx, mut rx) = unbounded_channel::<VolumeResult<_>>();
                                    sender.clone_from(&tx);
                                    let temp = VolumeCommand::GetCurrentPlaybackDevice(sender);
                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(temp);
                                    };

                                    let temp2 = VolumeCommand::GetCurrentPlaybackDevice(tx.clone());
                                    if let Some(r_value) = rx.recv().await {
                                        if let Ok(value) = r_value {
                                            let s_value = json!({
                                                "type": temp2.get_name(),
                                                "data": value
                                            })
                                            .to_string();

                                            let _ = c_client.1.send(s_value.into());
                                        }
                                    };
                                }
                                VolumeCommand::GetPlaybackDevices(mut unbounded_sender) => {
                                    let (tx, mut rx) = unbounded_channel::<VolumeResult<_>>();
                                    unbounded_sender.clone_from(&tx);

                                    let temp = VolumeCommand::GetPlaybackDevices(unbounded_sender);
                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(temp);
                                    };

                                    let temp2 = VolumeCommand::GetPlaybackDevices(tx.clone());
                                    if let Some(r_value) = rx.recv().await {
                                        let s_value = json!({
                                            "type": temp2.get_name(),
                                            "data": r_value.unwrap_or_default()
                                        })
                                        .to_string();

                                        let _ = c_client.1.send(s_value.into());
                                    };
                                }
                                rest => {
                                    if let Ok(v_send_error) = v_state.tx.lock() {
                                        let _ = v_send_error.send(rest);
                                    };
                                }
                            };
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
