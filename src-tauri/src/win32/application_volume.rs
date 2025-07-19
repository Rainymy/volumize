use windows::{
    core::{Interface, PWSTR},
    Win32::Media::Audio::{
        AudioSessionStateActive, IAudioSessionControl2, IAudioSessionEnumerator, ISimpleAudioVolume,
    },
};

use super::VolumeController;
use crate::{
    platform::util,
    types::shared::{
        AppIdentifier, ApplicationVolumeControl, AudioDevice, AudioSession, AudioVolume,
        ProcessInfo, SessionDirection, SessionType, VolumePercent, VolumeResult,
    },
};

unsafe fn process_sessions(sessions: &IAudioSessionEnumerator) -> VolumeResult<Vec<AudioSession>> {
    let count = sessions.GetCount()?;
    let mut result: Vec<AudioSession> = Vec::new();

    for i in 0..count {
        let session_control: IAudioSessionControl2 = sessions.GetSession(i)?.cast()?;

        let name_pwstr = session_control.GetDisplayName().unwrap_or(PWSTR::null());
        let is_active = session_control.GetState().ok() == Some(AudioSessionStateActive);

        // Get Process info
        let process_id = session_control.GetProcessId().unwrap_or(0);
        let (process_name, process_path) = if process_id != 0 {
            util::get_process_info(process_id).unwrap_or((String::new(), None))
        } else {
            (String::new(), None)
        };

        // Get volume information
        let (current_volume, is_muted) = match session_control.cast::<ISimpleAudioVolume>() {
            Ok(simple_volume) => {
                let volume = simple_volume.GetMasterVolume().unwrap_or(0.0);
                dbg!(simple_volume.GetMute().map(|b| b.as_bool()).ok());
                let muted = simple_volume
                    .GetMute()
                    .map(|b| b.as_bool())
                    .unwrap_or(false);
                (volume, muted)
            }
            Err(_) => (1.0, false),
        };

        // Determine session type
        let session_type = match session_control.IsSystemSoundsSession().ok() {
            Ok(()) => SessionType::System,
            Err(_) => SessionType::Application,
        };

        // Use process name if available, otherwise fall back to display name
        let final_name = if !process_name.is_empty() {
            process_name
        } else {
            util::pwstr_to_string(name_pwstr)
        };

        result.push(AudioSession {
            process: ProcessInfo {
                id: process_id,
                name: final_name,
                path: process_path,
            },
            session_type: session_type,
            direction: SessionDirection::Render,
            device: AudioDevice {
                id: String::new(),
                name: String::new(),
                direction: SessionDirection::Render,
            },
            volume: AudioVolume {
                current: current_volume,
                muted: is_muted,
            },
            sound_playing: is_active,
        });
    }

    Ok(result)
}

impl ApplicationVolumeControl for VolumeController {
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        self.with_default_audio_sessions_manager2(|endpoint| unsafe {
            process_sessions(&endpoint.GetSessionEnumerator()?)
        })
    }

    fn set_app_volume(&self, _app: AppIdentifier, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }
}
