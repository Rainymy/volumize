use windows::{
    core::{Interface, PWSTR},
    Win32::{
        Devices::FunctionDiscovery::{PKEY_Device_DeviceDesc, PKEY_Device_FriendlyName},
        Foundation::{MAX_PATH, S_OK},
        Media::Audio::{
            eCapture, eRender, AudioSessionStateActive, EDataFlow, Endpoints::IAudioEndpointVolume,
            IAudioSessionControl2, IAudioSessionEnumerator, IMMDevice, IMMEndpoint,
            ISimpleAudioVolume,
        },
        UI::Shell::SHLoadIndirectString,
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
    pub edataflow: EDataFlow,
    pub direction: SessionDirection,
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

fn extract_audio_volume(volume: Option<f32>, is_mute: Option<bool>) -> AudioVolume {
    AudioVolume {
        current: volume.unwrap_or(0.0),
        muted: is_mute.unwrap_or(false),
    }
}

fn get_volume_info(device: &IMMDevice) -> AudioVolume {
    unsafe {
        if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(ComManager::CLS_CONTEXT, None)
        {
            // let scalar_volume = endpoint_volume.GetMasterVolumeLevelScalar().unwrap_or(0.0);
            let volume_level = endpoint.GetMasterVolumeLevelScalar().ok();
            let is_muted = endpoint.GetMute().map(|b| b.as_bool()).ok();

            return extract_audio_volume(volume_level, is_muted);
        }

        extract_audio_volume(None, None)
    }
}

fn get_volume_info_generic<T: Interface>(source: &T) -> AudioVolume {
    unsafe {
        if let Ok(endpoint) = source.cast::<ISimpleAudioVolume>() {
            // let scalar_volume = endpoint_volume.GetMasterVolumeLevelScalar().unwrap_or(0.0);
            let volume_level = endpoint.GetMasterVolume().ok();
            let is_muted = endpoint.GetMute().map(|b| b.as_bool()).ok();

            return extract_audio_volume(volume_level, is_muted);
        }

        extract_audio_volume(None, None)
    }
}

pub fn process_device(device: IMMDevice) -> VolumeResult<AudioDevice> {
    let name = util::get_pkey_value(&device, &PKEY_Device_FriendlyName)?;
    let friendly_name = util::get_pkey_value(&device, &PKEY_Device_DeviceDesc)?;

    let direction = get_direction(&device)?;

    Ok(AudioDevice {
        id: util::pwstr_to_string(unsafe { device.GetId()? }),
        name: name,
        friendly_name: friendly_name,
        direction: direction.direction,
        is_default: util::is_default_device(&device, direction.edataflow, ComManager::E_ROLE),
        volume: get_volume_info(&device),
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

            let is_active = session_control
                .GetState()
                .is_ok_and(|state| state == AudioSessionStateActive);

            let (process_name, process_path) = util::get_process_info(process_id);
            let display_name = get_session_display_name(&session_control);

            let final_name = if display_name.is_empty() {
                process_name
            } else {
                display_name
            };

            result.push(AudioApplication {
                process: ProcessInfo {
                    id: process_id,
                    name: final_name,
                    path: process_path,
                },
                session_type: determine_session_type(&session_control),
                direction: direction.clone().unwrap_or(SessionDirection::Unknown),
                volume: get_volume_info_generic(&session_control),
                sound_playing: is_active,
            });
        }
    }

    Ok(result)
}

fn determine_session_type(session_control: &IAudioSessionControl2) -> SessionType {
    unsafe {
        match session_control.IsSystemSoundsSession() {
            S_OK => SessionType::System,
            _ => SessionType::Application,
        }
    }
}

fn get_session_display_name(session_control: &IAudioSessionControl2) -> String {
    unsafe {
        let name_pwstr = session_control.GetDisplayName().unwrap_or(PWSTR::null());
        let raw_name = util::pwstr_to_string(name_pwstr);

        // Handle indirect strings (like @%SystemRoot%\System32\AudioSrv.DLL,-202)
        if raw_name.starts_with("@") {
            // Fall back to raw string if expansion fails
            expand_indirect_string(&raw_name)
                .inspect_err(|e| eprintln!("Failed to expand indirect string, {:?}", e))
                .unwrap_or_else(|_| raw_name)
        } else {
            raw_name
        }
    }
}

fn expand_indirect_string(indirect_string: &str) -> VolumeResult<String> {
    unsafe {
        let mut buffer = [0u16; MAX_PATH as usize];
        let (_buffer_str, read_str) = util::string_to_pcwstr(indirect_string);

        SHLoadIndirectString(read_str, &mut buffer, None)?;

        Ok(util::process_lossy_name(&buffer))
    }
}
