import type {
    AppIdentifier,
    AudioApplication,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";

export interface ITauriVolumeController {
    getAllDevices(): Promise<DeviceIdentifier[]>;

    getDeviceVolume(id: DeviceIdentifier): Promise<VolumePercent>;
    setDeviceVolume(id: DeviceIdentifier, percent: number): Promise<unknown>;
    muteDevice(id: DeviceIdentifier): Promise<unknown>;
    unmuteDevice(id: DeviceIdentifier): Promise<unknown>;

    getApplicationIcon(app: AppIdentifier): Promise<Uint8Array | null>;
    getDeviceApplications(id: DeviceIdentifier): Promise<AppIdentifier[]>;
    getApplication(id: AppIdentifier): Promise<AudioApplication | null>;
    getApplicationDevice(app: AppIdentifier): Promise<AudioDevice | null>;

    getApplicationVolume(app: AppIdentifier): Promise<VolumePercent>;
    setApplicationVolume(app: AppIdentifier, percent: number): Promise<unknown>;
    muteApplication(app: AppIdentifier): Promise<unknown>;
    unmuteApplication(app: AppIdentifier): Promise<unknown>;

    getPlaybackDevices(): Promise<AudioDevice[]>;
    getCurrentPlaybackDevice(): Promise<AudioDevice | null>;
    discoverServer(): Promise<{ url: string; port: number } | null>;
}

export abstract class ATauriVolumeController {
    toggleMuteMaster(id: DeviceIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).unmuteDevice(id);
        }
        return (this as unknown as ITauriVolumeController).muteDevice(id);
    }

    toggleMuteApp(app: AppIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).unmuteApplication(app);
        }
        return (this as unknown as ITauriVolumeController).muteApplication(app);
    }
}
