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
    GetAllDevices {
        request_id: String,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<DeviceIdentifier>>>,
    },
    SetDeviceVolume {
        request_id: String,
        id: DeviceIdentifier,
        volume: VolumePercent,
    },
    GetDeviceVolume {
        request_id: String,
        id: DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<VolumePercent>>,
    },
    MuteDevice {
        request_id: String,
        id: DeviceIdentifier,
    },
    UnmuteDevice {
        request_id: String,
        id: DeviceIdentifier,
    },
    // Application
    GetApplicationIcon {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<u8>>>,
    },
    GetDeviceApplications {
        request_id: String,
        id: DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<AppIdentifier>>>,
    },
    GetApplication {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<AudioApplication>>,
    },
    GetAppVolume {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<VolumePercent>>,
    },
    SetAppVolume {
        request_id: String,
        id: AppIdentifier,
        volume: VolumePercent,
    },
    MuteApp {
        request_id: String,
        id: AppIdentifier,
    },
    UnmuteApp {
        request_id: String,
        id: AppIdentifier,
    },
    // DeviceControl
    GetCurrentPlaybackDevice {
        request_id: String,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<AudioDevice>>,
    },
    GetPlaybackDevices {
        request_id: String,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<AudioDevice>>>,
    },
}
fn default_sender<T>() -> UnboundedSender<T> {
    unbounded_channel().0
}

impl VolumeCommand {
    pub fn get_name(&self) -> String {
        let serde_value = match serde_json::to_value(&self) {
            Ok(value) => value,
            Err(_) => serde_json::Value::Null,
        };

        let obj = match serde_value.as_object() {
            Some(value) => value,
            None => &serde_json::Map::new(),
        };

        for key in obj.keys() {
            return key.to_string();
        }

        if let Some(name) = serde_value.as_str() {
            return name.to_string();
        }

        match serde_json::to_string(&self) {
            Ok(name) => name,
            Err(_) => "unknown_name".into(),
        }
    }

    pub fn get_request_id(&self) -> String {
        let serde_value = match serde_json::to_value(&self) {
            Ok(value) => value,
            Err(_) => serde_json::Value::Null,
        };

        let obj = match serde_value.as_object() {
            Some(value) => value,
            None => &serde_json::Map::new(),
        };

        for value in obj.values() {
            let v_request_id = &value["request_id"];
            let id = match v_request_id.as_str() {
                Some(id) => id,
                None => continue,
            };

            return id.to_string();
        }

        String::new()
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
        VolumeCommand::GetAllDevices { sender, .. } => {
            let _ = sender.send(controller.get_all_devices());
        }
        VolumeCommand::SetDeviceVolume { id, volume, .. } => {
            let _ = controller.set_device_volume(id, volume);
        }
        VolumeCommand::GetDeviceVolume { id, sender, .. } => {
            let _ = sender.send(controller.get_device_volume(id));
        }
        VolumeCommand::MuteDevice { id, .. } => {
            let _ = controller.mute_device(id);
        }
        VolumeCommand::UnmuteDevice { id, .. } => {
            let _ = controller.unmute_device(id);
        }
        // Application Controll
        VolumeCommand::GetApplicationIcon { id, sender, .. } => {
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
        VolumeCommand::SetAppVolume { id, volume, .. } => {
            let _ = controller.set_app_volume(id, volume);
        }
        VolumeCommand::GetAppVolume { id, sender, .. } => {
            let result = controller.get_app_volume(id).unwrap_or_default();
            let _ = sender.send(Ok(result.current));
        }
        VolumeCommand::UnmuteApp { id, .. } => {
            let _ = controller.unmute_app(id);
        }
        VolumeCommand::MuteApp { id, .. } => {
            let _ = controller.mute_app(id);
        }
        // Deevice Controll
        VolumeCommand::GetPlaybackDevices { sender, .. } => {
            let _ = sender.send(controller.get_playback_devices());
        }
        VolumeCommand::GetCurrentPlaybackDevice { sender, .. } => {
            let _ = sender.send(controller.get_current_playback_device());
        }
    }
}
