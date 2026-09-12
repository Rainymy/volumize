use std::sync::mpsc::Sender;

use shared_types::UpdateChange;

use crate::types::shared::{DeviceVolumeControl, VolumeControllerTrait};

pub struct VolumeController;

mod application_volume;
mod device_control;
mod master_volume;

type VolumeSender = Sender<UpdateChange>;
pub fn make_controller(sender: VolumeSender) -> Box<dyn VolumeControllerTrait> {
    Box::new(VolumeController::new(sender))
}

impl VolumeController {
    pub fn new(_sender: VolumeSender) -> Self {
        Self {}
    }
}

impl VolumeControllerTrait for VolumeController {
    fn cleanup(&self) {
        // Implement cleanup logic here
    }

    fn check_and_reinit(&self) {
        // Implement cleanup logic here
    }
}

pub fn extract_icon(_path: String) -> Option<Vec<u8>> {
    None
}
