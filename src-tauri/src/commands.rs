use tauri::State;
use volumize_lib::{VolumeCommand, VolumeCommandSender};

use std::sync::mpsc::channel;

#[tauri::command]
pub fn set_master_volume(percent: f32, state: State<VolumeCommandSender>) {
    let _ = state.tx.send(VolumeCommand::SetMasterVolume(percent));
}

#[tauri::command]
pub fn get_master_volume(state: State<VolumeCommandSender>) -> Option<f32> {
    let (tx, rx) = channel();

    let _ = state.tx.send(VolumeCommand::GetMasterVolume(tx));

    match rx.recv() {
        Ok(Ok(Some(percent))) => Some(percent),
        Ok(Ok(None)) => None,
        _ => None,
    }
}
