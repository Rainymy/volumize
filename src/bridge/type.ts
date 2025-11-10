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
    setDeviceVolume(id: DeviceIdentifier, percent: VolumePercent): Promise<unknown>;
    muteDevice(id: DeviceIdentifier): Promise<unknown>;
    unmuteDevice(id: DeviceIdentifier): Promise<unknown>;

    getDeviceApplications(id: DeviceIdentifier): Promise<AppIdentifier[]>;
    findApplicationWithId(id: AppIdentifier): Promise<AudioApplication | null>;
    getApplicationDevice(app: AppIdentifier): Promise<AudioDevice | null>;

    getAppVolume(app: AppIdentifier): Promise<VolumePercent>;
    setAppVolume(app: AppIdentifier, percent: number): Promise<unknown>;
    muteApp(app: AppIdentifier): Promise<unknown>;
    unmuteApp(app: AppIdentifier): Promise<unknown>;

    getPlaybackDevices(): Promise<AudioDevice[]>;
    getCurrentPlaybackDevice(): Promise<AudioDevice | null>;
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
            return (this as unknown as ITauriVolumeController).unmuteApp(app);
        }
        return (this as unknown as ITauriVolumeController).muteApp(app);
    }
}
