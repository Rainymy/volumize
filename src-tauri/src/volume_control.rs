use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread::{spawn, JoinHandle},
};

use crate::types::shared::{AppIdentifier, AudioDevice, AudioSession, VolumePercent, VolumeResult};

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
    // Thread - to close opened thread
    CloseThread,
}

pub struct VolumeCommandSender {
    pub tx: Sender<VolumeCommand>,
    pub thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

pub fn spawn_volume_thread() -> VolumeCommandSender {
    let (tx, rx) = channel::<VolumeCommand>();

    #[cfg(debug_assertions)]
    dbg!("Opening new thread!");

    let thread_handle = spawn(move || {
        let controller = platform::make_controller().expect("Failed to initialize");

        for command in rx {
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
                    let _ = controller.mute_app(app_id);
                }
                VolumeCommand::MuteApp(app_id) => {
                    let _ = controller.unmute_app(app_id);
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
                VolumeCommand::CloseThread => {
                    println!("Volume thread received CloseThread — exiting.");
                    break;
                }
            }
        }

        #[cfg(debug_assertions)]
        dbg!("Closing the thread!");
    });

    VolumeCommandSender {
        tx,
        thread_handle: Arc::new(Mutex::new(Some(thread_handle))),
    }
}
