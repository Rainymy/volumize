// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;
mod win32;

fn main() {
    win32::windows::windows_controller();
    volumize_lib::run();
}
