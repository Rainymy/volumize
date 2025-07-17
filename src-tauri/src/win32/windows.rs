use windows::core::Interface;
use windows::Win32::{
    // Foundation::HWND,
    Media::Audio::{Endpoints::IAudioEndpointVolume, *},
    System::Com::CLSCTX_ALL,
};

use crate::types::shared::VolumeControllerTrait;
use crate::types::shared::VolumeResult;

mod com_scope;
mod volume_controller_trait;

pub struct VolumeController;

pub fn make_controller() -> Box<dyn VolumeControllerTrait> {
    return Box::new(VolumeController);
}

impl VolumeController {
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

    fn _with_default_audio_sessions<F, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&IAudioSessionManager2) -> VolumeResult<R>,
    {
        self.with_default_generic_activate::<F, IAudioSessionManager2, R>(callback)
    }
}
