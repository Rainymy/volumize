use tauri::State;

use crate::{server::service_discovery, types::volume::VolumeCommandSender};

use shared_types::{
    protocol::{Command, CommandResponse, Response},
    AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, Identifier, VolumePercent,
};

// TODO: Tauri command should return a proper ERROR:s, String should be fine.

// ========================== Controller ===========================
#[tauri::command]
#[specta::specta]
pub async fn set_volume(
    id: Identifier,
    volume: VolumePercent,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let _response = client.request(Command::SetVolume { id, volume }).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_volume(
    id: Identifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<VolumePercent, String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client.request(Command::GetVolume { id }).await?;
    let CommandResponse { id: _id, response } = response;

    match response {
        Response::Volume { id: _id, volume } => Ok(volume.current),
        _ => Err("Unexpected response".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn set_mute(id: Identifier, state: State<'_, VolumeCommandSender>) -> Result<(), String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let _response = client.request(Command::SetMute { id, mute: true }).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_unmute(
    id: Identifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let _response = client.request(Command::SetMute { id, mute: false }).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_icon(
    id: Identifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<u8>, String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client.request(Command::GetIcon { id }).await?;

    let CommandResponse { id: _id, response } = response;
    match response {
        Response::Icon { data, .. } => Ok(data),
        _ => Err("Unexpected response".to_string()),
    }
}

// ========================= Application ===========================
#[tauri::command]
#[specta::specta]
pub async fn get_application(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<AudioApplication, String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client.request(Command::GetApplication { id }).await?;
    let CommandResponse { id: _id, response } = response;

    match response {
        Response::Application(app) => Ok(app),
        _ => Err("Unexpected response".to_string()),
    }
}

// =========================== Playback ============================
#[tauri::command]
#[specta::specta]
pub async fn get_playback_devices(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AudioDevice>, String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client.request(Command::GetPlaybackDevices).await?;
    let CommandResponse { id: _id, response } = response;

    match response {
        Response::DeviceList(devices) => Ok(devices),
        _ => Err("Unexpected response".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_device_applications(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AppIdentifier>, String> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client.request(Command::GetApplications { id }).await?;
    let CommandResponse { id: _id, response } = response;

    match response {
        Response::ApplicationList { apps, .. } => Ok(apps),
        _ => Err("Unexpected response".to_string()),
    }
}

// ========================= Miscellaneous =========================
#[tauri::command]
#[specta::specta]
pub async fn discover_server_address() -> Option<String> {
    service_discovery::discover_server().await.ok()
}
