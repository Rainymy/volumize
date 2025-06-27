// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod win32;

fn main() {
    #[cfg(target_os = "windows")]
    let _ = win32::windows::windows_controller();
    volumize_lib::run();
}
