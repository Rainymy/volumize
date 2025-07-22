use super::VolumeController;
use crate::types::shared::{AudioApplication, VolumeControllerTrait, VolumeResult};

impl VolumeControllerTrait for VolumeController {
    fn load_sessions(&self) -> VolumeResult<Vec<AudioApplication>> {
        Ok(vec![])
    }
}
