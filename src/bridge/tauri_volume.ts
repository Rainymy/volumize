import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { commands, UPDATE_EVENT_NAME, VOLUME_LABEL_EVENT } from "$type/bindings";
import { DEBOUNCE_DELAY, UPDATE_CENTER_EVENT } from "$type/constant";
import type { TauriConnection } from "$type/navigation";
import type { UpdateEvent } from "$type/update";
import type { AppIdentifier, DeviceIdentifier, VolumePercent } from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
import { isVolumePercent } from "$util/volume";
import { ATauriVolumeController, type ITauriVolumeController } from "./type";

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
            UPDATE_EVENT_NAME,
            (event) => {
                console.log("event.id:", event.id);
                const data = new CustomEvent(UPDATE_CENTER_EVENT, { detail: event });
                document.body.dispatchEvent(data);
            },
            { target: { kind: "AnyLabel", label: VOLUME_LABEL_EVENT } },
        );
    }

    async heartbeat() {
        // Only need to check if listener is already set.
        // By checking if there is unlisten function.
        return typeof this.listener === "function";
    }

    /* ===================== DEVICES ===================== */
    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(
        async () => {
            const result = await commands.getPlaybackDevices();
            if (result.status === "error") {
                console.log("Error getting playback devices:", result.error);
                return [];
            }
            return result.data;
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceGetVolume: ITauriVolumeController["deviceGetVolume"] = debounce(
        async (id: DeviceIdentifier) => {
            const result = await commands.getVolume({ type: "device", content: id });

            if (result.status === "error") {
                console.log("Error getting volume:", result.error);
                return 0.0 as VolumePercent;
            }

            return (result.data ?? 0.0) as VolumePercent;
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceSetVolume: ITauriVolumeController["deviceSetVolume"] = debounce(
        async (id: DeviceIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }

            const result = await commands.setVolume(
                { type: "device", content: id },
                volume,
            );

            if (result.status === "error") {
                console.log("Error setting volume:", result.error);
                return;
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceMute: ITauriVolumeController["deviceMute"] = debounce(
        async (id: DeviceIdentifier) => {
            const result = await commands.setMute({ type: "device", content: id });

            if (result.status === "error") {
                console.log("Error muting device:", result.error);
                return;
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceUnmute: ITauriVolumeController["deviceUnmute"] = debounce(
        async (id: DeviceIdentifier) => {
            const result = await commands.setUnmute({ type: "device", content: id });

            if (result.status === "error") {
                console.log("Error unmuting device:", result.error);
                return;
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    /* =================== APPLICATIONS ===================== */
    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] = debounce(
        async (id: DeviceIdentifier) => {
            const result = await commands.getDeviceApplications(id);

            if (result.status === "error") {
                console.log("Error getting device applications:", result.error);
                return [];
            }

            return result.data;
        },
        DEBOUNCE_DELAY.NORMAL,
    );
    applicationGetIcon: ITauriVolumeController["applicationGetIcon"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const result = await commands.getIcon({ type: "app", content: id });

            if (result.status === "error") {
                console.log("Error getting application icon:", result.error);
                return null;
            }

            return new Uint8Array(result.data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );
    getApplication: ITauriVolumeController["getApplication"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const result = await commands.getApplication(id);

            if (result.status === "error") {
                console.log("Error getting application:", result.error);
                return null;
            }

            return result.data;
        },
        DEBOUNCE_DELAY.FAST,
    );

    applicationGetVolume: ITauriVolumeController["applicationGetVolume"] = debounce(
        async (id: AppIdentifier) => {
            const result = await commands.getVolume({ type: "app", content: id });

            if (result.status === "error") {
                console.log("Error getting volume:", result.error);
                return 0.0 as VolumePercent;
            }

            return (result.data ?? 0.0) as VolumePercent;
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationSetVolume: ITauriVolumeController["applicationSetVolume"] = debounce(
        async (id: AppIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }

            const result = await commands.setVolume({ type: "app", content: id }, volume);

            if (result.status === "error") {
                console.log("Error setting volume:", result.error);
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationMute: ITauriVolumeController["applicationMute"] = debounce(
        async (id: AppIdentifier) => {
            const result = await commands.setMute({ type: "app", content: id });

            if (result.status === "error") {
                console.log("Error muting:", result.error);
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationUnmute: ITauriVolumeController["applicationUnmute"] = debounce(
        async (id: AppIdentifier) => {
            const result = await commands.setUnmute({ type: "app", content: id });

            if (result.status === "error") {
                console.log("Error unmuting:", result.error);
            }
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    async discoverServer() {
        return await new Promise<TauriConnection>((resolve) => {
            return resolve({ kind: "tauri", url: "", port: 0 });
        });
    }
}
