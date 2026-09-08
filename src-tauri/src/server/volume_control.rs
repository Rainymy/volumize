use futures_util::future::{select, Either};
use serde_json::json;
use shared_types::protocol::{
    Command, CommandRequest, CommandResponse, Envelope, RawFrame, Response,
};
use shared_types::{Identifier, UpdateChange};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tauri::{async_runtime as rt, AppHandle, Emitter, EventTarget, Manager};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::interval;

use crate::server::serial::SerialState;
use crate::server::websocket::WebSocketServerState;
use crate::types::shared::UPDATE_EVENT_NAME;
use crate::types::volume::CommandClient;
use crate::{
    platform,
    types::{
        shared::VolumeControllerTrait,
        volume::{VolumeCommandSender, VolumeServer},
    },
};

pub fn spawn_volume_thread(app_handle: &AppHandle, sender: Sender<UpdateChange>) {
    let (tx, mut rx) = unbounded_channel::<Envelope>();
    let (response_tx, response_rx) = unbounded_channel::<Envelope>();
    let handle_clone = app_handle.clone();

    let thread_handle = std::thread::spawn(move || {
        let controller = platform::make_controller(sender);

        rt::block_on(async move {
            let mut interval = interval(Duration::from_millis(3000));
            interval.tick().await; // Skip the first immediate tick.
            println!("[spawn_volume_thread] Main loop starting");

            let mut count = 1;
            loop {
                match select(Box::pin(interval.tick()), Box::pin(rx.recv())).await {
                    Either::Left(_) => {
                        println!("[spawn_volume_thread] Periodic check: {}", count);
                        count += 1;
                        controller.check_and_reinit();
                    }
                    Either::Right((command_result, _)) => match command_result {
                        Some(command) => {
                            execute_command(&handle_clone, command, &controller, &response_tx)
                        }
                        None => break,
                    },
                }
            }

            controller.cleanup();
        });

        println!("[spawn_volume_thread] Thread ended")
    });

    let new_server = VolumeServer {
        tx: tx,
        thread_handle: Some(thread_handle),
    };

    let new_client = CommandClient::new(new_server.tx.clone(), response_rx);

    let state = app_handle.state::<VolumeCommandSender>();
    let current_server = match state.server.lock() {
        Ok(mut current) => current.replace(new_server),
        Err(_) => None,
    };

    let current_client = state.client.blocking_lock().replace(new_client);
    if let Some(mut old_client) = current_client {
        old_client.shutdown();
    }
    if let Some(mut old) = current_server {
        let _ = old
            .shutdown()
            .inspect_err(|e| eprintln!("[spawn_volume_thread] Shutdown error: {}", e));
    }
}

pub fn spawn_update_thread(app_handle: &AppHandle, sender: Receiver<UpdateChange>) {
    let app_handle = app_handle.clone();

    std::thread::spawn(move || {
        while let Ok(msg) = sender.recv() {
            println!("sending: {:?}", msg);

            // ==================== SEND TO WEBVIEW ====================
            let target_event = EventTarget::labeled("volume-control-panel");
            let result = app_handle.emit_to(target_event, UPDATE_EVENT_NAME, &msg);
            if let Err(err) = result {
                eprintln!("Error emitting update event: {}", err);
            }
            // =============== SEND TO WEBSOCKET CLIENTS ===============
            let event_str = json! ({
                "event": UPDATE_EVENT_NAME,
                "payload": &msg
            })
            .to_string();
            let websocket_server = app_handle.state::<WebSocketServerState>();
            let clients = websocket_server.clients.blocking_lock();
            for (_id, client) in clients.iter() {
                let _ = client.1.send(event_str.clone().into());
            }
            // ================ SEND TO SERIAL CLIENTS =================
            let serial_state = app_handle.state::<SerialState>();
            let serial_clients = serial_state.server.blocking_lock();
            if let Some(serial_server) = serial_clients.as_ref() {
                let buffer = RawFrame::encode(&Envelope::Event(msg)).build();
                let _ = serial_server.sender.send(buffer);
            }
            // ====================== RECEIVE END ======================
        }
        println!("Closing update thread");
    });
}

fn execute_command(
    _handle: &AppHandle,
    envelope: Envelope,
    controller: &Box<dyn VolumeControllerTrait>,
    response_tx: &UnboundedSender<Envelope>,
) {
    let Envelope::Command(CommandRequest { id, command }) = envelope else {
        return; // ignore anything that isn't a command
    };

    let response = handle_command(command, controller);
    let _ = response_tx.send(Envelope::Response(CommandResponse { id, response }));
}

fn handle_command(command: Command, controller: &Box<dyn VolumeControllerTrait>) -> Response {
    match command {
        Command::GetApplication { app_id } => match controller.get_application(app_id) {
            Ok(app) => Response::Application(app),
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },

        Command::GetApplications { device_id } => {
            match controller.get_device_applications(device_id.clone()) {
                Ok(apps) => Response::ApplicationList { device_id, apps },
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            }
        }

        Command::GetIcon { app_id } => {
            let app = match controller.get_application(app_id) {
                Ok(app) => app,
                Err(e) => {
                    return Response::Error {
                        message: e.to_string(),
                    }
                }
            };

            let path = app.process.path.unwrap_or_default();
            let data = platform::extract_icon(path).unwrap_or_default();
            Response::Icon { app_id, data }
        }

        Command::GetPlaybackDevices => match controller.get_playback_devices() {
            Ok(devices) => Response::DeviceList(devices),
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },

        Command::GetVolume { id } => {
            let result = match id.clone() {
                Identifier::App(app_id) => controller.get_app_volume(app_id),
                Identifier::Device(device_id) => controller.get_device_volume(device_id),
            };
            match result {
                Ok(volume) => Response::Volume { id, volume },
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            }
        }

        Command::SetMute { id, mute } => {
            let result = match id {
                Identifier::App(app_id) => {
                    if mute {
                        controller.mute_app(app_id)
                    } else {
                        controller.unmute_app(app_id)
                    }
                }
                Identifier::Device(device_id) => {
                    if mute {
                        controller.mute_device(device_id)
                    } else {
                        controller.unmute_device(device_id)
                    }
                }
            };
            match result {
                Ok(_) => Response::ACK,
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            }
        }

        Command::SetVolume { id, volume } => {
            let result = match id {
                Identifier::App(app_id) => controller.set_app_volume(app_id, volume),
                Identifier::Device(device_id) => controller.set_device_volume(device_id, volume),
            };
            match result {
                Ok(_) => Response::ACK,
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            }
        }
    }
}
