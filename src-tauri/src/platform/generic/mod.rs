use std::path::PathBuf;

use crate::types::shared::{DeviceVolumeControl, VolumeControllerTrait};

pub struct VolumeController;

mod application_volume;
mod device_control;
mod master_volume;

pub fn make_controller() -> Box<dyn VolumeControllerTrait> {
    Box::new(VolumeController::new())
}

impl VolumeController {
    pub fn new() -> Self {
        Self
    }
}

impl VolumeControllerTrait for VolumeController {}

pub fn extract_icon(_path: PathBuf) -> Option<Vec<u8>> {
    None
}
