use crate::types::shared::{VolumeControllerTrait, VolumeResult};

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
