use std::sync::Mutex;

use crate::types::shared::VolumeControllerTrait;

mod com_scope;

mod application_volume;
mod device_control;
mod icon;
mod master_volume;
mod update;

mod convert;
mod util;

pub use icon::extract_icon;
pub struct VolumeController {
    com: com_scope::ComManager,
    audio_monitor: Mutex<update::AudioMonitor>,
}

pub fn make_controller() -> Box<dyn VolumeControllerTrait> {
    return Box::new(VolumeController::new());
}

impl VolumeController {
    pub fn new() -> Self {
        let com = com_scope::ComManager::try_new().expect("Failed to initialize COM manager");

        let mut audio_monitor = update::AudioMonitor::new();
        audio_monitor.register_callbacks(&com);

        Self {
            com,
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
}
