import type { AppIdentifier, AudioDevice, AudioSession, DeviceIdentifier, VolumePercent } from "$type/volume";

export abstract class ITauriVolumeController {
    abstract getMasterVolume: (device_id: DeviceIdentifier) => Promise<unknown>;
    abstract setMasterVolume: (device_id: DeviceIdentifier, percent: number) => Promise<unknown>;

    toggleMuteMaster(device_id: DeviceIdentifier, value: boolean) {
        if (value) {
            return this.unmuteMaster(device_id);
        }
        return this.muteMaster(device_id);
    }

    abstract muteMaster: (device_id: DeviceIdentifier) => Promise<unknown>;
    abstract unmuteMaster: (device_id: DeviceIdentifier) => Promise<unknown>;

    abstract getAllApplications: () => Promise<AudioSession[]>;
    abstract getAppVolume: (app: AppIdentifier) => Promise<VolumePercent>;
    abstract setAppVolume: (app: AppIdentifier, percent: number) => Promise<unknown>;

    toggleMuteApp(app: AppIdentifier, value: boolean) {
        if (value) {
            return this.unmuteApp(app);
        }
        return this.muteApp(app);
    }

    abstract muteApp: (app: AppIdentifier) => Promise<unknown>;
    abstract unmuteApp: (app: AppIdentifier) => Promise<unknown>;

    abstract getPlaybackDevices: () => Promise<AudioDevice[]>;
    abstract getCurrentPlaybackDevice: () => Promise<AudioDevice | null>;
}
