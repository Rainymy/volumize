use tauri::State;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    server::volume_control::{VolumeCommand, VolumeCommandSender},
    types::shared::{
        AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, VolumePercent,
    },
};

// ============================ Master ============================
#[tauri::command]
pub fn device_set_volume(
    id: DeviceIdentifier,
    volume: VolumePercent,
    state: State<VolumeCommandSender>,
) {
    let _ = state.send(VolumeCommand::DeviceSetVolume {
        id,
        volume,
        request_id: String::new(),
    });
}

#[tauri::command]
pub async fn device_get_volume(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<f32, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::DeviceGetVolume {
        id,
        sender: tx,
        request_id: String::new(),
    });

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub fn device_mute(id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::DeviceMute {
        id,
        request_id: String::new(),
    });
}

#[tauri::command]
pub fn device_unmute(id: DeviceIdentifier, state: State<VolumeCommandSender>) {
    let _ = state.send(VolumeCommand::DeviceUnmute {
        id,
        request_id: String::new(),
    });
}

// ========================= Application ===========================
#[tauri::command]
pub async fn application_get_icon(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<u8>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::ApplicationGetIcon {
        id,
        sender: tx,
        request_id: String::new(),
    });

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub async fn get_application(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<AudioApplication, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetApplication {
        id,
        sender: tx,
        request_id: String::new(),
    });

    let value = match rx.recv().await {
        Some(v) => v,
        None => return Err(()),
    };

    value.map_err(|_| ())
}

#[tauri::command]
pub async fn application_get_volume(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Option<VolumePercent>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::ApplicationGetVolume {
        id,
        sender: tx,
        request_id: String::new(),
    });

    if let Some(value) = rx.recv().await {
        return Ok(value.ok());
    }

    Err(())
}

#[tauri::command]
pub async fn application_set_volume(
    id: AppIdentifier,
    volume: VolumePercent,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), String> {
    state.send(VolumeCommand::ApplicationSetVolume {
        id,
        volume,
        request_id: String::new(),
    })
}

#[tauri::command]
pub async fn application_mute(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), String> {
    state.send(VolumeCommand::ApplicationMute {
        id,
        request_id: String::new(),
    })
}

#[tauri::command]
pub async fn application_unmute(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), String> {
    state.send(VolumeCommand::ApplicationUnmute {
        id,
        request_id: String::new(),
    })
}

// =========================== Playback ============================
#[tauri::command]
pub async fn get_playback_devices(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AudioDevice>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetPlaybackDevices {
        sender: tx,
        request_id: String::new(),
    });

    if let Some(value) = rx.recv().await {
        return value.map_err(|_err| ());
    }

    Err(())
}

#[tauri::command]
pub async fn get_device_applications(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AppIdentifier>, ()> {
    let (tx, mut rx) = unbounded_channel();

    let _ = state.send(VolumeCommand::GetDeviceApplications {
        id,
        sender: tx,
        request_id: String::new(),
    });

    if let Some(value) = rx.recv().await {
        return Ok(value.unwrap_or(vec![]));
    }

    Err(())
}
