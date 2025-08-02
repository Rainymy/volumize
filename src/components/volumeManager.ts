import { invoke } from "@tauri-apps/api/core";

import { AppIdentifier, AudioDevice, AudioSession, VolumeController, VolumePercent } from "../utils/volumeType";

class TauriVolumeController extends VolumeController {
    async getMasterVolume(): Promise<VolumePercent | null> {
        return await invoke<VolumePercent | null>("get_master_volume");
    }

    async setMasterVolume(percent: VolumePercent): Promise<void> {
        return await invoke("set_master_volume", { percent: percent });
    }

    async muteMaster(): Promise<void> {
        return await invoke("mute_master");
    }

    async unmuteMaster(): Promise<void> {
        return await invoke("unmute_master");
    }

    async getAllApplications(): Promise<AudioSession[]> {
        return await invoke("get_all_applications");
    }

    async getAppVolume(app: AppIdentifier): Promise<VolumePercent> {
        return await invoke<VolumePercent>("get_app_volume", { appIdentifier: app });
    }

    async setAppVolume(app: AppIdentifier, percent: VolumePercent): Promise<void> {
        return await invoke("set_app_volume", { appIdentifier: app, volume: percent });
    }

    async muteApp(app: AppIdentifier): Promise<void> {
        return await invoke("mute_app_volume", { appIdentifier: app });
    }

    async unmuteApp(app: AppIdentifier): Promise<void> {
        return await invoke("unmute_app_volume", { appIdentifier: app });
    }

    async getPlaybackDevices(): Promise<AudioDevice[]> {
        return await invoke("get_playback_devices");
    }

    async getCurrentPlaybackDevice(): Promise<AudioDevice | null> {
        return await invoke("get_current_playback_device");
    }
}

export const volumeController = new TauriVolumeController();