use tauri::State;

use crate::{server::service_discovery, types::volume::VolumeCommandSender};

use shared_types::{
    protocol::{Command, Response},
    AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, Identifier, VolumePercent,
};

// TODO: Tauri command should return a proper ERROR:s, String should be fine.

// ============================ Master ============================
#[tauri::command]
pub async fn device_set_volume(
    id: DeviceIdentifier,
    volume: VolumePercent,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetVolume {
            id: Identifier::Device(id),
            volume,
        })
        .await
        .map_err(|_| ())?;

    Ok(())
}

#[tauri::command]
pub async fn device_get_volume(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<VolumePercent, ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let response = client
        .request(&Command::GetVolume {
            id: Identifier::Device(id),
        })
        .await
        .map_err(|_| ())?;

    match response {
        Response::Volume { id: _id, volume } => Ok(volume.current),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn device_mute(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetMute {
            id: Identifier::Device(id),
            mute: true,
        })
        .await
        .map_err(|_| ())?;

    Ok(())
}

#[tauri::command]
pub async fn device_unmute(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetMute {
            id: Identifier::Device(id),
            mute: false,
        })
        .await
        .map_err(|_| ())?;

    Ok(())
}

// ========================= Application ===========================
#[tauri::command]
pub async fn application_get_icon(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<u8>, ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let response = client
        .request(&Command::GetIcon { app_id: id })
        .await
        .map_err(|_| ())?;

    match response {
        Response::Icon { data, .. } => Ok(data),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn get_application(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<AudioApplication, ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let response = client
        .request(&Command::GetApplication { app_id: id })
        .await
        .map_err(|_| ())?;

    match response {
        Response::Application(app) => Ok(app),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn application_get_volume(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<VolumePercent, ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let response = client
        .request(&Command::GetVolume {
            id: Identifier::App(id),
        })
        .await
        .map_err(|_| ())?;

    match response {
        Response::Volume { volume, .. } => Ok(volume.current),
        _ => Err(()),
    }
}

#[tauri::command]
pub async fn application_set_volume(
    id: AppIdentifier,
    volume: VolumePercent,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetVolume {
            id: Identifier::App(id),
            volume,
        })
        .await;

    Ok(())
}

#[tauri::command]
pub async fn application_mute(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetMute {
            id: Identifier::App(id),
            mute: true,
        })
        .await
        .map_err(|_| ())?;

    Ok(())
}

#[tauri::command]
pub async fn application_unmute(
    id: AppIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<(), ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let _response = client
        .request(&Command::SetMute {
            id: Identifier::App(id),
            mute: false,
        })
        .await
        .map_err(|_| ())?;

    Ok(())
}

// =========================== Playback ============================
#[tauri::command]
pub async fn get_playback_devices(
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AudioDevice>, String> {
    let client_lock = state.client.lock().await;
    // dbg!("{:#?}", &client_lock);

    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err("No client".to_string()),
    };

    let response = client
        .request(&Command::GetPlaybackDevices)
        .await
        .map_err(|_e| "Requset failed")?;

    match response {
        Response::DeviceList(devices) => Ok(devices),
        _ => Err("Unexpected response".to_string()),
    }
}

#[tauri::command]
pub async fn get_device_applications(
    id: DeviceIdentifier,
    state: State<'_, VolumeCommandSender>,
) -> Result<Vec<AppIdentifier>, ()> {
    let client_lock = state.client.lock().await;
    let client = match client_lock.as_ref() {
        Some(client) => client,
        None => return Err(()),
    };

    let response = client
        .request(&Command::GetApplications { device_id: id })
        .await
        .map_err(|_| ())?;

    match response {
        Response::ApplicationList { apps, .. } => Ok(apps),
        _ => Err(()),
    }
}

// ========================= Miscellaneous =========================
#[tauri::command]
pub async fn discover_server_address() -> Option<String> {
    service_discovery::discover_server().await.ok()
}
