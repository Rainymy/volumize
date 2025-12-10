use tauri::{Manager, Result as TauriResult};

use crate::{
    server::{ServiceDiscovery, WebSocketServerState},
    types::{click::ClickState, storage::Storage, volume::VolumeCommandSender},
};

use crate::commands;

pub fn create_tauri_app() -> TauriResult<tauri::App> {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .manage(VolumeCommandSender::new())
        .manage(WebSocketServerState::default())
        .manage(ServiceDiscovery::default())
        .manage(ClickState::new(None))
        .manage(Storage::default())
        .setup(super::setup)
        .on_menu_event(super::menu_event)
        .on_window_event(|_window, _event| {
            let storage = _window.app_handle().state::<Storage>();
            let should_exit_to_tray = storage.get().exit_to_tray;

            if should_exit_to_tray {
                // Turn off exit to tray functionality to test other features.
                #[cfg(not(debug_assertions))]
                {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = _event {
                        let _ = _window.hide();
                        api.prevent_close();
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Master volume controls
            commands::device_get_volume,
            commands::device_set_volume,
            commands::device_mute,
            commands::device_unmute,
            // Application volume controls
            commands::get_application,
            commands::application_get_icon,
            commands::application_get_volume,
            commands::application_set_volume,
            commands::application_mute,
            commands::application_unmute,
            // Device controls
            commands::get_playback_devices,
            commands::get_device_applications,
            // Miscellaneous
            commands::discover_server_address
        ])
        .build(tauri::generate_context!())
}
