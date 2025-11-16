use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::types::shared::{
    AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, VolumeControllerError,
    VolumeControllerTrait, VolumePercent, VolumeResult,
};

#[cfg(target_os = "windows")]
#[path = "../win32/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "../linux/linux.rs"]
mod platform;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VolumeCommand {
    // Device
    GetAllDevices(
        #[serde(skip, default = "default_sender")]
        UnboundedSender<VolumeResult<Vec<DeviceIdentifier>>>,
    ),
    SetDeviceVolume(DeviceIdentifier, VolumePercent),
    GetDeviceVolume(
        DeviceIdentifier,
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<VolumePercent>>,
    ),
    MuteDevice(DeviceIdentifier),
    UnmuteDevice(DeviceIdentifier),
    // Application
    GetApplicationIcon(
        AppIdentifier,
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<Vec<u8>>>,
    ),
    GetDeviceApplications(
        DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        UnboundedSender<VolumeResult<Vec<AppIdentifier>>>,
    ),
    GetApplication(
        AppIdentifier,
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<AudioApplication>>,
    ),
    GetAppVolume(
        AppIdentifier,
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<VolumePercent>>,
    ),
    SetAppVolume(AppIdentifier, VolumePercent),
    MuteApp(AppIdentifier),
    UnmuteApp(AppIdentifier),
    // DeviceControl
    GetCurrentPlaybackDevice(
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<AudioDevice>>,
    ),
    GetPlaybackDevices(
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<Vec<AudioDevice>>>,
    ),
}
fn default_sender<T>() -> UnboundedSender<T> {
    unbounded_channel().0
}

impl VolumeCommand {
    pub fn get_name(&self) -> String {
        if let Ok(name) = serde_json::to_value(&self) {
            if let Some(obj) = name.as_object() {
                for key in obj.keys() {
                    return key.to_string();
                }
            }

            if let Some(name) = name.as_str() {
                return name.to_string();
            }
        }

        match serde_json::to_string(&self) {
            Ok(name) => name,
            Err(_) => "unknown_name".into(),
        }
    }
}

pub struct VolumeCommandSender {
    pub tx: Arc<Mutex<UnboundedSender<VolumeCommand>>>,
    pub thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl VolumeCommandSender {
    pub fn send(&self, cmd: VolumeCommand) -> Result<(), String> {
        if let Ok(tx_guard) = self.tx.lock() {
            tx_guard
                .send(cmd)
                .map_err(|e| format!("Send failed: {}", e))
        } else {
            Err("Failed to lock sender".to_string())
        }
    }

    pub fn close_channel(&self) {
        if let Ok(mut tx_guard) = self.tx.lock() {
            // Replace the sender with a new one and drop the original
            let (new_tx, _) = unbounded_channel::<VolumeCommand>();
            let old_tx = std::mem::replace(&mut *tx_guard, new_tx);
            drop(old_tx); // Explicitly drop the sender
        }
    }

    pub fn shutdown(&self) -> Result<(), String> {
        self.close_channel();

        let mut handle = self
            .thread_handle
            .lock()
            .map_err(|e| format!("Failed to lock thread handle: {}", e))?;

        if let Some(join_handle) = handle.take() {
            join_handle
                .join()
                .map_err(|e| format!("Volume thread panicked during shutdown: {:?}", e))?;
        }

        Ok(())
    }
}

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
        VolumeCommand::GetAllDevices(resp_tx) => {
            let _ = resp_tx.send(controller.get_all_devices());
        }
        VolumeCommand::SetDeviceVolume(device_id, p) => {
            let _ = controller.set_device_volume(device_id, p);
        }
        VolumeCommand::GetDeviceVolume(device_id, resp_tx) => {
            let _ = resp_tx.send(controller.get_device_volume(device_id));
        }
        VolumeCommand::MuteDevice(device_id) => {
            let _ = controller.mute_device(device_id);
        }
        VolumeCommand::UnmuteDevice(device_id) => {
            let _ = controller.unmute_device(device_id);
        }
        // Application Controll
        VolumeCommand::GetApplicationIcon(app_id, icon_tx) => {
            let error = VolumeControllerError::ApplicationNotFound("Application not found".into());

            let get_app = match controller.get_application(app_id) {
                Ok(app) => app,
                Err(_) => {
                    let _ = icon_tx.send(Err(error));
                    return ();
                }
            };

            let path = match get_app.process.path {
                Some(path) => path,
                None => PathBuf::new(),
            };

            let error = VolumeControllerError::Unknown("Could not extract icon from path.".into());
            let app_icon = platform::extract_icon(path).ok_or(error);
            let _ = icon_tx.send(app_icon);
        }
        VolumeCommand::GetApplication(app_id, resp_tx) => {
            let _ = resp_tx.send(controller.get_application(app_id));
        }
        VolumeCommand::GetDeviceApplications(device_id, resp_tx) => {
            let _ = resp_tx.send(controller.get_device_applications(device_id));
        }
        VolumeCommand::SetAppVolume(app_id, volume) => {
            let _ = controller.set_app_volume(app_id, volume);
        }
        VolumeCommand::GetAppVolume(app_id, resp_tx) => {
            let result = controller.get_app_volume(app_id).unwrap_or_default();
            let _ = resp_tx.send(Ok(result.current));
        }
        VolumeCommand::UnmuteApp(app_id) => {
            let _ = controller.unmute_app(app_id);
        }
        VolumeCommand::MuteApp(app_id) => {
            let _ = controller.mute_app(app_id);
        }
        // Deevice Controll
        VolumeCommand::GetPlaybackDevices(resp_tx) => {
            let _ = resp_tx.send(controller.get_playback_devices());
        }
        VolumeCommand::GetCurrentPlaybackDevice(resp_tx) => {
            let _ = resp_tx.send(controller.get_current_playback_device());
        }
    }
}
