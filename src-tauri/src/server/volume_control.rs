use shared_types::{
    protocol::{Command, CommandRequest, CommandResponse, Envelope, Response},
    Identifier, UpdateChange, UpdateChangeEvent,
};

use futures_util::future::{select, Either};
use std::sync::mpsc::{Receiver, Sender};
use tauri::{async_runtime as rt, AppHandle, Emitter, EventTarget, Manager};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::{interval, Duration};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    platform,
    server::{serial::SerialState, websocket::WebSocketServerState},
    types::{
        shared::{VolumeControllerTrait, UPDATE_EVENT_NAME, VOLUME_LABEL_EVENT},
        volume::{CommandClient, VolumeCommandSender, VolumeServer},
    },
};

pub fn spawn_volume_thread(app_handle: &AppHandle, sender: Sender<UpdateChange>) {
    let (tx, mut rx) = unbounded_channel::<Envelope>();
    let (response_tx, response_rx) = unbounded_channel::<Envelope>();

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
                        // Sanity check: periodically print to check if the thread is still running.
                        println!("[spawn_volume_thread] Periodic check: {}", count);
                        count += 1;
                        controller.check_and_reinit();
                    }
                    Either::Right((command_result, _)) => match command_result {
                        Some(command) => execute_command(command, &controller, &response_tx),
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
            println!("[spawn_update_thread] sending: {:?}", msg);

            // ==================== SEND TO WEBVIEW ====================
            let target_event = EventTarget::labeled(VOLUME_LABEL_EVENT);
            let result = app_handle.emit_to(target_event, UPDATE_EVENT_NAME, &msg);
            if let Err(err) = result {
                eprintln!("Error emitting update event: {}", err);
            }
            // =============== SEND TO WEBSOCKET CLIENTS ===============
            {
                let event = UpdateChangeEvent::new(UPDATE_EVENT_NAME, &msg);
                let event_str = serde_json::to_string(&event).unwrap_or_default();

                let websocket_server = app_handle.state::<WebSocketServerState>();
                let clients = websocket_server.clients.blocking_lock();

                for (_id, client) in clients.iter() {
                    if let Err(err) = client.1.send(Message::Text(event_str.clone().into())) {
                        eprintln!("Error sending update event to websocket client: {}", err);
                    }
                }
            }

            // ================ SEND TO SERIAL CLIENTS =================
            {
                let serial_state = app_handle.state::<SerialState>();
                let serial_clients = serial_state.server.blocking_lock();

                if let Some(serial_server) = serial_clients.as_ref() {
                    let _ = serial_server.sender.send(Envelope::Event(msg));
                    // if let Err(err) = serial_server.sender.send(buffer) {
                    //     eprintln!("Error sending update event to serial client: {}", err);
                    // }
                }
            }
            // ====================== RECEIVE END ======================
        }
        println!("Closing update thread");
    });
}

fn execute_command(
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
        Command::GetApplication { id } => match controller.get_application(id) {
            Ok(app) => Response::Application(app),
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },

        Command::GetApplications { id } => match controller.get_device_applications(id.clone()) {
            Ok(apps) => Response::ApplicationList { id, apps },
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },

        Command::GetIcon { id } => {
            let icon = match id {
                Identifier::App(id) => {
                    let app = match controller.get_application(id) {
                        Ok(app) => app,
                        Err(e) => {
                            return Response::Error {
                                message: e.to_string(),
                            }
                        }
                    };

                    if id == 0 {
                        platform::extract_system_icon()
                    } else {
                        platform::extract_icon(app.process.path.unwrap_or_default())
                    }
                }
                Identifier::Device(ref id) => {
                    let devices = controller.get_playback_devices().unwrap_or_default();
                    let devices = devices.into_iter().find(|d| d.id == *id);
                    match devices {
                        Some(device) => platform::extract_device_icon(device.id),
                        None => {
                            return Response::Error {
                                message: "Device not found".to_string(),
                            };
                        }
                    }
                }
            };
            Response::Icon {
                id,
                data: icon.unwrap_or_default(),
            }
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
