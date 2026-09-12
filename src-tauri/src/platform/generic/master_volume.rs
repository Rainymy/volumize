use shared_types::{AudioVolume, DeviceIdentifier, VolumePercent};

use crate::types::shared::VolumeResult;

use super::{DeviceVolumeControl, VolumeController};

impl DeviceVolumeControl for VolumeController {
    fn get_device_volume(&self, _id: DeviceIdentifier) -> VolumeResult<AudioVolume> {
        Ok(AudioVolume::default())
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
