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
        let data = unsafe { &*pnotify };

        println!("Device volume changed to {}", data.fMasterVolume);
        println!("Muted: {}", data.bMuted.as_bool());

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
        println!("[APP: {}] Display name changed", self.app_name);
        Ok(())
    }

    fn OnIconPathChanged(&self, _newiconpath: &PCWSTR, _eventcontext: *const GUID) -> Result<()> {
        println!("[APP: {}] Icon path changed", self.app_name);
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
        println!("[APP: {}] Channel volume changed", self.app_name);
        Ok(())
    }

    fn OnGroupingParamChanged(
        &self,
        _newgroupingparam: *const GUID,
        _eventcontext: *const GUID,
    ) -> Result<()> {
        println!("[APP: {}] Grouping param changed", self.app_name);
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
        let session = match &*newsession {
            Some(session) => session,
            None => return Ok(()),
        };
        let session2 = session.cast::<IAudioSessionControl2>()?;
        let pid = unsafe { session2.GetProcessId() }?;

        let display_name = super::convert::get_display_name(&session2, pid);

        unsafe {
            println!("[NEW SESSION] {}: {}", display_name, pid);
            let events: IAudioSessionEvents = SessionEvents::new(display_name).into();
            session
                .RegisterAudioSessionNotification(&events)
                .inspect_err(|e| eprintln!("ERROR: {e}"))?;
        }
        Ok(())
    }
}

use super::com_scope::ComManager;

type RDevice = (IAudioEndpointVolume, IAudioEndpointVolumeCallback);

fn register_device(manager: &ComManager, device_id: &str) -> Result<RDevice> {
    let endpoint_volume: IAudioEndpointVolume =
        match manager.with_generic_device_activate(&device_id) {
            Ok(device) => device,
            Err(_err) => return Err(windows::core::Error::from_win32()),
        };

    let callback: IAudioEndpointVolumeCallback = VolumeCallback.into();

    match unsafe { endpoint_volume.RegisterControlChangeNotify(&callback) } {
        Ok(_) => Ok((endpoint_volume, callback)),
        Err(err) => Err(err),
    }
}

type RSNotice = (IAudioSessionManager2, IAudioSessionNotification);

fn register_session_notification(manager: &IAudioSessionManager2) -> Result<RSNotice> {
    let session_notification: IAudioSessionNotification = SessionNotification.into();
    unsafe { manager.RegisterSessionNotification(&session_notification) }?;

    Ok((manager.clone(), session_notification))
}

type RAEvents = (IAudioSessionControl, IAudioSessionEvents);

fn register_application_notification(manager: &IAudioSessionManager2) -> Result<Vec<RAEvents>> {
    let mut callbacks = vec![];

    let session_enum = unsafe { manager.GetSessionEnumerator()? };
    let count = unsafe { session_enum.GetCount()? };

    println!("Found {} existing audio sessions", count);
    for i in 0..count {
        let session = unsafe { session_enum.GetSession(i)? };

        let session_control: IAudioSessionControl2 = session.cast()?;
        let pid = unsafe { session_control.GetProcessId()? };

        let display_name = super::convert::get_display_name(&session_control, pid);
        println!(" - [EXISTING SESSION] {}", display_name);

        let events: IAudioSessionEvents = SessionEvents::new(display_name).into();

        unsafe {
            match session.RegisterAudioSessionNotification(&events) {
                Ok(_) => callbacks.push((session, events)),
                Err(e) => eprintln!("Error register audio session notification: {}", e),
            }
        }
    }

    Ok(callbacks)
}

pub struct AudioMonitor {
    device_callback: Vec<RDevice>,
    session_notification: Vec<RSNotice>,
    sessions_application: Vec<RAEvents>,
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
                Ok(callback) => callbacks.push(callback),
                Err(_) => {
                    eprintln!("Error registering device: {}", device_id);
                    continue;
                }
            };

            let session_manager: IAudioSessionManager2 = match manager
                .with_generic_device_activate(&device_id)
                .map_err(|_| windows::core::Error::from_win32())
            {
                Ok(manager) => manager,
                Err(e) => {
                    eprintln!("Error activating generic device: {}", e);
                    continue;
                }
            };

            match register_session_notification(&session_manager) {
                Ok(session_notification) => callbacks_session.push(session_notification),
                Err(e) => {
                    eprintln!("Error session notification for device: {}", e);
                    continue;
                }
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
