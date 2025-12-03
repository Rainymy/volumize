import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { DEBOUNCE_DELAY, TAURI_UPDATE_EVENT, UPDATE_CENTER_EVENT } from "$type/constant";
import type { TauriConnection } from "$type/navigation";
import type { UpdateEvent } from "$type/update";
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
    private listener: UnlistenFn | null = null;

    async close() {
        this.listener?.();
        this.listener = null;
    }

    async setup(_url: string, _port: number) {
        await this.close();
        this.listener = await listen<UpdateEvent>(
            TAURI_UPDATE_EVENT,
            (event) => {
                console.log("event.id:", event.id);
                const CENTRAL = UPDATE_CENTER_EVENT;
                const data = new CustomEvent(CENTRAL, { detail: event });
                document.body.dispatchEvent(data);
            },
            { target: { kind: "AnyLabel", label: "volume-control-panel" } },
        );
    }

    async heartbeat() {
        // Only need to check if listener is already set.
        // By checking if there is unlisten function.
        return typeof this.listener === "function";
    }

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

    async discoverServer() {
        return await new Promise<TauriConnection>((resolve) => {
            return resolve({ kind: "tauri", url: "", port: 0 });
        });
    }
}
