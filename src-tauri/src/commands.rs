use tauri::State;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    server::{VolumeCommand, VolumeCommandSender},
    types::shared::{AppIdentifier, AudioDevice, AudioSession, DeviceIdentifier, VolumePercent},
};

// ============================ Master ============================
#[tauri::command]
pub fn set_device_volume(
    device_id: DeviceIdentifier,
    percent: VolumePercent,
    state: State<VolumeCommandSender>,
) {
    let _ = state.send(VolumeCommand::SetDeviceVolume(device_id, percent));
}

#[tauri::command]
pub async fn get_device_volume(
    device_id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Option<f32>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetDeviceVolume(device_id, tx));

    if let Some(value) = rx.recv().await {
        return value.map_err(|_err| ());
    }

    Ok(None)
}

#[tauri::command]
pub fn mute_device(device_id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::MuteDevice(device_id));
}

#[tauri::command]
pub fn unmute_device(device_id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::UnmuteDevice(device_id));
}

// ============================ Application ============================
#[tauri::command]
pub async fn get_all_applications(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AudioSession>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetAllApplications(tx));

    if let Some(value) = rx.recv().await {
        return value.map_err(|_err| ());
    }

    Ok(vec![])
}

#[tauri::command]
pub async fn get_app_volume(
    app_identifier: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Option<VolumePercent>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetAppVolume(app_identifier, tx));

    if let Some(value) = rx.recv().await {
        let volume = match value {
            Ok(volume) => volume,
            Err(_) => None,
        };
        return Ok(volume);
    }

    Err(())
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
pub async fn get_current_playback_device(
    state: State<'_, VolumeCommandSender>,
) -> Result<AudioDevice, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetCurrentPlaybackDevice(tx));

    if let Some(value) = rx.recv().await {
        return value.map_err(|_err| ());
    }

    Err(())
}

#[tauri::command]
pub async fn get_playback_devices(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AudioDevice>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetPlaybackDevices(tx));

    if let Some(value) = rx.recv().await {
        return value.map_err(|_err| ());
    }

    Err(())
}
