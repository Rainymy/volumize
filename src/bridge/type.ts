import type { AppIdentifier, AudioDevice, AudioSession, DeviceIdentifier, VolumePercent } from "$type/volume";

export interface ITauriVolumeController {
    getMasterVolume(device_id: DeviceIdentifier): Promise<VolumePercent>;
    setMasterVolume(device_id: DeviceIdentifier, percent: number): Promise<unknown>;

    muteMaster(device_id: DeviceIdentifier): Promise<unknown>;
    unmuteMaster(device_id: DeviceIdentifier): Promise<unknown>;

    getAllApplications(): Promise<AudioSession[]>;
    getAppVolume(app: AppIdentifier): Promise<VolumePercent>;
    setAppVolume(app: AppIdentifier, percent: number): Promise<unknown>;

    muteApp(app: AppIdentifier): Promise<unknown>;
    unmuteApp(app: AppIdentifier): Promise<unknown>;

    getPlaybackDevices(): Promise<AudioDevice[]>;
    getCurrentPlaybackDevice(): Promise<AudioDevice | null>;
}

export abstract class ATauriVolumeController {
    toggleMuteMaster(device_id: DeviceIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).unmuteMaster(device_id);
        }
        return (this as unknown as ITauriVolumeController).muteMaster(device_id);
    }

    toggleMuteApp(app: AppIdentifier, value: boolean) {
        if (value) {
            return (this as unknown as ITauriVolumeController).unmuteApp(app);
        }
        return (this as unknown as ITauriVolumeController).muteApp(app);
    }
}
