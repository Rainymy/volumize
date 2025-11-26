use windows::{
    core::{Interface, PWSTR},
    Win32::{
        Devices::FunctionDiscovery::{PKEY_Device_DeviceDesc, PKEY_Device_FriendlyName},
        Foundation::{MAX_PATH, S_OK},
        Media::Audio::{
            eCapture, eRender, EDataFlow, Endpoints::IAudioEndpointVolume, IAudioSessionControl2,
            IAudioSessionEnumerator, IMMDevice, IMMEndpoint, ISimpleAudioVolume,
        },
        UI::Shell::SHLoadIndirectString,
    },
};

use crate::types::shared::{
    AudioApplication, AudioDevice, AudioVolume, DeviceIdentifier, ProcessInfo, SessionDirection,
    SessionType, VolumeResult,
};

use super::{com_scope::ComManager, util};

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
    is_default_deivce: bool,
    device_id: &DeviceIdentifier,
) -> VolumeResult<Vec<AudioApplication>> {
    let mut applications = vec![];

    unsafe {
        for i in 0..sessions.GetCount()? {
            let session_control: IAudioSessionControl2 = sessions.GetSession(i)?.cast()?;
            let process_id = session_control.GetProcessId()?;

            let is_system_sound = session_control.IsSystemSoundsSession().is_ok();
            if is_system_sound && !is_default_deivce {
                continue;
            }

            applications.push(AudioApplication {
                process: ProcessInfo {
                    id: process_id,
                    name: get_display_name(&session_control, process_id),
                    path: util::get_process_info(process_id).1,
                },
                session_type: determine_session_type(&session_control),
                direction: direction.clone().unwrap_or(SessionDirection::Unknown),
                volume: get_volume_info_generic(&session_control),
                device_id: device_id.clone(),
            });
        }
    }

    Ok(applications)
}

fn determine_session_type(session_control: &IAudioSessionControl2) -> SessionType {
    unsafe {
        match session_control.IsSystemSoundsSession() {
            S_OK => SessionType::System,
            _ => SessionType::Application,
        }
    }
}

pub fn get_display_name(session_control: &IAudioSessionControl2, pid: u32) -> String {
    let display_name = get_session_display_name(&session_control);

    if !display_name.is_empty() {
        return display_name;
    }

    let (process_name, process_path) = util::get_process_info(pid);
    if let Some(path) = process_path {
        // Read Executeable FileDescription.
        let window_title = util::get_main_window_title(&path);
        return window_title.unwrap_or(process_name);
    }

    process_name
}

fn get_session_display_name(session_control: &IAudioSessionControl2) -> String {
    let raw_name = unsafe {
        let name_pwstr = session_control.GetDisplayName().unwrap_or(PWSTR::null());
        util::pwstr_to_string(name_pwstr)
    };
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

fn expand_indirect_string(indirect_string: &str) -> VolumeResult<String> {
    let mut buffer = [0u16; MAX_PATH as usize];
    let (_buffer_str, read_str) = util::string_to_pcwstr(indirect_string);

    unsafe {
        SHLoadIndirectString(read_str, &mut buffer, None)?;
    }
    Ok(util::process_lossy_name(&buffer))
}
