use futures_util::future::{select, Either};
use std::{path::PathBuf, time::Duration};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::interval;

use crate::{
    platform,
    types::{
        shared::{VolumeControllerError, VolumeControllerTrait},
        volume::{VolumeCommand, VolumeCommandSender, VolumeServer},
    },
};

pub fn spawn_volume_thread(app_handle: &AppHandle) {
    let (tx, mut rx) = unbounded_channel::<VolumeCommand>();

    let state = app_handle.state::<VolumeCommandSender>();

    let thread_handle = std::thread::spawn(move || {
        let controller = platform::make_controller();
        let mut count = 0;

        tauri::async_runtime::block_on(async move {
            let mut interval = interval(Duration::from_millis(3000));
            interval.tick().await; // Skip the first immediate tick.

            println!("Main loop starting");

            loop {
                let interval = interval.tick();
                let recv = rx.recv();

                match select(Box::pin(interval), Box::pin(recv)).await {
                    Either::Left(_) => {
                        count += 1;
                        println!("Periodic check: {}", count);

                        if count >= 20 {
                            println!("Limit reached, breaking");
                            break;
                        }

                        controller.check_and_reinit();
                    }
                    Either::Right((command_result, _)) => match command_result {
                        Some(command) => execute_command(command, &controller),
                        None => {
                            println!("Channel closed, exiting");
                            break;
                        }
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

    let current_server = match state.server.lock() {
        Ok(mut current) => current.replace(new_server),
        Err(e) => {
            eprintln!("Failed to lock server: {:?}", e);
            return;
        }
    };

    if let Some(mut old) = current_server {
        let _ = old
            .shutdown()
            .inspect_err(|e| eprintln!("Shutdown error: {}", e));
    }
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
