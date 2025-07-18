use super::VolumeController;
use crate::types::shared::{
    AppIdentifier, ApplicationVolumeControl, AudioSession, VolumePercent, VolumeResult,
};

impl ApplicationVolumeControl for VolumeController {
    fn set_app_volume(&self, _app: AppIdentifier, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }
}
