use windows::Win32::Media::Audio::IAudioSessionManager2;

use crate::types::shared::{
    AppIdentifier, ApplicationVolumeControl, AudioApplication, AudioVolume, DeviceControl,
    VolumeControllerError, VolumePercent, VolumeResult, VolumeValidation,
};

use super::{com_scope::ComManager, convert, convert::get_direction, util, VolumeController};

impl VolumeController {}

impl ApplicationVolumeControl for VolumeController {
    fn get_application(&self, app: AppIdentifier) -> VolumeResult<AudioApplication> {
        for device in self.get_playback_devices()? {
            let session_enums = {
                let session: IAudioSessionManager2 =
                    self.com.with_generic_device_activate(&device.id)?;

                unsafe { session.GetSessionEnumerator()? }
            };

            let imm_device = self.com.get_device_with_id(&device.id)?;
            let direction = get_direction(&imm_device)?;

            let is_default_device =
                util::is_default_device(&imm_device, direction.edataflow, ComManager::E_ROLE);

            let device_applications = convert::process_sessions(
                &session_enums,
                Some(direction.direction),
                is_default_device,
                &util::pwstr_to_string(unsafe { imm_device.GetId()? }),
            )?;

            let application = device_applications
                .into_iter()
                .find(|val| val.process.id == app);

            if let Some(application) = application {
                return Ok(application);
            }
        }

        Err(VolumeControllerError::ApplicationNotFound(format!(
            "[ find ] Application not found - id: {}",
            app
        )))
    }

    fn get_app_volume(&self, app: AppIdentifier) -> VolumeResult<AudioVolume> {
        Ok(self.get_application(app)?.volume)
    }

    fn set_app_volume(&self, app: AppIdentifier, volume: VolumePercent) -> VolumeResult<()> {
        let volume = AudioVolume::validate_volume(volume)?;

        let app = self.get_application(app)?;
        let endpoint = self
            .com
            .with_application_session_control(app.process.id, &app.device_id)?;

        unsafe {
            endpoint
                .SetMasterVolume(volume.current, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to mute the application - id: {} \n {:?}",
                        app.process.id, err
                    ))
                })
        }
    }

    fn mute_app(&self, app: AppIdentifier) -> VolumeResult<()> {
        let app = self.get_application(app)?;
        let endpoint = self
            .com
            .with_application_session_control(app.process.id, &app.device_id)?;

        unsafe {
            endpoint
                .SetMute(true, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to mute the application - id: {} \n {:?}",
                        app.process.id, err
                    ))
                })
        }
    }

    fn unmute_app(&self, app: AppIdentifier) -> VolumeResult<()> {
        let app = self.get_application(app)?;
        let endpoint = self
            .com
            .with_application_session_control(app.process.id, &app.device_id)?;

        unsafe {
            endpoint
                .SetMute(false, self.com.get_event_context())
                .map_err(|err| {
                    VolumeControllerError::OsApiError(format!(
                        "Unable to unmute the application - id: {} \n {:?}",
                        app.process.id, err
                    ))
                })
        }
    }
}
