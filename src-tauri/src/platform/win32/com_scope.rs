use std::sync::atomic::{AtomicBool, Ordering};

use windows::{
    core::{Error, Interface, Result as WinResult, GUID},
    Win32::{
        Foundation::E_FAIL,
        Media::Audio::{
            eConsole, eRender, EDataFlow, ERole, IAudioSessionControl2, IAudioSessionManager2,
            IMMDevice, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator, DEVICE_STATE,
            DEVICE_STATE_ACTIVE,
        },
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX, CLSCTX_ALL,
            COINIT_MULTITHREADED,
        },
    },
};

use super::util;
use crate::types::shared::DeviceIdentifier;

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
    pub const E_DATAFLOW: EDataFlow = eRender;

    pub fn try_new() -> WinResult<Self> {
        if INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err(Error::new(E_FAIL, "COM is already initialized."));
        }

        #[cfg(debug_assertions)]
        dbg!("ComScope initialized!");

        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };

        Ok(Self {
            event_context: GUID::new()?,
            device_enumerator: unsafe {
                CoCreateInstance(&MMDeviceEnumerator, None, Self::CLS_CONTEXT)?
            },
        })
    }

    pub fn get_event_context(&self) -> &GUID {
        &self.event_context
    }

    pub fn with_application_session_control(
        &self,
        target_pid: u32,
        device_id: &str,
    ) -> WinResult<ISimpleAudioVolume> {
        let endpoint: IAudioSessionManager2 = self.with_generic_device_activate(&device_id)?;

        let session_enum = unsafe { endpoint.GetSessionEnumerator() }?;
        let count = unsafe { session_enum.GetCount() }?;

        for i in 0..count {
            let session_control = unsafe { session_enum.GetSession(i) }?;

            let session_control2 = session_control.cast::<IAudioSessionControl2>()?;
            let pid = unsafe { session_control2.GetProcessId()? };

            if pid == target_pid {
                return Ok(session_control.cast::<ISimpleAudioVolume>()?);
            }
        }

        let error_msg = format!(
            "Could not find any application with: {} - device: {}",
            target_pid, device_id
        );
        Err(Error::new(E_FAIL, error_msg))
    }

    pub fn with_generic_device_activate<T>(&self, id: &str) -> WinResult<T>
    where
        T: Interface,
    {
        unsafe {
            self.get_device_with_id(id)?
                .Activate::<T>(Self::CLS_CONTEXT, None)
        }
    }

    pub fn get_all_device_id(&self) -> WinResult<Vec<DeviceIdentifier>> {
        unsafe {
            let device_collection = self
                .device_enumerator
                .EnumAudioEndpoints(Self::E_DATAFLOW, Self::DEVICE_STATE_CONTEXT)?;

            let count = device_collection.GetCount()?;
            let mut ids: Vec<DeviceIdentifier> = Vec::with_capacity(count as usize);

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

    pub fn get_device_with_id(&self, id: &str) -> WinResult<IMMDevice> {
        let (_buffer_pcw, pcw_str) = util::string_to_pcwstr(&id);

        unsafe { self.device_enumerator.GetDevice(pcw_str) }
    }
}
