import { invoke } from "@tauri-apps/api/core";
import { DEBOUNCE_DELAY, UPDATE_CENTER_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import { isDataEvent, isUpdateEvent } from "$type/update";
import type {
    AppIdentifier,
    AudioApplication,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
import { tryParseURL } from "$util/temp";
import { uuid } from "$util/uuid";
import { isVolumePercent } from "$util/volume";
import {
    ATauriVolumeController,
    type ITauriVolumeController,
    type PARAM_ACTION,
    RUST_INVOKE,
    type T_RUST_INVOKE,
} from "./type";
import { ConnectSocket } from "./websocket";

type SEND_ACTION = {
    action: T_RUST_INVOKE;
    request_id: string;
    data: string;
};

export class WebsocketTauriVolumeController
    extends ATauriVolumeController
    implements ITauriVolumeController
{
    private listener = new EventTarget();
    private connection: ConnectSocket = new ConnectSocket();

    async setup(url: string, port: number) {
        this.connection.set_url(url, port);
        await this.connection.connect();
        console.log("We have a connection!");

        this.connection.addListener((event) => {
            if (event.type === "Pong") {
                return;
            }
            const data = this.connection.parse_data(event);
            if (data === null) {
                console.log("Encountered parse error: ", event);
                return;
            }

            if (isDataEvent(data)) {
                const data2 = { channel: data.type, data: data.data };
                const payload = { detail: data2.data };
                this.listener.dispatchEvent(new CustomEvent(data2.channel, payload));
            }

            if (isUpdateEvent(data)) {
                const payload = { detail: data.payload };
                this.listener.dispatchEvent(
                    new CustomEvent(UPDATE_CENTER_EVENT, payload),
                );
                return;
            }
        });
        return this;
    }

    async close() {
        await this.connection?.close();
    }

    async heartbeat() {
        return await this.connection.heartbeat();
    }

    private async sendEvent<T>(
        action: SEND_ACTION,
        timeoutMs: number = 2_500,
    ): Promise<T | null> {
        let listener: ((event: EventType<T>) => void) | null = null;

        const cleanup = () => {
            if (listener) {
                this.listener.removeEventListener(action.request_id, listener);
                listener = null;
            }
        };

        const timer = new Promise<never>((_, reject) => {
            setTimeout(() => {
                cleanup();
                reject(`Event '${action.action}' timed out after ${timeoutMs}ms`);
            }, timeoutMs);
        });

        const waitFor = new Promise<T | null>((resolve) => {
            listener = (event: EventType<T>) => {
                cleanup();
                resolve(event.detail ?? null);
            };

            this.listener.addEventListener(action.request_id, listener);
        });

        await this.connection.send(action.data);

        try {
            return await Promise.race([waitFor, timer]);
        } catch (error) {
            console.warn(`[ ${this.sendEvent.name} / ${action.action} ]:`, error);
            return null;
        }
    }

    private parse_params(action: T_RUST_INVOKE, data?: PARAM_ACTION): SEND_ACTION {
        const unique_id = uuid();

        return {
            action: action,
            request_id: unique_id,
            data: JSON.stringify({
                [action]: { ...(data ?? {}), request_id: unique_id },
            }),
        };
    }

    /* ============== DEVICES ============== */
    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debouncePerKey(
        async () => {
            const invoke_action = RUST_INVOKE.GET_PLAYBACK_DEVICES;
            const data = this.parse_params(invoke_action);
            const devices = await this.sendEvent<AudioDevice[]>(data);
            return devices ?? [];
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceGetVolume: ITauriVolumeController["deviceGetVolume"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_GET_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            const volume = await this.sendEvent<VolumePercent>(data);
            return volume ?? (0.0 as VolumePercent);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceSetVolume: ITauriVolumeController["deviceSetVolume"] = debounce(
        async (id: DeviceIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }
            const invoke_action = RUST_INVOKE.DEVICE_SET_VOLUME;
            const data = this.parse_params(invoke_action, { id, volume });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceMute: ITauriVolumeController["deviceMute"] = debouncePerKey(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_MUTE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceUnmute: ITauriVolumeController["deviceUnmute"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.DEVICE_UNMUTE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    /* ============== APPLICATIONS ============== */
    getApplication: ITauriVolumeController["getApplication"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APPLICATION;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<AudioApplication>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationGetIcon: ITauriVolumeController["applicationGetIcon"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_GET_ICON;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<Uint8Array | null>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationGetVolume: ITauriVolumeController["applicationGetVolume"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_GET_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            const volume = await this.sendEvent<VolumePercent>(data);
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
            const data = this.parse_params(invoke_action, { id, volume });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationMute: ITauriVolumeController["applicationMute"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_MUTE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<VolumePercent>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationUnmute: ITauriVolumeController["applicationUnmute"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.APPLICATION_UNMUTE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<VolumePercent>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] =
        debouncePerKey(async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_DEVICE_APPLICATIONS;
            const data = this.parse_params(invoke_action, { id });
            const applications_ids = await this.sendEvent<AppIdentifier[]>(data);
            return applications_ids ?? [];
        }, DEBOUNCE_DELAY.NORMAL);

    discoverServer: ITauriVolumeController["discoverServer"] = debounce(async () => {
        const invoke_action = RUST_INVOKE.DISCOVER_SERVER_ADDRESS;
        return tryParseURL(await invoke<string | null>(invoke_action));
    }, DEBOUNCE_DELAY.NORMAL);
}
