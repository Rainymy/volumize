use crate::types::shared::VolumeResult;
use crate::types::shared::{AppIdentifier, AudioSession, VolumeControllerTrait, VolumePercent};

pub struct VolumeController;

pub fn make_controller() -> Box<dyn VolumeControllerTrait> {
    return Box::new(VolumeController);
}

impl VolumeControllerTrait for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>> {
        Ok(None)
    }

    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        Ok(Some(0.8))
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

    fn set_app_volume(&self, _app: AppIdentifier, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }
}
