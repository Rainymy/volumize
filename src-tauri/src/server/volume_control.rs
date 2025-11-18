use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    platform,
    types::{
        shared::{VolumeControllerError, VolumeControllerTrait},
        volume::{VolumeCommand, VolumeCommandSender},
    },
};

pub fn spawn_volume_thread() -> VolumeCommandSender {
    let (tx, mut rx) = unbounded_channel::<VolumeCommand>();

    let thread_handle = std::thread::spawn(move || {
        let controller = match platform::make_controller() {
            Ok(c) => c,
            Err(err) => {
                eprintln!("Failed to initialize, {}", err);
                return;
            }
        };

        let rt = match tokio::runtime::Builder::new_current_thread()
            .thread_name("tokie_spawn_volume_thread")
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Failed to create tokio runtime: {:?}", e);
                return;
            }
        };

        rt.block_on(async move {
            while let Some(command) = rx.recv().await {
                execute_command(command, &controller);
            }
        });
    });

    VolumeCommandSender {
        tx: Arc::new(Mutex::new(tx)),
        thread_handle: Arc::new(Mutex::new(Some(thread_handle))),
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
