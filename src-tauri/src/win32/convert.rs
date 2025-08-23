use windows::{
    core::{Interface, PWSTR},
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Foundation::{MAX_PATH, S_OK},
        Media::Audio::{
            eCapture, eRender, AudioSessionStateActive, EDataFlow, IAudioSessionControl2,
            IAudioSessionEnumerator, IMMDevice, IMMEndpoint, ISimpleAudioVolume,
        },
        System::Com::{
            StructuredStorage::{PropVariantToString, PROPVARIANT},
            STGM_READ,
        },
    },
};

use crate::{
    types::shared::{
        AudioApplication, AudioDevice, AudioVolume, ProcessInfo, SessionDirection, SessionType,
        VolumeResult,
    },
    volume_control::platform::com_scope::ComManager,
};

use super::util;

pub struct IDirection {
    edataflow: EDataFlow,
    direction: SessionDirection,
}

pub fn get_direction(device: &impl Interface) -> VolumeResult<IDirection> {
    let dataflow = unsafe { device.cast::<IMMEndpoint>()?.GetDataFlow()? };

    #[allow(non_upper_case_globals)]
    let direction = match dataflow {
        eRender => SessionDirection::Render,
        eCapture => SessionDirection::Capture,
        _ => SessionDirection::Unknown,
    };

    Ok(IDirection {
        direction: direction,
        edataflow: dataflow,
    })
}

pub fn process_device(device: IMMDevice) -> VolumeResult<AudioDevice> {
    // Get device name
    let name = unsafe {
        let props = device.OpenPropertyStore(STGM_READ)?;
        let name_prop: PROPVARIANT = props.GetValue(&PKEY_Device_FriendlyName)?;

        let mut buffer = [0u16; MAX_PATH as usize];
        PropVariantToString(&name_prop, &mut buffer)?;

        util::process_lossy_name(&buffer)
    };

    let direction = get_direction(&device)?;

    Ok(AudioDevice {
        id: util::pwstr_to_string(unsafe { device.GetId()? }),
        name: name,
        direction: direction.direction,
        is_default: util::is_default_device(&device, direction.edataflow, ComManager::E_ROLE),
    })
}

pub fn process_sessions(
    sessions: &IAudioSessionEnumerator,
    direction: Option<SessionDirection>,
) -> VolumeResult<Vec<AudioApplication>> {
    let mut result: Vec<AudioApplication> = Vec::new();

    unsafe {
        for i in 0..sessions.GetCount()? {
            let session_control: IAudioSessionControl2 = sessions.GetSession(i)?.cast()?;

            let process_id = session_control.GetProcessId()?;
            let (process_name, process_path) = util::get_process_info(process_id);

            let (current_volume, is_muted) = match session_control.cast::<ISimpleAudioVolume>() {
                Ok(simple_volume) => {
                    let muted = simple_volume
                        .GetMute()
                        .map(|b| b.as_bool())
                        .unwrap_or(false);
                    let current = simple_volume.GetMasterVolume().unwrap_or(0.0);
                    (current, muted)
                }
                Err(_) => (0.0, false),
            };

            let session_type = match session_control.IsSystemSoundsSession() {
                S_OK => SessionType::System,
                _ => SessionType::Application,
            };

            // Use process name if available, otherwise fall back to display name
            let final_name = if !process_name.is_empty() {
                process_name
            } else {
                let name_pwstr = session_control.GetDisplayName().unwrap_or(PWSTR::null());
                util::pwstr_to_string(name_pwstr)
            };

            let is_active = session_control.GetState().ok() == Some(AudioSessionStateActive);

            result.push(AudioApplication {
                process: ProcessInfo {
                    id: process_id,
                    name: final_name,
                    path: process_path,
                },
                session_type: session_type,
                direction: direction.clone().unwrap_or(SessionDirection::Unknown),
                volume: AudioVolume {
                    current: current_volume,
                    muted: is_muted,
                },
                sound_playing: is_active,
            });
        }
    }

    Ok(result)
}
