use crate::types::shared::DeviceIdentifier;
use crate::types::shared::{VolumePercent, VolumeResult};

use super::DeviceVolumeControl;
use super::VolumeController;

impl DeviceVolumeControl for VolumeController {
    fn get_device_volume(&self, _id: DeviceIdentifier) -> VolumeResult<VolumePercent> {
        Ok(0.0)
    }

    fn set_device_volume(
        &self,
        _id: DeviceIdentifier,
        _percent: VolumePercent,
    ) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_device(&self, _id: DeviceIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_device(&self, _id: DeviceIdentifier) -> VolumeResult<()> {
        Ok(())
    }
}
