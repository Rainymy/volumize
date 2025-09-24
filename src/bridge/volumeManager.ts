import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

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

// In milliseconds
export enum BOUNCE_DELAY {
    NORMAL = 100,
    SLOW = 200,
    FAST = 70,
    SUPER_FAST = 50,
}

export const volumeController = !is_desktop()
    ? new TauriVolumeController()
    : new WebsocketTauriVolumeController();