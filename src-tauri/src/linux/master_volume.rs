use super::VolumeController;
use crate::types::shared::{MasterVolumeControl, VolumePercent, VolumeResult};

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        Ok(None)
    }

    fn set_master_volume(&self, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_master(&self) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        Ok(())
    }
}
