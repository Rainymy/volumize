use windows::{
    core::{Interface, PCWSTR},
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Foundation::MAX_PATH,
        Media::Audio::{eCapture, eRender, IMMDevice, IMMEndpoint},
        System::Com::{
            StructuredStorage::{PropVariantToString, PROPVARIANT},
            STGM_READ,
        },
    },
};

use super::VolumeController;
use crate::types::shared::{AudioDevice, DeviceControl, SessionDirection, VolumeResult};

use super::util;

fn process_device(device: IMMDevice) -> VolumeResult<AudioDevice> {
    // Get device ID
    let id = unsafe { util::pwstr_to_string(device.GetId()?) };

    // Get device name
    let name = unsafe {
        let props = device.OpenPropertyStore(STGM_READ)?;
        let name_prop: PROPVARIANT = props.GetValue(&PKEY_Device_FriendlyName)?;

        let mut buffer = vec![0u16; MAX_PATH as usize];
        PropVariantToString(&name_prop, &mut buffer)?;

        String::from_utf16_lossy(&buffer)
    };

    // Get device direction
    let direction = unsafe {
        let endpoint: IMMEndpoint = device.cast()?;

        #[allow(non_upper_case_globals)]
        match endpoint.GetDataFlow()? {
            eRender => SessionDirection::Render,
            eCapture => SessionDirection::Capture,
            _ => SessionDirection::Unknown,
        }
    };

    Ok(AudioDevice {
        id: id,
        name: name,
        direction: direction,
    })
}

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        let device_ids = self.com.get_all_device_id()?;

        let devices = device_ids
            .into_iter()
            .filter_map(|val| self.com.get_device_with_id(PCWSTR(val.as_ptr())).ok())
            .filter_map(|val| process_device(val).ok());

        Ok(devices.collect())
    }

    fn get_current_playback_device(&self) -> VolumeResult<AudioDevice> {
        process_device(self.com.get_default_device()?)
    }
}
