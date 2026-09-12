use crate::types::shared::{ApplicationVolumeControl, VolumeResult};
use shared_types::{AppIdentifier, AudioApplication, AudioVolume, VolumePercent};

use super::VolumeController;

impl ApplicationVolumeControl for VolumeController {
    fn get_application(&self, _id: AppIdentifier) -> VolumeResult<AudioApplication> {
        Ok(AudioApplication::default())
    }
    fn get_app_volume(&self, _app: AppIdentifier) -> VolumeResult<AudioVolume> {
        Ok(AudioVolume::default())
    }

    fn set_app_volume(&self, _app: AppIdentifier, _volume: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }
}
