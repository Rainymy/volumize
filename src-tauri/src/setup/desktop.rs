use tauri::{Manager, Result as TauriResult};

use crate::commands;
use crate::{
    server::{serial::SerialState, websocket::WebSocketServerState, ServiceDiscovery},
    types::{click::DoubleClickState, storage::Storage, volume::VolumeCommandSender},
};

pub fn create_tauri_app() -> TauriResult<tauri::App> {
    let builder = spectra_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            super::show_window_visibility(app);
        }))
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::default(),
            None,
        ))
        .manage(VolumeCommandSender::default())
        .manage(WebSocketServerState::default())
        .manage(ServiceDiscovery::default())
        .manage(DoubleClickState::new(None))
        .manage(SerialState::default())
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
        .invoke_handler(builder.invoke_handler())
        .build(tauri::generate_context!())
}

fn spectra_builder() -> tauri_specta::Builder<tauri::Wry> {
    use crate::types::shared::{UPDATE_EVENT_NAME, VOLUME_LABEL_EVENT, WEBSOCKET_PORT};
    use shared_types::{
        protocol::{CommandRequest, CommandResponse},
        UpdateChange,
    };
    use tauri_specta::collect_commands;

    let collected_types = specta::Types::default()
        .register::<CommandRequest>()
        .register::<CommandResponse>()
        .register::<UpdateChange>();

    tauri_specta::Builder::<tauri::Wry>::new()
        .types(&collected_types)
        .commands(collect_commands![
            commands::get_volume,
            commands::set_volume,
            commands::set_mute,
            commands::set_unmute,
            commands::get_icon,
            // Application
            commands::get_application,
            commands::get_playback_devices,
            commands::get_device_applications,
            // Miscellaneous
            commands::discover_server_address,
        ])
        .constant(stringify!(UPDATE_EVENT_NAME), UPDATE_EVENT_NAME)
        .constant(stringify!(VOLUME_LABEL_EVENT), VOLUME_LABEL_EVENT)
        .constant(stringify!(WEBSOCKET_PORT), WEBSOCKET_PORT)
}

#[test]
fn export_spectra() {
    use specta_typescript::Typescript;
    spectra_builder()
        .export(Typescript::default(), "../src/types/bindings.ts")
        .expect("Failed to export typescript bindings");
}
