/** biome-ignore-all lint/suspicious/noDuplicateEnumValues: temp solution */
import type { AudioApplication, AudioDevice, Identifier } from "$type/bindings";
import type { TauriConnection, WebConnection } from "$type/navigation";
import type { AppIdentifier, DeviceIdentifier, VolumePercent } from "$type/volume";

export enum RUST_INVOKE {
    DISCOVER_SERVER_ADDRESS = "discover_server_address",
    HEARTBEAT = "heartbeat",

    // ============= DEVICE =============
    DEVICE_GET_VOLUME = "get_volume",
    DEVICE_SET_VOLUME = "set_volume",
    DEVICE_MUTE = "set_mute",
    DEVICE_UNMUTE = "set_mute",

    // =========== APPLICATION ===========
    GET_APPLICATION = "get_application",

    APPLICATION_GET_ICON = "get_icon",
    APPLICATION_GET_VOLUME = `get_volume`,
    APPLICATION_SET_VOLUME = "set_volume",
    APPLICATION_MUTE = "set_mute",
    APPLICATION_UNMUTE = "set_mute",

    // ============= MANAGER =============
    GET_PLAYBACK_DEVICES = "get_playback_devices",
    GET_DEVICE_APPLICATIONS = "get_applications",
}

export type T_RUST_INVOKE = `${RUST_INVOKE}`;
export type PARAM_ACTION = {
    id: Identifier;
    volume?: VolumePercent;
};

export interface ITauriVolumeController {
    discoverServer(): Promise<TauriConnection | WebConnection | null>;
    heartbeat(): Promise<boolean>;

    setup(url: string, port: number): Promise<void>;
    close(): Promise<void>;

    // ============= DEVICE =============
    deviceGetVolume(id: DeviceIdentifier): Promise<VolumePercent>;
    deviceSetVolume(id: DeviceIdentifier, percent: number): Promise<unknown>;
    deviceMute(id: DeviceIdentifier): Promise<unknown>;
    deviceUnmute(id: DeviceIdentifier): Promise<unknown>;

    // =========== APPLICATION ===========
    getApplication(id: AppIdentifier): Promise<AudioApplication | null>;
    applicationGetIcon(app: AppIdentifier): Promise<Uint8Array | null>;

    applicationGetVolume(app: AppIdentifier): Promise<VolumePercent>;
    applicationSetVolume(app: AppIdentifier, percent: number): Promise<unknown>;
    applicationMute(app: AppIdentifier): Promise<unknown>;
    applicationUnmute(app: AppIdentifier): Promise<unknown>;

    // ============= MANAGER =============
    getPlaybackDevices(): Promise<AudioDevice[]>;
    getDeviceApplications(id: DeviceIdentifier): Promise<AppIdentifier[]>;
}

export abstract class ATauriVolumeController {
    toggleMuteMaster(id: DeviceIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).deviceUnmute(id);
        }
        return (this as unknown as ITauriVolumeController).deviceMute(id);
    }

    toggleMuteApp(app: AppIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).applicationUnmute(app);
        }
        return (this as unknown as ITauriVolumeController).applicationMute(app);
    }
}
