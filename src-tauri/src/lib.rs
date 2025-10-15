#![allow(dead_code)]
mod server;
mod storage;
mod tray;
mod types;

#[tauri::command]
async fn discover_server_address() -> Option<String> {
    server::discover_server().await.ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![discover_server_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
