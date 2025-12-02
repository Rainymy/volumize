use futures_util::future::{select, Either};
use serde_json::json;
use std::sync::mpsc::{Receiver, Sender};
use std::{path::PathBuf, time::Duration};
use tauri::{async_runtime as rt, AppHandle, Emitter, EventTarget, Manager};
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::interval;

use crate::server::WebSocketServerState;
use crate::types::shared::UPDATE_EVENT_NAME;
use crate::{
    platform,
    types::{
        shared::{UpdateChange, VolumeControllerError, VolumeControllerTrait},
        volume::{VolumeCommand, VolumeCommandSender, VolumeServer},
    },
};

pub fn spawn_volume_thread(app_handle: &AppHandle, sender: Sender<UpdateChange>) {
    let (tx, mut rx) = unbounded_channel::<VolumeCommand>();

    let thread_handle = std::thread::spawn(move || {
        let controller = platform::make_controller(sender);

        rt::block_on(async move {
            let mut interval = interval(Duration::from_millis(3000));
            interval.tick().await; // Skip the first immediate tick.
            println!("Main loop starting");

            let mut count = 1;
            loop {
                match select(Box::pin(interval.tick()), Box::pin(rx.recv())).await {
                    Either::Left(_) => {
                        println!("Periodic check: {}", count);
                        // if count >= 20 {
                        //     break; // Temp: Exit after 20 checks
                        // };
                        count += 1;
                        controller.check_and_reinit();
                    }
                    Either::Right((command_result, _)) => match command_result {
                        Some(command) => execute_command(command, &controller),
                        None => break,
                    },
                }
            }

            controller.cleanup();
        });

        println!("Thread ended")
    });

    let new_server = VolumeServer {
        tx: tx,
        thread_handle: Some(thread_handle),
    };

    let state = app_handle.state::<VolumeCommandSender>();
    let current_server = match state.server.lock() {
        Ok(mut current) => current.replace(new_server),
        Err(_) => None,
    };

    if let Some(mut old) = current_server {
        let _ = old
            .shutdown()
            .inspect_err(|e| eprintln!("Shutdown error: {}", e));
    }
}

pub fn spawn_update_thread(app_handle: &AppHandle, sender: Receiver<UpdateChange>) {
    let app_handle = app_handle.clone();

    std::thread::spawn(move || {
        while let Ok(msg) = sender.recv() {
            println!("sending: {:?}", msg);

            // ==================== SEND TO WEBVIEW ====================
            let target_event = EventTarget::WebviewWindow {
                label: "main".into(),
            };
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
            // ====================== RECEIVE END ======================
        }
        println!("Closing update thread");
    });
}

fn execute_command(command: VolumeCommand, controller: &Box<dyn VolumeControllerTrait>) {
    match command {
        // Master Controll
        VolumeCommand::GetPlaybackDevices { sender, .. } => {
            let _ = sender.send(controller.get_playback_devices());
        }
        VolumeCommand::DeviceSetVolume { id, volume, .. } => {
            let _ = controller.set_device_volume(id, volume);
        }
        VolumeCommand::DeviceGetVolume { id, sender, .. } => {
            let _ = sender.send(controller.get_device_volume(id));
        }
        VolumeCommand::DeviceMute { id, .. } => {
            let _ = controller.mute_device(id);
        }
        VolumeCommand::DeviceUnmute { id, .. } => {
            let _ = controller.unmute_device(id);
        }
        // Application Controll
        VolumeCommand::ApplicationGetIcon { id, sender, .. } => {
            let error = VolumeControllerError::ApplicationNotFound("Application not found".into());

            let get_app = match controller.get_application(id) {
                Ok(app) => app,
                Err(_) => {
                    let _ = sender.send(Err(error));
                    return ();
                }
            };

            let path = match get_app.process.path {
                Some(path) => path,
                None => PathBuf::new(),
            };

            let error = VolumeControllerError::Unknown("Could not extract icon from path.".into());
            let app_icon = platform::extract_icon(path).ok_or(error);
            let _ = sender.send(app_icon);
        }
        VolumeCommand::GetApplication { id, sender, .. } => {
            let _ = sender.send(controller.get_application(id));
        }
        VolumeCommand::GetDeviceApplications { id, sender, .. } => {
            let _ = sender.send(controller.get_device_applications(id));
        }
        VolumeCommand::ApplicationSetVolume { id, volume, .. } => {
            let _ = controller.set_app_volume(id, volume);
        }
        VolumeCommand::ApplicationGetVolume { id, sender, .. } => {
            let result = controller.get_app_volume(id).unwrap_or_default();
            let _ = sender.send(Ok(result.current));
        }
        VolumeCommand::ApplicationUnmute { id, .. } => {
            let _ = controller.unmute_app(id);
        }
        VolumeCommand::ApplicationMute { id, .. } => {
            let _ = controller.mute_app(id);
        }
    }
}
