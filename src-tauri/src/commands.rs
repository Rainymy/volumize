use tauri::State;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    server::{VolumeCommand, VolumeCommandSender},
    types::shared::{
        AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, VolumePercent,
    },
};

// ============================ Master ============================
#[tauri::command]
pub async fn get_all_devices(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<DeviceIdentifier>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetAllDevices(tx));

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub fn set_device_volume(
    id: DeviceIdentifier,
    percent: VolumePercent,
    state: State<VolumeCommandSender>,
) {
    let _ = state.send(VolumeCommand::SetDeviceVolume(id, percent));
}

#[tauri::command]
pub async fn get_device_volume(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<f32, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetDeviceVolume(id, tx));

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub fn mute_device(id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::MuteDevice(id));
}

#[tauri::command]
pub fn unmute_device(id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::UnmuteDevice(id));
}

// ============================ Application ============================
#[tauri::command]
pub async fn get_application_icon(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<u8>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetApplicationIcon(id, tx));

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())

    // if let Some(value) = rx.recv().await {
    //     return match value {
    //         Ok(Some(icon)) => Ok(icon),
    //         Ok(None) => Err(()),
    //         Err(_) => Err(()),
    //     };
    // }

    // Err(())
}

#[tauri::command]
pub async fn get_application(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<AudioApplication, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetApplication(id, tx));

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub async fn get_device_applications(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AppIdentifier>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetDeviceApplications(id, tx));

    if let Some(value) = rx.recv().await {
        return Ok(value.unwrap_or(vec![]));
    }

    Err(())
}

#[tauri::command]
pub async fn get_app_volume(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Option<VolumePercent>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetAppVolume(id, tx));

    if let Some(value) = rx.recv().await {
        return Ok(value.ok());
    }

    Err(())
}

#[tauri::command]
pub fn set_app_volume(id: AppIdentifier, volume: VolumePercent, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::SetAppVolume(id, volume));
}

#[tauri::command]
pub fn mute_app_volume(id: AppIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::MuteApp(id));
}

#[tauri::command]
pub fn unmute_app_volume(id: AppIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::UnmuteApp(id));
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
