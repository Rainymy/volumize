use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc, Mutex,
    },
};

use windows::{
    core::{implement, Interface, Ref, Result as WinResult, BOOL, GUID, PCWSTR},
    Win32::Media::Audio::{
        AudioSessionDisconnectReason, AudioSessionState, AudioSessionStateActive,
        AudioSessionStateExpired, AudioSessionStateInactive,
        Endpoints::{
            IAudioEndpointVolume, IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Impl,
        },
        IAudioSessionControl, IAudioSessionControl2, IAudioSessionEvents, IAudioSessionEvents_Impl,
        IAudioSessionManager2, IAudioSessionNotification, IAudioSessionNotification_Impl,
        AUDIO_VOLUME_NOTIFICATION_DATA,
    },
};
// use windows_core::implement;

use crate::types::shared::{DeviceIdentifier, EntityState, Identifier, UpdateChange};

#[derive(Clone)]
struct AudioInfo {
    name: String,
    sessions: Vec<IAudioSessionControl>,
}

type AppMap = HashMap<u32, AudioInfo>;
type VolumeSender = Sender<UpdateChange>;

#[implement(IAudioEndpointVolumeCallback)]
struct VolumeCallback {
    sender: VolumeSender,
    id: DeviceIdentifier,
}

impl VolumeCallback {
    fn new(id: DeviceIdentifier, sender: VolumeSender) -> Self {
        Self { sender, id }
    }
}

impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
    fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> WinResult<()> {
        let data = unsafe { &*pnotify };

        println!("Device volume changed to {}", data.fMasterVolume);
        println!("Muted: {}", data.bMuted.as_bool());

        let update = UpdateChange::volume_change(
            Identifier::Device(self.id.clone()),
            data.fMasterVolume,
            data.bMuted.as_bool(),
        );
        let _ = self.sender.send(update);

        Ok(())
    }
}

#[implement(IAudioSessionEvents)]
struct SessionEvents {
    name: String,
    pid: u32,
    session: IAudioSessionControl,
    apps: Arc<Mutex<AppMap>>,
    needs_reinit: Arc<AtomicBool>,
    sender: VolumeSender,
}

impl SessionEvents {
    fn is_expire_state(&self, newstate: AudioSessionState) -> bool {
        #[allow(nonstandard_style)]
        match newstate {
            AudioSessionStateActive | AudioSessionStateInactive => false,
            AudioSessionStateExpired | _ => true,
        }
    }

    fn cleanup_session(&self) {
        let mut app_map = match self.apps.lock() {
            Ok(map) => map,
            Err(_) => return (),
        };

        let entry = match app_map.get_mut(&self.pid) {
            Some(entry) => entry,
            None => return (),
        };

        // Erase current session entry in-place - using built-in equality check:
        // - NOTE: Need to check COM *value* not Rust address.
        entry.sessions.retain(|s| s.ne(&self.session));

        println!(
            "[{}] Removed session. Remaining: {}",
            entry.name,
            entry.sessions.len()
        );

        if entry.sessions.is_empty() {
            println!("[{}] All sessions closed: pid = {}", entry.name, self.pid);
            let _ = app_map.remove(&self.pid);
        }
    }
}

impl IAudioSessionEvents_Impl for SessionEvents_Impl {
    fn OnDisplayNameChanged(
        &self,
        _newdisplayname: &PCWSTR,
        _eventcontext: *const GUID,
    ) -> WinResult<()> {
        let new_name = super::util::pcwstr_to_string(_newdisplayname);
        let update = UpdateChange::app_name_change(Identifier::App(self.pid), new_name);
        let _ = self.sender.send(update);
        println!("[{}] Display name changed", self.name);
        Ok(())
    }

    fn OnIconPathChanged(
        &self,
        _newiconpath: &PCWSTR,
        _eventcontext: *const GUID,
    ) -> WinResult<()> {
        let new_icon_path = super::util::pcwstr_to_string(_newiconpath);
        let update = UpdateChange::app_icon_change(Identifier::App(self.pid), new_icon_path);
        let _ = self.sender.send(update);
        println!("[{}] Icon path changed", self.name);
        Ok(())
    }

    fn OnSimpleVolumeChanged(
        &self,
        newvolume: f32,
        newmute: BOOL,
        _eventcontext: *const GUID,
    ) -> WinResult<()> {
        let update =
            UpdateChange::volume_change(Identifier::App(self.pid), newvolume, newmute.as_bool());
        let _ = self.sender.send(update);
        println!(
            "[{}] Volume: {:.0}%, Muted: {}",
            self.name,
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
    ) -> WinResult<()> {
        println!("[{}] Channel volume changed", self.name);
        Ok(())
    }

    fn OnGroupingParamChanged(
        &self,
        _newgroupingparam: *const GUID,
        _eventcontext: *const GUID,
    ) -> WinResult<()> {
        println!("[{}] Grouping param changed", self.name);
        Ok(())
    }

    fn OnStateChanged(&self, newstate: AudioSessionState) -> WinResult<()> {
        println!("OnStateChanges fired: {} | {:?}", self.name, newstate);
        if self.is_expire_state(newstate) {
            self.cleanup_session();

            let update =
                UpdateChange::app_state_change(Identifier::App(self.pid), EntityState::Disconnect);
            let _ = self.sender.send(update);
        }
        Ok(())
    }

    fn OnSessionDisconnected(
        &self,
        disconnectreason: AudioSessionDisconnectReason,
    ) -> WinResult<()> {
        use windows::Win32::Media::Audio::{
            DisconnectReasonDeviceRemoval, DisconnectReasonExclusiveModeOverride,
            DisconnectReasonFormatChanged, DisconnectReasonServerShutdown,
            DisconnectReasonSessionDisconnected, DisconnectReasonSessionLogoff,
        };

        #[allow(nonstandard_style)]
        let reason = match disconnectreason {
            DisconnectReasonDeviceRemoval => "Device removed",
            DisconnectReasonServerShutdown => {
                self.needs_reinit.store(true, Ordering::SeqCst);
                "Service stopped"
            }
            DisconnectReasonFormatChanged => "Format changed",
            DisconnectReasonSessionLogoff => "User logged off",
            DisconnectReasonSessionDisconnected => "RDP disconnected",
            DisconnectReasonExclusiveModeOverride => "Exclusive mode",
            _ => "Unknown",
        };

        self.cleanup_session();

        let update =
            UpdateChange::app_state_change(Identifier::App(self.pid), EntityState::Disconnect);
        let _ = self.sender.send(update);

        println!("[{}] Disconnected: {:?}", self.name, reason);
        Ok(())
    }
}

#[implement(IAudioSessionNotification)]
struct SessionNotification {
    apps: Arc<Mutex<AppMap>>,
    callbacks: Arc<Mutex<RAEvents>>,
    needs_reinit: Arc<AtomicBool>,
    sender: VolumeSender,
}

impl IAudioSessionNotification_Impl for SessionNotification_Impl {
    fn OnSessionCreated(&self, newsession: Ref<IAudioSessionControl>) -> WinResult<()> {
        let session = newsession.ok()?;
        let session2 = session.cast::<IAudioSessionControl2>()?;

        let pid = unsafe { session2.GetProcessId() }?;
        let display_name = super::convert::get_display_name(&session2, pid);

        let mut app_map = match self.apps.lock() {
            Ok(map) => map,
            Err(_) => return Ok(()),
        };

        let entry = app_map.entry(pid).or_insert_with(|| {
            println!("[NEW APP] {} (PID: {})", display_name, pid);
            AudioInfo {
                name: display_name.clone(),
                sessions: vec![],
            }
        });

        entry.sessions.push(session.clone());

        println!(
            " - {} now has {} sessions",
            entry.name,
            entry.sessions.len(),
        );

        let events: IAudioSessionEvents = SessionEvents {
            name: display_name,
            pid: pid,
            session: session.clone(),
            apps: self.apps.clone(),
            needs_reinit: self.needs_reinit.clone(),
            sender: self.sender.clone(),
        }
        .into();

        unsafe { session.RegisterAudioSessionNotification(&events)? };

        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.push((session.clone(), events));
        }

        let _ = self.sender.send(UpdateChange::app_state_change(
            Identifier::App(pid),
            EntityState::Created,
        ));

        Ok(())
    }
}

use super::com_scope::ComManager;

type RDevice = (IAudioEndpointVolume, IAudioEndpointVolumeCallback);

fn register_device(
    manager: &ComManager,
    device_id: &str,
    sender: VolumeSender,
) -> WinResult<RDevice> {
    let dstring = device_id.to_string();
    let callback: IAudioEndpointVolumeCallback = VolumeCallback::new(dstring, sender).into();

    let endpoint_volume: IAudioEndpointVolume = manager.with_generic_device_activate(&device_id)?;
    match unsafe { endpoint_volume.RegisterControlChangeNotify(&callback) } {
        Ok(_) => Ok((endpoint_volume, callback)),
        Err(err) => Err(err),
    }
}

type RSNotice = (IAudioSessionManager2, IAudioSessionNotification);

fn register_session_notification(
    manager: &IAudioSessionManager2,
    apps: Arc<Mutex<AppMap>>,
    callbacks: Arc<Mutex<RAEvents>>,
    needs_reinit: Arc<AtomicBool>,
    sender: VolumeSender,
) -> WinResult<RSNotice> {
    let session_notification: IAudioSessionNotification = SessionNotification {
        apps,
        callbacks,
        needs_reinit,
        sender: sender.clone(),
    }
    .into();

    unsafe { manager.RegisterSessionNotification(&session_notification) }?;

    Ok((manager.clone(), session_notification))
}

type RAEvents = Vec<(IAudioSessionControl, IAudioSessionEvents)>;

fn register_application_notification(
    manager: &IAudioSessionManager2,
    apps: Arc<Mutex<AppMap>>,
    needs_reinit: Arc<AtomicBool>,
    sender: VolumeSender,
) -> WinResult<RAEvents> {
    let mut callbacks = vec![];

    let session_enum = unsafe { manager.GetSessionEnumerator()? };
    let count = unsafe { session_enum.GetCount()? };

    println!("Found {} existing audio sessions", count);
    for i in 0..count {
        let session = unsafe { session_enum.GetSession(i)? };
        let session2: IAudioSessionControl2 = session.cast()?;

        let pid = unsafe { session2.GetProcessId()? };
        let display_name = super::convert::get_display_name(&session2, pid);
        println!(" - [EXISTING] {} (pid: {}):", display_name, pid);

        let events: IAudioSessionEvents = SessionEvents {
            name: display_name.clone(),
            pid: pid,
            apps: apps.clone(),
            session: session.clone(),
            needs_reinit: needs_reinit.clone(),
            sender: sender.clone(),
        }
        .into();

        if let Ok(mut apps) = apps.lock() {
            let entry = apps.entry(pid).or_insert_with(|| AudioInfo {
                name: display_name.clone(),
                sessions: vec![session.clone()],
            });
            entry.sessions.push(session.clone());
        };

        unsafe { session.RegisterAudioSessionNotification(&events)? };
        callbacks.push((session.clone(), events));
    }

    Ok(callbacks)
}

pub struct AudioMonitor {
    device_callback: Vec<RDevice>,
    session_notification: Vec<RSNotice>,
    sessions_application: Arc<Mutex<RAEvents>>,
    apps: Arc<Mutex<AppMap>>,
    needs_reinit: Arc<AtomicBool>,
    sender: VolumeSender,
}

impl AudioMonitor {
    pub fn new(sender: VolumeSender) -> Self {
        Self {
            device_callback: Default::default(),
            session_notification: Default::default(),
            sessions_application: Default::default(),
            apps: Default::default(),
            needs_reinit: Default::default(),
            sender: sender,
        }
    }

    pub fn register_callbacks(&mut self, manager: &ComManager) {
        self.unregister_callbacks();
        self.needs_reinit.store(false, Ordering::SeqCst);

        let mut callbacks_application = vec![];

        println!("\nMonitoring device volume changes...\n");

        for device_id in manager.get_all_device_id().unwrap_or_default() {
            match register_device(&manager, &device_id, self.sender.clone()) {
                Ok(callback) => self.device_callback.push(callback),
                Err(_) => {
                    eprintln!("Error registering device: {}", device_id);
                    continue;
                }
            };

            let session_manager: IAudioSessionManager2 =
                match manager.with_generic_device_activate(&device_id) {
                    Ok(manager) => manager,
                    Err(e) => {
                        eprintln!("Error activating generic device: {}", e);
                        continue;
                    }
                };

            // Register session creation notifications.
            match register_session_notification(
                &session_manager,
                self.apps.clone(),
                self.sessions_application.clone(),
                self.needs_reinit.clone(),
                self.sender.clone(),
            ) {
                Ok(session) => self.session_notification.push(session),
                Err(e) => {
                    eprintln!("Error session notification for device: {}", e);
                    continue;
                }
            };

            // Register existing applications to receive notifications.
            match register_application_notification(
                &session_manager,
                self.apps.clone(),
                self.needs_reinit.clone(),
                self.sender.clone(),
            ) {
                Ok(events) => callbacks_application.extend(events),
                Err(e) => {
                    eprintln!("Error register application notification: {}", e);
                    continue;
                }
            };
        }

        if let Ok(mut sessions_application) = self.sessions_application.lock() {
            sessions_application.extend(callbacks_application);
        }
    }

    pub fn check_and_reinit(&mut self, manager: &ComManager) -> bool {
        let need_reinit = self.needs_reinit.load(Ordering::Relaxed);

        if need_reinit {
            println!("\n!!! Audio service restarted, re-initializing... !!!\n");
            // Small delay to let service stabilize
            std::thread::sleep(std::time::Duration::from_millis(500));
            self.register_callbacks(manager);
        };

        need_reinit
    }

    pub fn unregister_callbacks(&mut self) {
        // Remove while iterating.
        self.device_callback.retain(|(endpoint, sessions)| {
            let _ = unsafe { endpoint.UnregisterControlChangeNotify(sessions) };
            false
        });

        self.session_notification.retain(|(manager, notice)| {
            let _ = unsafe { manager.UnregisterSessionNotification(notice) };
            false
        });

        // Safe to remove: `self.sessions_application` holds the application notifications.
        if let Ok(mut apps) = self.apps.lock() {
            apps.clear();
        }

        match self.sessions_application.lock() {
            Ok(mut sessions_application) => {
                sessions_application.retain(|(control, sessions)| {
                    let _ = unsafe { control.UnregisterAudioSessionNotification(sessions) };
                    false
                });
            }
            Err(e) => {
                eprintln!("Error unregister application notification: {}", e);
            }
        }
    }
}
