use std::sync::mpsc::channel;
use tauri::State;

use crate::{
    types::shared::{AppIdentifier, AudioDevice, AudioSession, VolumePercent},
    volume_control::{VolumeCommand, VolumeCommandSender},
};

// ============================ Master ============================
#[tauri::command]
pub fn set_master_volume(percent: f32, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::SetMasterVolume(percent));
}

#[tauri::command]
pub fn get_master_volume(state: State<VolumeCommandSender>) -> Option<f32> {
    let (tx, rx) = channel();

    let _ = state.send(VolumeCommand::GetMasterVolume(tx));

    match rx.recv() {
        Ok(Ok(Some(percent))) => Some(percent),
        Ok(Ok(None)) => None,
        _ => None,
    }
}

#[tauri::command]
pub fn mute_master(state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::MuteMaster);
}

#[tauri::command]
pub fn unmute_master(state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::UnmuteMaster);
}

// ============================ Application ============================
#[tauri::command]
pub fn get_all_applications(state: State<VolumeCommandSender>) -> Vec<AudioSession> {
    let (tx, rx) = channel();

    let _ = state.send(VolumeCommand::GetAllApplications(tx));

    match rx.recv() {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => vec![],
        Err(_) => vec![],
    }
}

#[tauri::command]
pub fn get_app_volume(
    app_identifier: AppIdentifier,
    state: State<VolumeCommandSender>,
) -> VolumePercent {
    let (tx, rx) = channel();

    let _ = state.send(VolumeCommand::GetAppVolume(app_identifier, tx));

    match rx.recv() {
        Ok(result) => result,
        Err(_) => 0.0,
    }
}

#[tauri::command]
pub fn set_app_volume(
    app_identifier: AppIdentifier,
    volume: VolumePercent,
    state: State<VolumeCommandSender>,
) {
    let _ = state.send(VolumeCommand::SetAppVolume(app_identifier, volume));
}

#[tauri::command]
pub fn mute_app_volume(app_identifier: AppIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::MuteApp(app_identifier));
}

#[tauri::command]
pub fn unmute_app_volume(app_identifier: AppIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::UnmuteApp(app_identifier));
}

// ============================ DeviceControl ============================
#[tauri::command]
pub fn get_current_playback_device(state: State<VolumeCommandSender>) -> Option<AudioDevice> {
    let (tx, rx) = channel();

    let _ = state.send(VolumeCommand::GetCurrentPlaybackDevice(tx));

    match rx.recv() {
        Ok(Ok(result)) => Some(result),
        Ok(Err(_)) => None,
        Err(_) => None,
    }
}

#[tauri::command]
pub fn get_playback_devices(state: State<VolumeCommandSender>) -> Vec<AudioDevice> {
    let (tx, rx) = channel();

    let _ = state.send(VolumeCommand::GetPlaybackDevices(tx));

    match rx.recv() {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => vec![],
        Err(_) => vec![],
    }
}
