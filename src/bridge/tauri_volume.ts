import { invoke } from "@tauri-apps/api/core";
import { DEBOUNCE_DELAY } from "$type/constant";
import type {
    AppIdentifier,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
import { isVolumePercent } from "$util/volume";
import {
    ATauriVolumeController,
    type ITauriVolumeController,
    type PARAM_ACTION,
    RUST_INVOKE,
    type T_RUST_INVOKE,
} from "./type";

export class TauriVolumeController
    extends ATauriVolumeController
    implements ITauriVolumeController
{
    private async sendEvent<T>(
        action: T_RUST_INVOKE,
        data?: PARAM_ACTION,
    ): Promise<T | null> {
        try {
            return await invoke<T>(action, data);
        } catch (error) {
            console.error(`Error sending event ${action}:`, error);
            return null;
        }
    }

    /* ===================== DEVICES ===================== */
    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(
        async () => {
            const invoke_action = RUST_INVOKE.GET_PLAYBACK_DEVICES;
            const playback_devices = await this.sendEvent<AudioDevice[]>(invoke_action);

            return playback_devices ?? [];
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceGetVolume: ITauriVolumeController["deviceGetVolume"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_GET_VOLUME;
            const params: PARAM_ACTION = { id };

            const device_volume = await this.sendEvent<VolumePercent>(
                invoke_action,
                params,
            );
            return device_volume ?? (0.0 as VolumePercent);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceSetVolume: ITauriVolumeController["deviceSetVolume"] = debounce(
        async (id: DeviceIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }

            const invoke_action = RUST_INVOKE.DEVICE_SET_VOLUME;
            const params: PARAM_ACTION = { id, volume };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceMute: ITauriVolumeController["deviceMute"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_MUTE;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceUnmute: ITauriVolumeController["deviceUnmute"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_UNMUTE;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    /* =================== APPLICATIONS ===================== */
    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_DEVICE_APPLICATIONS;
            const params: PARAM_ACTION = { id };

            return (await this.sendEvent(invoke_action, params)) ?? [];
        },
        DEBOUNCE_DELAY.NORMAL,
    );
    applicationGetIcon: ITauriVolumeController["applicationGetIcon"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_GET_ICON;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );
    getApplication: ITauriVolumeController["getApplication"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APPLICATION;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.FAST,
    );

    applicationGetVolume: ITauriVolumeController["applicationGetVolume"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_GET_VOLUME;
            const params: PARAM_ACTION = { id };
            const volume = await this.sendEvent<VolumePercent>(invoke_action, params);

            return volume ?? (0.0 as VolumePercent);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationSetVolume: ITauriVolumeController["applicationSetVolume"] = debounce(
        async (id: AppIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }

            const invoke_action = RUST_INVOKE.APPLICATION_SET_VOLUME;
            const params: PARAM_ACTION = { id, volume };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationMute: ITauriVolumeController["applicationMute"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_MUTE;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationUnmute: ITauriVolumeController["applicationUnmute"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_UNMUTE;
            const params: PARAM_ACTION = { id };

            return await this.sendEvent(invoke_action, params);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    discoverServer() {
        console.warn("You are using Tauri Volume Controller.");
        return new Promise<null>((resolve) => resolve(null));
    }
}
