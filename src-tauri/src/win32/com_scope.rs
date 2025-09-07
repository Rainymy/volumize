use std::sync::atomic::{AtomicBool, Ordering};

use windows::{
    core::{Interface, GUID},
    Win32::{
        Media::Audio::{
            eConsole, eRender, ERole, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
            DEVICE_STATE, DEVICE_STATE_ACTIVE,
        },
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX, CLSCTX_ALL,
            COINIT_MULTITHREADED,
        },
    },
};

use super::util;
use crate::types::shared::{VolumeControllerError, VolumeResult};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[must_use = "ComScope must be kept alive to maintain COM initialization"]
pub struct ComManager {
    event_context: GUID,
    device_enumerator: IMMDeviceEnumerator,
}

impl Drop for ComManager {
    fn drop(&mut self) {
        if INITIALIZED.load(Ordering::SeqCst) {
            unsafe {
                CoUninitialize();
            }
        }

        INITIALIZED.store(false, Ordering::SeqCst);

        #[cfg(debug_assertions)]
        dbg!("ComScope Dropped!");
    }
}

impl ComManager {
    pub const CLS_CONTEXT: CLSCTX = CLSCTX_ALL;
    pub const DEVICE_STATE_CONTEXT: DEVICE_STATE = DEVICE_STATE_ACTIVE;
    pub const E_ROLE: ERole = eConsole;

    pub fn try_new() -> VolumeResult<Self> {
        if INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err(VolumeControllerError::ComError(
                "COM is already initialized.".to_string(),
            ));
        }

        #[cfg(debug_assertions)]
        dbg!("ComScope initialized!");

        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .ok()
                .map_err(|e| {
                    VolumeControllerError::ComError(format!("Failed to initialize COM: {}", e))
                })?
        };

        let device_enumerator = unsafe {
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| {
                VolumeControllerError::ComError(format!("Failed to create instance COM: {}", e))
            })?
        };

        let event_context = GUID::new().map_err(|e| {
            VolumeControllerError::ComError(format!("Failed to create new GUID COM: {}", e))
        })?;

        Ok(Self {
            event_context,
            device_enumerator,
        })
    }

    pub fn get_event_context(&self) -> &GUID {
        &self.event_context
    }

    pub fn get_default_device(&self) -> VolumeResult<IMMDevice> {
        unsafe {
            self.device_enumerator
                .GetDefaultAudioEndpoint(eRender, Self::E_ROLE)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }

    pub fn with_default_generic_activate<T>(&self) -> VolumeResult<T>
    where
        T: Interface,
    {
        unsafe {
            self.get_default_device()?
                .Activate::<T>(Self::CLS_CONTEXT, None)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }

    pub fn with_generic_device_activate<T>(&self, id: &str) -> VolumeResult<T>
    where
        T: Interface,
    {
        let device = self.get_device_with_id(id)?;

        unsafe {
            device
                .Activate::<T>(Self::CLS_CONTEXT, None)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }

    pub fn get_all_device_id(&self) -> VolumeResult<Vec<String>> {
        unsafe {
            let device_collection = self
                .device_enumerator
                .EnumAudioEndpoints(eRender, Self::DEVICE_STATE_CONTEXT)?;

            let count = device_collection.GetCount()?;
            let mut ids = Vec::with_capacity(count as usize);

            for i in 0..count {
                match device_collection.Item(i) {
                    Ok(device) => match device.GetId() {
                        Ok(id_pw_str) => ids.push(util::pwstr_to_string(id_pw_str)),
                        Err(e) => eprintln!("Failed to get device ID: {}", e),
                    },
                    Err(e) => eprintln!("Failed to get device at index {}: {}", i, e),
                }
            }

            Ok(ids)
        }
    }

    pub fn get_device_with_id(&self, id: &str) -> VolumeResult<IMMDevice> {
        let (_buffer_pcw, pcw_str) = util::string_to_pcwstr(&id);

        unsafe {
            self.device_enumerator
                .GetDevice(pcw_str)
                .map_err(VolumeControllerError::WindowsApiError)
        }
    }
}
