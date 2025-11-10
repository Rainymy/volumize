import { invoke } from "@tauri-apps/api/core";
import type {
    AppIdentifier,
    AudioApplication,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce } from "$util/generic";
import { isVolumePercent } from "$util/volume";
import { ATauriVolumeController, type ITauriVolumeController } from "./type";
import { BOUNCE_DELAY, RUST_INVOKE } from "./volumeManager";

export class TauriVolumeController
    extends ATauriVolumeController
    implements ITauriVolumeController
{
    getAllDevices: ITauriVolumeController["getAllDevices"] = debounce(async () => {
        return invoke<DeviceIdentifier[]>(RUST_INVOKE.GET_ALL_DEVICES);
    }, BOUNCE_DELAY.NORMAL);

    getDeviceVolume: ITauriVolumeController["getDeviceVolume"] = debounce(
        (device_id: DeviceIdentifier) => {
            return invoke<VolumePercent>(RUST_INVOKE.GET_DEVICE_VOLUME, {
                deviceId: device_id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    setDeviceVolume: ITauriVolumeController["setDeviceVolume"] = debounce(
        (device_id: DeviceIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }

            return invoke(RUST_INVOKE.SET_DEVICE_VOLUME, {
                deviceId: device_id,
                percent: percent,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteDevice: ITauriVolumeController["muteDevice"] = debounce(
        (device_id: DeviceIdentifier) => {
            return invoke(RUST_INVOKE.MUTE_DEVICE, { deviceId: device_id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    unmuteDevice: ITauriVolumeController["unmuteDevice"] = debounce(
        (device_id: DeviceIdentifier) => {
            return invoke(RUST_INVOKE.UNMUTE_DEVICE, { deviceId: device_id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    /* =================== APPLICATIONS ===================== */

    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] = debounce(
        (device_id: DeviceIdentifier) => {
            return invoke<AppIdentifier[]>(RUST_INVOKE.GET_DEVICE_APPLICATIONS, {
                deviceId: device_id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );
    findApplicationWithId: ITauriVolumeController["findApplicationWithId"] = debounce(
        (id: AppIdentifier) => {
            return invoke<AudioApplication>(RUST_INVOKE.FIND_APPLICATION_WITH_ID, {
                id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );
    getApplicationDevice: ITauriVolumeController["getApplicationDevice"] = debounce(
        (id: AppIdentifier) => {
            return invoke<AudioDevice>(RUST_INVOKE.GET_APPLICATION_DEVICE, {
                id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    getAppVolume: ITauriVolumeController["getAppVolume"] = debounce(
        (app: AppIdentifier) => {
            return invoke<VolumePercent>(RUST_INVOKE.GET_APP_VOLUME, {
                appIdentifier: app,
            });
        },
        BOUNCE_DELAY.FAST,
    );

    setAppVolume: ITauriVolumeController["setAppVolume"] = debounce(
        (app: AppIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }

            return invoke(RUST_INVOKE.SET_APP_VOLUME, {
                appIdentifier: app,
                volume: percent,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteApp: ITauriVolumeController["muteApp"] = debounce((app: AppIdentifier) => {
        return invoke(RUST_INVOKE.MUTE_APP_VOLUME, { appIdentifier: app });
    }, BOUNCE_DELAY.NORMAL);

    unmuteApp: ITauriVolumeController["unmuteApp"] = debounce((app: AppIdentifier) => {
        return invoke(RUST_INVOKE.UNMUTE_APP_VOLUME, {
            appIdentifier: app,
        });
    }, BOUNCE_DELAY.NORMAL);

    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(() => {
        return invoke<AudioDevice[]>(RUST_INVOKE.GET_PLAYBACK_DEVICES);
    }, BOUNCE_DELAY.FAST);
    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debounce(() => {
            return invoke<AudioDevice | null>(RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE);
        }, BOUNCE_DELAY.FAST);
}
