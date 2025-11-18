use std::path::PathBuf;

use crate::types::shared::{DeviceVolumeControl, VolumeControllerTrait, VolumeResult};

pub struct VolumeController;

mod application_volume;
mod device_control;
mod master_volume;

pub fn make_controller() -> VolumeResult<Box<dyn VolumeControllerTrait>> {
    return Ok(Box::new(VolumeController::try_new()?));
}

impl VolumeController {
    pub fn try_new() -> VolumeResult<Self> {
        Ok(Self)
    }
}

impl VolumeControllerTrait for VolumeController {}

pub fn extract_icon(_path: PathBuf) -> Option<Vec<u8>> {
    None
}
