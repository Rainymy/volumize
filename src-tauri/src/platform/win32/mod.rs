use crate::types::shared::{VolumeControllerTrait, VolumeResult};

mod com_scope;

mod application_volume;
mod device_control;
mod icon;
mod master_volume;

mod convert;
mod util;

pub use icon::extract_icon;

pub struct VolumeController {
    com: com_scope::ComManager,
}

pub fn make_controller() -> VolumeResult<Box<dyn VolumeControllerTrait>> {
    return Ok(Box::new(VolumeController::try_new()?));
}

impl VolumeController {
    pub fn try_new() -> VolumeResult<Self> {
        Ok(Self {
            com: com_scope::ComManager::try_new()?,
        })
    }
}

impl VolumeControllerTrait for VolumeController {}
