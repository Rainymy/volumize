import { invoke } from "@tauri-apps/api/core";

import type {
    AppIdentifier,
    AudioDevice,
    AudioSession,
    VolumePercent,
} from "$type/volume";

enum RUST_INVOKE {
    SET_MASTER_VOLUME = "set_master_volume",
    GET_MASTER_VOLUME = "get_master_volume",
    MUTE_MASTER = "mute_master",
    UNMUTE_MASTER = "unmute_master",
    GET_ALL_APPLICATIONS = "get_all_applications",
    GET_APP_VOLUME = "get_app_volume",
    SET_APP_VOLUME = "set_app_volume",
    MUTE_APP_VOLUME = "mute_app_volume",
    UNMUTE_APP_VOLUME = "unmute_app_volume",
    GET_PLAYBACK_DEVICES = "get_playback_devices",
    GET_CURRENT_PLAYBACK_DEVICE = "get_current_playback_device",
}

class TauriVolumeController {
    getMasterVolume() {
        return invoke<VolumePercent | null>(RUST_INVOKE.GET_MASTER_VOLUME);
    }

    setMasterVolume(percent: VolumePercent) {
        return invoke(RUST_INVOKE.SET_MASTER_VOLUME, { percent: percent });
    }

    muteMaster() {
        return invoke(RUST_INVOKE.MUTE_MASTER);
    }

    unmuteMaster() {
        return invoke(RUST_INVOKE.UNMUTE_MASTER);
    }

    getAllApplications() {
        return invoke<AudioSession[]>(RUST_INVOKE.GET_ALL_APPLICATIONS);
    }

    getAppVolume(app: AppIdentifier) {
        return invoke<VolumePercent>(RUST_INVOKE.GET_APP_VOLUME, {
            appIdentifier: app,
        });
    }

    setAppVolume(app: AppIdentifier, percent: VolumePercent) {
        return invoke(RUST_INVOKE.SET_APP_VOLUME, {
            appIdentifier: app,
            volume: percent,
        });
    }

    muteApp(app: AppIdentifier) {
        return invoke(RUST_INVOKE.MUTE_APP_VOLUME, { appIdentifier: app });
    }

    unmuteApp(app: AppIdentifier) {
        return invoke(RUST_INVOKE.UNMUTE_APP_VOLUME, { appIdentifier: app });
    }

    getPlaybackDevices() {
        return invoke<AudioDevice[]>(RUST_INVOKE.GET_PLAYBACK_DEVICES);
    }

    getCurrentPlaybackDevice() {
        return invoke<AudioDevice | null>(
            RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
        );
    }
}

export const volumeController = new TauriVolumeController();
