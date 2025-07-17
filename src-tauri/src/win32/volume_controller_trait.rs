use super::VolumeController;
use crate::types::shared::{
    AppIdentifier, ApplicationVolumeControl, AudioSession, DeviceControl, MasterVolumeControl,
    VolumeControllerTrait, VolumePercent, VolumeResult,
};

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        self.with_default_audio_endpoint(|endpoint| unsafe {
            let volume = endpoint.GetMasterVolumeLevelScalar()?;
            Ok(Some(volume))
        })
    }

    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()> {
        self.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMasterVolumeLevelScalar(percent, std::ptr::null())?;
            Ok(())
        })
    }

    fn mute_master(&self) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        Ok(())
    }
}

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

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>> {
        Ok(None)
    }
}

impl VolumeControllerTrait for VolumeController {
    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }
}
