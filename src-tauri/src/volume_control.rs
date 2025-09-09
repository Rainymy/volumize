use std::{
    sync::{
        mpsc::{channel, RecvError, Sender},
        Arc, Mutex,
    },
    thread::{spawn, JoinHandle},
};

use crate::types::shared::{
    AppIdentifier, AudioDevice, AudioSession, VolumeControllerTrait, VolumePercent, VolumeResult,
};

#[cfg(target_os = "windows")]
#[path = "win32/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/linux.rs"]
mod platform;

pub enum VolumeCommand {
    // Master
    SetMasterVolume(VolumePercent),
    GetMasterVolume(Sender<VolumeResult<Option<VolumePercent>>>),
    MuteMaster,
    UnmuteMaster,
    // Application
    GetAllApplications(Sender<VolumeResult<Vec<AudioSession>>>),
    GetAppVolume(AppIdentifier, Sender<VolumePercent>),
    SetAppVolume(AppIdentifier, VolumePercent),
    MuteApp(AppIdentifier),
    UnmuteApp(AppIdentifier),
    // DeviceControl
    GetCurrentPlaybackDevice(Sender<VolumeResult<AudioDevice>>),
    GetPlaybackDevices(Sender<VolumeResult<Vec<AudioDevice>>>),
}

pub struct VolumeCommandSender {
    pub tx: Arc<Mutex<Sender<VolumeCommand>>>,
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
            let (new_tx, _) = channel::<VolumeCommand>();
            let old_tx = std::mem::replace(&mut *tx_guard, new_tx);
            drop(old_tx); // Explicitly drop the sender
        }
    }
}

pub fn spawn_volume_thread() -> VolumeCommandSender {
    let (tx, rx) = channel::<VolumeCommand>();

    let thread_handle = spawn(move || {
        let controller = platform::make_controller().expect("Failed to initialize");

        loop {
            match rx.recv() {
                Ok(command) => {
                    let _ = execute_command(command, &controller);
                }
                Err(RecvError) => {
                    println!("Command channel disconnected/closed, shutting down thread");
                    break;
                }
            }
        }
    });

    VolumeCommandSender {
        tx: Arc::new(Mutex::new(tx)),
        thread_handle: Arc::new(Mutex::new(Some(thread_handle))),
    }
}

fn execute_command(command: VolumeCommand, controller: &Box<dyn VolumeControllerTrait>) {
    match command {
        // Master Controll
        VolumeCommand::SetMasterVolume(p) => {
            let _ = controller.set_master_volume(p);
        }
        VolumeCommand::GetMasterVolume(resp_tx) => {
            let volume = controller.get_master_volume();
            let _ = resp_tx.send(volume);
        }
        VolumeCommand::MuteMaster => {
            let _ = controller.mute_master();
        }
        VolumeCommand::UnmuteMaster => {
            let _ = controller.unmute_master();
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
