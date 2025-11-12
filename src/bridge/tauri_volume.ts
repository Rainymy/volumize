import { invoke } from "@tauri-apps/api/core";
import type {
    AppIdentifier,
    AudioApplication,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
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
        (id: DeviceIdentifier) => {
            return invoke<VolumePercent>(RUST_INVOKE.GET_DEVICE_VOLUME, { id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    setDeviceVolume: ITauriVolumeController["setDeviceVolume"] = debounce(
        (id: DeviceIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }

            return invoke(RUST_INVOKE.SET_DEVICE_VOLUME, { id, percent });
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteDevice: ITauriVolumeController["muteDevice"] = debounce(
        (id: DeviceIdentifier) => {
            return invoke(RUST_INVOKE.MUTE_DEVICE, { id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    unmuteDevice: ITauriVolumeController["unmuteDevice"] = debounce(
        (id: DeviceIdentifier) => {
            return invoke(RUST_INVOKE.UNMUTE_DEVICE, { id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    /* =================== APPLICATIONS ===================== */

    getApplicationIcon: ITauriVolumeController["getApplicationIcon"] = debouncePerKey(
        (id: AppIdentifier) => {
            return invoke<Uint8Array | null>(RUST_INVOKE.GET_APPLICATION_ICON, {
                id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );
    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] = debounce(
        (id: DeviceIdentifier) => {
            return invoke<AppIdentifier[]>(RUST_INVOKE.GET_DEVICE_APPLICATIONS, {
                id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );
    findApplicationWithId: ITauriVolumeController["findApplicationWithId"] =
        debouncePerKey((id: AppIdentifier) => {
            return invoke<AudioApplication>(RUST_INVOKE.FIND_APPLICATION_WITH_ID, {
                id,
            });
        }, BOUNCE_DELAY.FAST);
    getApplicationDevice: ITauriVolumeController["getApplicationDevice"] = debounce(
        (id: AppIdentifier) => {
            return invoke<AudioDevice>(RUST_INVOKE.GET_APPLICATION_DEVICE, {
                id,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    getAppVolume: ITauriVolumeController["getAppVolume"] = debounce(
        (id: AppIdentifier) => {
            return invoke<VolumePercent>(RUST_INVOKE.GET_APP_VOLUME, { id });
        },
        BOUNCE_DELAY.NORMAL,
    );

    setAppVolume: ITauriVolumeController["setAppVolume"] = debounce(
        (id: AppIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }

            return invoke(RUST_INVOKE.SET_APP_VOLUME, {
                id,
                volume: percent,
            });
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteApp: ITauriVolumeController["muteApp"] = debounce((id: AppIdentifier) => {
        return invoke(RUST_INVOKE.MUTE_APP_VOLUME, { id });
    }, BOUNCE_DELAY.NORMAL);

    unmuteApp: ITauriVolumeController["unmuteApp"] = debounce((id: AppIdentifier) => {
        return invoke(RUST_INVOKE.UNMUTE_APP_VOLUME, { id });
    }, BOUNCE_DELAY.NORMAL);

    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(() => {
        return invoke<AudioDevice[]>(RUST_INVOKE.GET_PLAYBACK_DEVICES);
    }, BOUNCE_DELAY.NORMAL);

    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debounce(() => {
            return invoke<AudioDevice | null>(RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE);
        }, BOUNCE_DELAY.NORMAL);

    discoverServer: ITauriVolumeController["discoverServer"] = debounce(() => {
        return { url: "127.0.0.1", port: 9002 };
    }, BOUNCE_DELAY.NORMAL);
}
