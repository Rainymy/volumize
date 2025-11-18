use crate::types::shared::{
    AppIdentifier, ApplicationVolumeControl, AudioApplication, AudioVolume, VolumeControllerError,
    VolumePercent, VolumeResult,
};

use super::VolumeController;

impl ApplicationVolumeControl for VolumeController {
    fn get_application(&self, _id: AppIdentifier) -> VolumeResult<AudioApplication> {
        Err(VolumeControllerError::Unknown("Not implemented".into()))
    }
    fn get_app_volume(&self, _app: AppIdentifier) -> VolumeResult<AudioVolume> {
        Err(VolumeControllerError::Unknown("Not implemented".into()))
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
