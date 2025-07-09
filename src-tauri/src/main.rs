// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;

#[cfg(target_os = "windows")]
#[path = "win32/windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/linux.rs"]
mod platform;

fn main() {
    let _controller = platform::make_controller();
    volumize_lib::run();
}
