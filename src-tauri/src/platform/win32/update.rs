use windows::{
    core::{Interface, Ref, Result, BOOL, GUID, PCWSTR},
    Win32::Media::Audio::{
        AudioSessionDisconnectReason, AudioSessionState,
        Endpoints::{
            IAudioEndpointVolume, IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Impl,
        },
        IAudioSessionControl, IAudioSessionControl2, IAudioSessionEvents, IAudioSessionEvents_Impl,
        IAudioSessionManager2, IAudioSessionNotification, IAudioSessionNotification_Impl,
        AUDIO_VOLUME_NOTIFICATION_DATA,
    },
};
use windows_core::implement;

#[implement(IAudioEndpointVolumeCallback)]
struct VolumeCallback;

impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
    fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> Result<()> {
        unsafe {
            let data = &*pnotify;
            println!("Device volume changed to {}", data.fMasterVolume);
            println!("Muted: {}", data.bMuted.as_bool());
        }
        Ok(())
    }
}

#[implement(IAudioSessionEvents)]
struct SessionEvents {
    app_name: String,
}

impl SessionEvents {
    fn new(app_name: String) -> Self {
        Self { app_name }
    }
}

impl IAudioSessionEvents_Impl for SessionEvents_Impl {
    fn OnDisplayNameChanged(
        &self,
        _newdisplayname: &PCWSTR,
        _eventcontext: *const GUID,
    ) -> Result<()> {
        Ok(())
    }

    fn OnIconPathChanged(&self, _newiconpath: &PCWSTR, _eventcontext: *const GUID) -> Result<()> {
        Ok(())
    }

    fn OnSimpleVolumeChanged(
        &self,
        newvolume: f32,
        newmute: BOOL,
        _eventcontext: *const GUID,
    ) -> Result<()> {
        println!(
            "[APP: {}] Volume: {:.0}%, Muted: {}",
            self.app_name,
            newvolume * 100.0,
            newmute.as_bool()
        );
        Ok(())
    }

    fn OnChannelVolumeChanged(
        &self,
        _channelcount: u32,
        _newchannelvolumearray: *const f32,
        _changedchannel: u32,
        _eventcontext: *const GUID,
    ) -> Result<()> {
        Ok(())
    }

    fn OnGroupingParamChanged(
        &self,
        _newgroupingparam: *const GUID,
        _eventcontext: *const GUID,
    ) -> Result<()> {
        Ok(())
    }

    fn OnStateChanged(&self, newstate: AudioSessionState) -> Result<()> {
        println!("[APP: {}] State changed: {:?}", self.app_name, newstate);
        Ok(())
    }

    fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> Result<()> {
        println!(
            "[APP: {}] Disconnected: {:?}",
            self.app_name, disconnectreason
        );
        Ok(())
    }
}

#[implement(IAudioSessionNotification)]
struct SessionNotification;

impl IAudioSessionNotification_Impl for SessionNotification_Impl {
    fn OnSessionCreated(&self, newsession: Ref<IAudioSessionControl>) -> Result<()> {
        if let Some(session) = &*newsession {
            unsafe {
                let session2: IAudioSessionControl2 = session.cast()?;
                let name = super::util::pwstr_to_string(session2.GetDisplayName()?);

                println!("[NEW SESSION] {}", name);
                let events: IAudioSessionEvents = SessionEvents::new(name).into();
                session.RegisterAudioSessionNotification(&events)?;
            }
        }
        Ok(())
    }
}

use super::com_scope::ComManager;

fn register_device(
    manager: &ComManager,
    device_id: &str,
) -> Option<(IAudioEndpointVolume, IAudioEndpointVolumeCallback)> {
    let endpoint_volume: IAudioEndpointVolume = match manager.with_device_id_activate(&device_id) {
        Ok(device) => device,
        Err(err) => {
            eprintln!("Failed to get device with ID {}: {}", device_id, err);
            return None;
        }
    };

    let callback: IAudioEndpointVolumeCallback = VolumeCallback.into();

    match unsafe { endpoint_volume.RegisterControlChangeNotify(&callback) } {
        Ok(_) => Some((endpoint_volume, callback)),
        Err(err) => {
            eprintln!("Error register callback [{}]: {}", device_id, err);
            None
        }
    }
}

fn register_session_notification(
    manager: &IAudioSessionManager2,
    device_id: &str,
) -> Option<(IAudioSessionManager2, IAudioSessionNotification)> {
    let session_notification: IAudioSessionNotification = SessionNotification.into();

    match unsafe { manager.RegisterSessionNotification(&session_notification) } {
        Ok(_) => Some((manager.clone(), session_notification)),
        Err(err) => {
            eprintln!(
                "Failed to register session notification for device {}: {}",
                device_id, err
            );
            None
        }
    }
}

fn get_app_name(control: &IAudioSessionControl) -> Result<String> {
    unsafe {
        let session = control.cast::<IAudioSessionControl2>()?;
        let display_name = super::util::pwstr_to_string(session.GetDisplayName()?);

        Ok(display_name)
    }
}

fn register_application_notification(
    manager: &IAudioSessionManager2,
) -> windows::core::Result<Vec<(IAudioSessionControl, IAudioSessionEvents)>> {
    let mut callbacks = vec![];

    let session_enum = unsafe { manager.GetSessionEnumerator()? };
    let count = unsafe { session_enum.GetCount()? };

    println!("Found {} existing audio sessions", count);
    for i in 0..count {
        unsafe {
            let session = session_enum.GetSession(i)?;
            let app_name = get_app_name(&session)?;

            // let app_name = super::util::get_pkey_name(manager, &PKEY_Device_FriendlyName)?;

            println!(" - [EXISTING SESSION] {}", app_name);

            let events: IAudioSessionEvents = SessionEvents::new(app_name).into();
            match session.RegisterAudioSessionNotification(&events) {
                Ok(_) => callbacks.push((session, events)),
                Err(e) => eprintln!("Error register audio session notification: {}", e),
            }
        }
    }

    Ok(callbacks)
}

pub struct AudioMonitor {
    device_callback: Vec<(IAudioEndpointVolume, IAudioEndpointVolumeCallback)>,
    session_notification: Vec<(IAudioSessionManager2, IAudioSessionNotification)>,
    sessions_application: Vec<(IAudioSessionControl, IAudioSessionEvents)>,
}

impl AudioMonitor {
    pub fn new() -> Self {
        Self {
            device_callback: Vec::new(),
            session_notification: Vec::new(),
            sessions_application: Vec::new(),
        }
    }
    pub fn register_callbacks(&mut self, manager: &ComManager) {
        let mut callbacks = vec![];
        let mut callbacks_session = vec![];
        let mut callbacks_application = vec![];

        println!("\nMonitoring device volume changes...\n");

        for device_id in manager.get_all_device_id().unwrap_or_default() {
            match register_device(&manager, &device_id) {
                Some(callback) => callbacks.push(callback),
                None => continue,
            };

            let session_manager: IAudioSessionManager2 =
                match manager.with_generic_device_activate(&device_id) {
                    Ok(session_manager) => session_manager,
                    Err(err) => {
                        eprintln!(
                            "Failed to get session manager for device {}: {}",
                            device_id, err
                        );
                        continue;
                    }
                };

            match register_session_notification(&session_manager, &device_id) {
                Some(session_notification) => callbacks_session.push(session_notification),
                None => continue,
            };

            match register_application_notification(&session_manager) {
                Ok(events) => callbacks_application.extend(events),
                Err(e) => {
                    eprintln!("Error register application notification: {}", e);
                    continue;
                }
            };
        }

        self.device_callback.extend(callbacks);
        self.session_notification.extend(callbacks_session);
        self.sessions_application.extend(callbacks_application);
    }

    pub fn unregister_callbacks(&mut self) {
        for (endpoint, sessions) in &self.device_callback {
            let _ = unsafe { endpoint.UnregisterControlChangeNotify(sessions) };
        }

        for (manager, sessions) in &self.session_notification {
            let _ = unsafe { manager.UnregisterSessionNotification(sessions) };
        }

        for (control, sessions) in &self.sessions_application {
            let _ = unsafe { control.UnregisterAudioSessionNotification(sessions) };
        }

        self.device_callback.clear();
        self.session_notification.clear();
        self.sessions_application.clear();
    }
}
