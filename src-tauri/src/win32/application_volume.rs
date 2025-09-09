use windows::Win32::Media::Audio::{Endpoints::IAudioEndpointVolume, IAudioSessionManager2};

use crate::{
    types::shared::{
        AppIdentifier, ApplicationVolumeControl, AudioApplication, AudioDevice, AudioSession,
        AudioVolume, DeviceControl, VolumeControllerError, VolumePercent, VolumeResult,
        VolumeValidation,
    },
    volume_control::platform::convert::get_direction,
};

use super::{convert, VolumeController};

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

    fn get_application_device2(&self, app: AppIdentifier) -> VolumeResult<AudioDevice> {
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
            let direction = get_direction(&imm_device)?.direction;

            applications.push(AudioSession {
                applications: convert::process_sessions(&session_enums, Some(direction))?,
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
        AudioVolume::validate_volume(volume)?;

        let session = self
            .get_all_applications()?
            .into_iter()
            .find(|val| val.applications.iter().any(|val| val.process.id == app))
            .ok_or_else(|| {
                VolumeControllerError::ApplicationNotFound(format!(
                    "[ set_app_volume ] Application not found - id: {}",
                    app
                ))
            })?;

        let endpoint: IAudioEndpointVolume =
            self.com.with_generic_device_activate(&session.device.id)?;

        let guid = self.com.get_event_context();
        unsafe {
            for index in 0..endpoint.GetChannelCount()? {
                endpoint.SetChannelVolumeLevelScalar(index, volume, guid)?;
            }
        };

        Ok(())
    }

    fn mute_app(&self, app: AppIdentifier) -> VolumeResult<()> {
        let device = self.get_application_device(app.clone())?;
        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device.id)?;

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
        let device = self.get_application_device(app.clone())?;
        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device.id)?;

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
