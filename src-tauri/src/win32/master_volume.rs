use super::VolumeController;
use crate::types::shared::{
    AudioVolume, MasterVolumeControl, VolumePercent, VolumeResult, VolumeValidation,
};

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        self.with_default_audio_endpoint(|endpoint| unsafe {
            let volume = endpoint.GetMasterVolumeLevelScalar()?;
            Ok(Some(volume))
        })
    }

    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        self.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMasterVolumeLevelScalar(percent, &self.event_context)?;
            Ok(())
        })
    }

    fn mute_master(&self) -> VolumeResult<()> {
        self.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMute(true, &self.event_context)?;
            Ok(())
        })
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        self.with_default_audio_endpoint(|endpoint| unsafe {
            endpoint.SetMute(false, &self.event_context)?;
            Ok(())
        })
    }
}
