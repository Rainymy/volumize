use super::VolumeController;
use crate::types::shared::{
    AudioVolume, MasterVolumeControl, VolumePercent, VolumeResult, VolumeValidation,
};

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        self.com.with_default_audio_endpoint(|endpoint| unsafe {
            Ok(Some(endpoint.GetMasterVolumeLevelScalar()?))
        })
    }

    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        self.com.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMasterVolumeLevelScalar(percent, self.com.get_event_context())?;
            Ok(())
        })
    }

    fn mute_master(&self) -> VolumeResult<()> {
        self.com.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMute(true, self.com.get_event_context())?;
            Ok(())
        })
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        self.com.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMute(false, self.com.get_event_context())?;
            Ok(())
        })
    }
}
