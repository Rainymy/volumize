use windows::Win32::{
    Media::Audio::{EDataFlow, ERole, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{CoCreateInstance, CLSCTX_ALL},
};

mod pkey_value;
mod process;
mod pstring;

pub use pkey_value::*;
pub use process::*;
pub use pstring::*;

pub fn is_default_device(device: &IMMDevice, flow: EDataFlow, role: ERole) -> bool {
    unsafe {
        CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .ok()
            .and_then(|enumerator| enumerator.GetDefaultAudioEndpoint(flow, role).ok())
            .and_then(|default_device| {
                let device_id = device.GetId().ok()?.to_hstring();
                let default_id = default_device.GetId().ok()?.to_hstring();
                Some(device_id == default_id)
            })
            .unwrap_or_else(|| false)
    }
}

pub fn process_lossy_name(value: &[u16]) -> String {
    String::from_utf16_lossy(&value)
        .trim_end_matches('\0')
        .to_string()
}
