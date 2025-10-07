use windows::Win32::Media::Audio::{IAudioSessionManager2, ISimpleAudioVolume};

use crate::{
    server::volume_control::platform::{com_scope::ComManager, convert::get_direction},
    types::shared::{
        AppIdentifier, ApplicationVolumeControl, AudioApplication, AudioDevice, AudioSession,
        AudioVolume, DeviceControl, VolumeControllerError, VolumePercent, VolumeResult,
        VolumeValidation,
    },
};

use super::{convert, util, VolumeController};

impl VolumeController {
    fn get_application_device(&self, app: AppIdentifier) -> VolumeResult<AudioDevice> {
        let session = self
            .get_all_applications()?
            .into_iter()
            .find(|val| val.applications.iter().any(|val| val.process.id == app))
            .ok_or_else(|| {
                VolumeControllerError::ApplicationNotFound(format!(
                    "[ get_application_device ] Application not found - id: {}",
                    app
                ))
            })?;

        Ok(session.device)
    }

    fn get_application_session_control(
        &self,
        app: AppIdentifier,
    ) -> VolumeResult<ISimpleAudioVolume> {
        let id = self.get_application_device(app)?.id;
        self.com.with_application_sesstion_control(app, &id)
    }
}

impl ApplicationVolumeControl for VolumeController {
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        let mut applications = vec![];

        for device in self.get_playback_devices()? {
            let device_id = device.id;

            let session_enums = {
                let session: IAudioSessionManager2 =
                    self.com.with_generic_device_activate(&device_id)?;

                unsafe { session.GetSessionEnumerator()? }
            };

            let imm_device = self.com.get_device_with_id(&device_id)?;
            let direction = get_direction(&imm_device)?;

            let current_device = self.com.get_device_with_id(&device_id)?;

            let is_default_device =
                util::is_default_device(&current_device, direction.edataflow, ComManager::E_ROLE);

            applications.push(AudioSession {
                applications: convert::process_sessions(
                    &session_enums,
                    Some(direction.direction),
                    is_default_device,
                )?,
                device: convert::process_device(imm_device)?,
            });
        }

        Ok(applications)
    }

    fn find_application_with_id(&self, app: AppIdentifier) -> VolumeResult<AudioApplication> {
        self.get_all_applications()?
            .into_iter()
            .flat_map(|session| session.applications.into_iter())
            .find(|val| val.process.id == app)
            .ok_or_else(|| {
                VolumeControllerError::ApplicationNotFound(format!(
                    "[ find ] Application not found - id: {}",
                    app
                ))
            })
    }

    fn get_app_volume(&self, app: AppIdentifier) -> VolumeResult<AudioVolume> {
        Ok(self.find_application_with_id(app)?.volume)
    }

    fn set_app_volume(&self, app: AppIdentifier, volume: VolumePercent) -> VolumeResult<()> {
        let volume = AudioVolume::validate_volume(volume)?;
        let endpoint = self.get_application_session_control(app)?;

        unsafe {
            endpoint
                .SetMasterVolume(volume.current, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to mute the application - id: {} \n {:?}",
                        app, err
                    ))
                })
        }
    }

    fn mute_app(&self, app: AppIdentifier) -> VolumeResult<()> {
        let endpoint = self.get_application_session_control(app)?;

        unsafe {
            endpoint
                .SetMute(true, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to mute the application - id: {} \n {:?}",
                        app, err
                    ))
                })
        }
    }

    fn unmute_app(&self, app: AppIdentifier) -> VolumeResult<()> {
        let endpoint = self.get_application_session_control(app)?;

        unsafe {
            endpoint
                .SetMute(false, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to unmute the application - id: {} \n {:?}",
                        app, err
                    ))
                })
        }
    }
}
