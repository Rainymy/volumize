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
    fn with_default_audio_endpoint<F, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&IAudioEndpointVolume) -> VolumeResult<R>,
    {
        com_scope::with_com_initialized(|device_enumerator| unsafe {
            let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
            let endpoint_volume: IAudioEndpointVolume =
                default_device.Activate(CLSCTX_ALL, None)?;
            callback(&endpoint_volume)
        })
    }
    fn _with_default_audio_sessions<F, R>(&self, callback: F) -> VolumeResult<R>
    where
        F: FnOnce(&IAudioSessionManager2) -> VolumeResult<R>,
    {
        com_scope::with_com_initialized(|device_enumerator| unsafe {
            let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
            let session_manager: IAudioSessionManager2 =
                default_device.Activate(CLSCTX_ALL, None)?;
            callback(&session_manager)
        })
    }
}
