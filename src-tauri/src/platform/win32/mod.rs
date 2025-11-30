use std::sync::mpsc::Sender;
use std::sync::Mutex;

use crate::types::shared::{UpdateChange, VolumeControllerTrait};

mod com_scope;

mod application_volume;
mod device_control;
mod icon;
mod master_volume;
mod update;

mod convert;
mod util;

pub use icon::extract_icon;

type VolumeSender = Sender<UpdateChange>;
pub struct VolumeController {
    com: com_scope::ComManager,
    audio_monitor: Mutex<update::AudioMonitor>,
}

pub fn make_controller(sender: VolumeSender) -> Box<dyn VolumeControllerTrait> {
    Box::new(VolumeController::new(sender))
}

impl VolumeController {
    pub fn new(sender: VolumeSender) -> Self {
        let com_manager =
            com_scope::ComManager::try_new().expect("Failed to initialize COM manager");

        let mut audio_monitor = update::AudioMonitor::new(sender);
        audio_monitor.register_callbacks(&com_manager);

        Self {
            com: com_manager,
            audio_monitor: Mutex::new(audio_monitor),
        }
    }
}

impl VolumeControllerTrait for VolumeController {
    fn cleanup(&self) {
        if let Ok(mut audio) = self.audio_monitor.lock() {
            audio.unregister_callbacks();
        }
    }

    fn check_and_reinit(&self) {
        if let Ok(mut audio) = self.audio_monitor.lock() {
            if audio.check_and_reinit(&self.com) {
                println!("Re-initialization complete!");
            }
        }
    }
}
