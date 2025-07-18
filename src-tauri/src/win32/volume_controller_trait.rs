use super::VolumeController;
use crate::types::shared::{AudioSession, VolumeControllerTrait, VolumeResult};

impl VolumeControllerTrait for VolumeController {
    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }
}
