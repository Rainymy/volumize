import type { AppIdentifier, DeviceIdentifier } from "$type/volume";
import {
    debouncedAppVolume,
    debouncedGetAllApplications,
    debouncedGetAppVolume,
    debouncedGetCurrentPlaybackDevice,
    debouncedGetPlaybackDevices,
    debouncedMasterVolume,
    debouncedMuteApp,
    debouncedMuteMaster,
    debouncedsetMasterVolume,
    debouncedUnmuteApp,
    debouncedUnmuteMaster,
} from "./debounced";

export enum RUST_INVOKE {
    SET_DEVICE_VOLUME = "set_device_volume",
    GET_DEVICE_VOLUME = "get_device_volume",
    MUTE_DEVICE = "mute_device",
    UNMUTE_DEVICE = "unmute_device",

    GET_ALL_APPLICATIONS = "get_all_applications",
    GET_APP_VOLUME = "get_app_volume",
    SET_APP_VOLUME = "set_app_volume",
    MUTE_APP_VOLUME = "mute_app_volume",
    UNMUTE_APP_VOLUME = "unmute_app_volume",

    GET_PLAYBACK_DEVICES = "get_playback_devices",
    GET_CURRENT_PLAYBACK_DEVICE = "get_current_playback_device",
}

class TauriVolumeController {
    getMasterVolume = debouncedMasterVolume;
    setMasterVolume = debouncedsetMasterVolume;

    toggleMuteMaster(device_id: DeviceIdentifier, value: boolean) {
        if (value) { return this.unmuteMaster(device_id); }
        return this.muteMaster(device_id);
    }

    private muteMaster = debouncedMuteMaster;
    private unmuteMaster = debouncedUnmuteMaster;

    getAllApplications = debouncedGetAllApplications;
    getAppVolume = debouncedGetAppVolume;
    setAppVolume = debouncedAppVolume;

    toggleMuteApp(app: AppIdentifier, value: boolean) {
        if (value) { return this.unmuteApp(app); }
        return this.muteApp(app);
    }

    private muteApp = debouncedMuteApp;
    private unmuteApp = debouncedUnmuteApp;
    getPlaybackDevices = debouncedGetPlaybackDevices;
    getCurrentPlaybackDevice = debouncedGetCurrentPlaybackDevice;
}

export const volumeController = new TauriVolumeController();
