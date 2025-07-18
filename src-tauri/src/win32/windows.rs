use windows::core::{Interface, GUID};
use windows::Win32::{
    Media::Audio::{Endpoints::IAudioEndpointVolume, *},
    System::Com::CLSCTX_ALL,
};

use crate::types::shared::VolumeResult;
use crate::types::shared::{AudioVolume, VolumeControllerTrait, VolumePercent, VolumeValidation};

mod com_scope;

mod application_volume;
mod device_control;
mod master_volume;
mod volume_controller_trait;

pub struct VolumeController {
    event_context: GUID,
    com_guard: com_scope::ComScope,
}

pub fn make_controller() -> VolumeResult<Box<dyn VolumeControllerTrait>> {
    return Ok(Box::new(VolumeController::new()?));
}

impl VolumeController {
    #[allow(dead_code)]
    const NO_CONTEXT: *const GUID = std::ptr::null();

    pub fn new() -> VolumeResult<Self> {
        Ok(Self {
            event_context: GUID::new()?,
            com_guard: com_scope::ComScope::try_new()?,
        })
    }

    fn with_default_generic_activate<'a, F, T, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&T) -> VolumeResult<R>,
        T: Interface + 'a,
    {
        com_scope::with_com_initialized(|device_enumerator| unsafe {
            let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
            let endpoint_volume = default_device.Activate::<T>(CLSCTX_ALL, None)?;
            callback(&endpoint_volume)
        })
    }

    fn with_default_audio_endpoint<F, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&IAudioEndpointVolume) -> VolumeResult<R>,
    {
        self.with_default_generic_activate::<F, IAudioEndpointVolume, R>(callback)
    }

    #[allow(dead_code)]
    fn with_default_audio_sessions<F, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&IAudioSessionManager2) -> VolumeResult<R>,
    {
        self.with_default_generic_activate::<F, IAudioSessionManager2, R>(callback)
    }
}

impl VolumeValidation for VolumeController {
    fn validate_volume(volume: VolumePercent) -> VolumeResult<()> {
        AudioVolume::validate_volume(volume)
    }
}
