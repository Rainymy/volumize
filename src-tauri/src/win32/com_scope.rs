use windows::core::{Interface, GUID, PCWSTR};
use windows::Win32::{
    Media::Audio::Endpoints::IAudioEndpointVolume,
    Media::Audio::{
        eConsole, eRender, IAudioSessionControl2, IAudioSessionManager2, IMMDevice,
        IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE, DEVICE_STATEMASK_ALL,
    },
    System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX, CLSCTX_ALL, COINIT_MULTITHREADED,
    },
};

use super::util;
use crate::types::shared::{VolumeControllerError, VolumeResult};

#[must_use = "ComScope must be kept alive to maintain COM initialization"]
pub struct ComScope;

impl ComScope {
    pub fn try_new() -> VolumeResult<Self> {
        #[cfg(debug_assertions)]
        dbg!("ComScope initialized!");

        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .ok()
                .map_err(|e| {
                    VolumeControllerError::ComError(format!("Failed to initialize COM: {}", e))
                })?;
        }
        Ok(Self)
    }
}

impl Drop for ComScope {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }

        #[cfg(debug_assertions)]
        dbg!("ComScope Dropped!");
    }
}

pub struct ComManager {
    _com_guard: ComScope,
    event_context: GUID,
    device_enumerator: IMMDeviceEnumerator,
}

impl ComManager {
    pub const CLS_CONTEXT: CLSCTX = CLSCTX_ALL;
    pub const DEVICE_STATE_CONTEXT: DEVICE_STATE = DEVICE_STATE(DEVICE_STATEMASK_ALL);

    pub fn try_new() -> VolumeResult<Self> {
        let com_guard = ComScope::try_new()?;
        let device_enumerator = unsafe {
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| {
                VolumeControllerError::ComError(format!("Failed to create instance COM: {}", e))
            })?
        };

        let event_context = GUID::new().map_err(|e| {
            VolumeControllerError::ComError(format!("Failed to create new GUID COM: {}", e))
        })?;

        Ok(Self {
            _com_guard: com_guard,
            event_context,
            device_enumerator,
        })
    }

    pub fn get_event_context(&self) -> &GUID {
        &self.event_context
    }

    pub fn with_default_generic_activate<T>(&self) -> VolumeResult<T>
    where
        T: Interface,
    {
        unsafe {
            let default_device = self
                .device_enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .map_err(VolumeControllerError::WindowsApiError)?;

            default_device
                .Activate::<T>(ComManager::CLS_CONTEXT, None)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }

    pub fn with_default_audio_endpoint(&self) -> VolumeResult<IAudioEndpointVolume> {
        self.with_default_generic_activate::<IAudioEndpointVolume>()
    }

    pub fn _with_default_audio_session_control2(&self) -> VolumeResult<IAudioSessionControl2> {
        self.with_default_generic_activate::<IAudioSessionControl2>()
    }

    pub fn _with_default_audio_sessions_manager2(&self) -> VolumeResult<IAudioSessionManager2> {
        self.with_default_generic_activate::<IAudioSessionManager2>()
    }

    pub fn get_all_device_id(&self) -> VolumeResult<Vec<String>> {
        unsafe {
            let device_collection = self
                .device_enumerator
                .EnumAudioEndpoints(eRender, Self::DEVICE_STATE_CONTEXT)?;

            let count = device_collection.GetCount()?;
            let mut ids = Vec::with_capacity(count as usize);

            for i in 0..count {
                if let Ok(device) = device_collection.Item(i) {
                    if let Ok(id_pw_str) = device.GetId() {
                        if let Ok(pw_string) = util::pwstr_to_string(&id_pw_str) {
                            ids.push(pw_string);
                        }
                    }
                }
            }

            Ok(ids)
        }
    }

    pub fn get_default_device(&self) -> VolumeResult<IMMDevice> {
        unsafe {
            self.device_enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }

    pub fn get_device_with_id(&self, id: PCWSTR) -> VolumeResult<IMMDevice> {
        unsafe {
            self.device_enumerator
                .GetDevice(id)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }
}
