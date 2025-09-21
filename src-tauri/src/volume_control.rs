use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::types::shared::{
    AppIdentifier, AudioDevice, AudioSession, DeviceIdentifier, VolumeControllerTrait,
    VolumePercent, VolumeResult,
};

#[cfg(target_os = "windows")]
#[path = "win32/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/linux.rs"]
mod platform;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VolumeCommand {
    // Device
    SetDeviceVolume(DeviceIdentifier, VolumePercent),
    GetDeviceVolume(
        DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        UnboundedSender<VolumeResult<Option<VolumePercent>>>,
    ),
    MuteDevice(DeviceIdentifier),
    UnmuteDevice(DeviceIdentifier),
    // Application
    GetAllApplications(
        #[serde(skip, default = "default_sender")] UnboundedSender<VolumeResult<Vec<AudioSession>>>,
    ),
    GetAppVolume(
        AppIdentifier,
        #[serde(skip, default = "default_sender2")] UnboundedSender<VolumePercent>,
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
fn default_sender<T>() -> UnboundedSender<VolumeResult<T>> {
    unbounded_channel().0
}
fn default_sender2<T>() -> UnboundedSender<T> {
    unbounded_channel().0
}

pub struct VolumeCommandSender {
    pub tx: UnboundedSender<VolumeCommand>,
    pub thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl VolumeCommandSender {
    pub fn send(&self, cmd: VolumeCommand) -> Result<(), String> {
        self.tx.send(cmd).map_err(|e| format!("Send failed: {}", e))
    }

    pub fn close_channel(&self) {
        let (new_tx, _) = unbounded_channel::<VolumeCommand>();
        let old_tx = std::mem::replace(&mut self.tx.clone(), new_tx);
        drop(old_tx);
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
            .enable_all()
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
        tx: tx,
        thread_handle: Arc::new(Mutex::new(Some(thread_handle))),
    }
}

fn execute_command(command: VolumeCommand, controller: &Box<dyn VolumeControllerTrait>) {
    match command {
        // Master Controll
        VolumeCommand::SetDeviceVolume(device_id, p) => {
            let _ = controller.set_device_volume(device_id, p);
        }
        VolumeCommand::GetDeviceVolume(device_id, resp_tx) => {
            let volume = controller.get_device_volume(device_id);
            let _ = resp_tx.send(volume);
        }
        VolumeCommand::MuteDevice(device_id) => {
            let _ = controller.mute_device(device_id);
        }
        VolumeCommand::UnmuteDevice(device_id) => {
            let _ = controller.unmute_device(device_id);
        }
        // Application Controll
        VolumeCommand::GetAllApplications(resp_tx) => {
            let all_app = controller.get_all_applications();
            let _ = resp_tx.send(all_app);
        }
        VolumeCommand::SetAppVolume(app_id, volume) => {
            let _ = controller.set_app_volume(app_id, volume);
        }
        VolumeCommand::GetAppVolume(app_id, resp_tx) => {
            if let Ok(app_volume) = controller.get_app_volume(app_id) {
                let _ = resp_tx.send(app_volume.current);
            } else {
                let _ = resp_tx.send(0.0);
            }
        }
        VolumeCommand::UnmuteApp(app_id) => {
            let _ = controller.unmute_app(app_id);
        }
        VolumeCommand::MuteApp(app_id) => {
            let _ = controller.mute_app(app_id);
        }
        // Deevice Controll
        VolumeCommand::GetPlaybackDevices(resp_tx) => {
            let current_device = controller.get_playback_devices();
            let _ = resp_tx.send(current_device);
        }
        VolumeCommand::GetCurrentPlaybackDevice(resp_tx) => {
            let current_device = controller.get_current_playback_device();
            let _ = resp_tx.send(current_device);
        }
    }
}
