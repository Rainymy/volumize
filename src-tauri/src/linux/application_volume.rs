use crate::types::shared::{
    AppIdentifier, ApplicationVolumeControl, AudioApplication, AudioSession, AudioVolume,
    VolumeControllerError, VolumePercent, VolumeResult,
};

use super::VolumeController;

impl ApplicationVolumeControl for VolumeController {
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_app_volume(&self, _app: AppIdentifier) -> VolumeResult<AudioVolume> {
        Err(VolumeControllerError::NotImplemented())
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
