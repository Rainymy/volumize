import { invoke } from "@tauri-apps/api/core";
import type {
    AppIdentifier,
    AudioApplication,
    AudioDevice,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
import { getNumber } from "$util/generic";
import { isVolumePercent } from "$util/volume";
import { ATauriVolumeController, type ITauriVolumeController } from "./type";
import { BOUNCE_DELAY, RUST_INVOKE } from "./volumeManager";
import { ConnectSocket } from "./websocket";

export type T_RUST_INVOKE = `${RUST_INVOKE}`;
type EventType = Event & CustomEventInit;

type PARAM_ACTION = {
    id: DeviceIdentifier | AppIdentifier;
    volume?: VolumePercent;
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
            const data = this.connection.parse_data(event);
            if (data === null) {
                console.log("Encountered parse error: ", event);
                return;
            }

            const custom_event = new CustomEvent(data.channel, {
                detail: data.data,
            });
            this.listener.dispatchEvent(custom_event);
        });
        return this;
    }

    async close() {
        await this.connection?.close();
    }

    private async sendEvent<T>(
        action: T_RUST_INVOKE,
        data: string,
        timeoutMs: number = 2_500,
    ): Promise<T | null> {
        let listener: ((event: EventType) => void) | null = null;

        const cleanup = () => {
            if (listener) {
                this.listener.removeEventListener(action, listener);
                listener = null;
            }
        };

        const timer = new Promise<never>((_, reject) => {
            setTimeout(() => {
                cleanup();
                reject(new Error(`Event '${action}' timed out after ${timeoutMs}ms`));
            }, timeoutMs);
        });

        const waitFor = new Promise<T>((resolve) => {
            listener = (event: EventType) => {
                cleanup();
                resolve(event.detail as T);
            };

            this.listener.addEventListener(action, listener);
        });

        await this.connection.send(data);

        try {
            return await Promise.race([waitFor, timer]);
        } catch (error) {
            console.log(`[ ${this.sendEvent.name}/${action} ]:`, error);
            return null;
        }
    }

    private parse_params(action: T_RUST_INVOKE, data?: PARAM_ACTION) {
        return JSON.stringify({ [action]: data ?? {} });
    }

    /* ============== DEVICES ============== */
    getAllDevices: ITauriVolumeController["getAllDevices"] = debouncePerKey(async () => {
        const invoke_action = RUST_INVOKE.GET_ALL_DEVICES;
        const data = this.parse_params(invoke_action);
        const devices = await this.sendEvent<DeviceIdentifier[]>(invoke_action, data);
        return devices ?? [];
    }, BOUNCE_DELAY.NORMAL);

    getDeviceVolume: ITauriVolumeController["getDeviceVolume"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_DEVICE_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            const volume = await this.sendEvent<VolumePercent>(invoke_action, data);
            return volume ?? (0.0 as VolumePercent);
        },
        BOUNCE_DELAY.NORMAL,
    );

    setDeviceVolume: ITauriVolumeController["setDeviceVolume"] = debounce(
        async (id: DeviceIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }
            const invoke_action = RUST_INVOKE.SET_DEVICE_VOLUME;
            const data = this.parse_params(invoke_action, { id, volume });
            return await this.sendEvent(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteDevice: ITauriVolumeController["muteDevice"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.MUTE_DEVICE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    unmuteDevice: ITauriVolumeController["unmuteDevice"] = debounce(
        async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.UNMUTE_DEVICE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    /* ============== DEVICES ============== */
    getApplicationIcon: ITauriVolumeController["getApplicationIcon"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APPLICATION_ICON;
            const data = this.parse_params(invoke_action, { id });

            return await this.sendEvent<Uint8Array | null>(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] =
        debouncePerKey(async (id: DeviceIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_DEVICE_APPLICATIONS;
            const data = this.parse_params(invoke_action, { id });
            const applications_ids = await this.sendEvent<AppIdentifier[]>(
                invoke_action,
                data,
            );
            return applications_ids ?? [];
        }, BOUNCE_DELAY.NORMAL);

    getApplication: ITauriVolumeController["getApplication"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APPLICATION;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<AudioApplication>(invoke_action, data);
        },
        BOUNCE_DELAY.SLOW,
    );

    getApplicationDevice: ITauriVolumeController["getApplicationDevice"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APPLICATION_DEVICE;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<AudioDevice>(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    getApplicationVolume: ITauriVolumeController["getApplicationVolume"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.GET_APP_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            const volume = await this.sendEvent<VolumePercent>(invoke_action, data);
            return volume ?? (0.0 as VolumePercent);
        },
        BOUNCE_DELAY.NORMAL,
    );

    setApplicationVolume: ITauriVolumeController["setApplicationVolume"] = debounce(
        async (id: AppIdentifier, volume: number) => {
            if (!isVolumePercent(volume)) {
                throw Error(`Invalid VolumePercent value: ${volume}`);
            }
            const invoke_action = RUST_INVOKE.SET_APP_VOLUME;
            const data = this.parse_params(invoke_action, { id, volume });
            return await this.sendEvent(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteApplication: ITauriVolumeController["muteApplication"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.MUTE_APP_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<VolumePercent>(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    unmuteApplication: ITauriVolumeController["unmuteApplication"] = debounce(
        async (id: AppIdentifier) => {
            const invoke_action = RUST_INVOKE.UNMUTE_APP_VOLUME;
            const data = this.parse_params(invoke_action, { id });
            return await this.sendEvent<VolumePercent>(invoke_action, data);
        },
        BOUNCE_DELAY.NORMAL,
    );

    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debouncePerKey(
        async () => {
            const invoke_action = RUST_INVOKE.GET_PLAYBACK_DEVICES;
            const data = this.parse_params(invoke_action);
            const devices = await this.sendEvent<AudioDevice[]>(invoke_action, data);
            return devices ?? [];
        },
        BOUNCE_DELAY.NORMAL,
    );

    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debouncePerKey(async () => {
            const invoke_action = RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE;
            const data = this.parse_params(invoke_action);
            return await this.sendEvent<AudioDevice | null>(invoke_action, data);
        }, BOUNCE_DELAY.NORMAL);

    discoverServer: ITauriVolumeController["discoverServer"] = debounce(async () => {
        const invoke_action = RUST_INVOKE.DISCOVER_SERVER_ADDRESS;
        const address = await invoke<string | null>(invoke_action);
        return tryParseURL(address);
    }, BOUNCE_DELAY.NORMAL);
}

/**
 * TEMP FIX:
 *  - find a way to handle parse IP or URL address without protocol.
 *
 * This function is implemented with:
 * ```js
 *  const url = new URL(`http://${urlString}`);
 * ```
 */
function tryParseURL(urlString: string | null) {
    if (urlString === null || urlString.length === 0) {
        return null;
    }
    try {
        const url = new URL(`http://${urlString}`);
        const port = getNumber(url.port);
        if (port === undefined) {
            return null;
        }
        return { url: url.hostname, port: port };
    } catch (error) {
        console.log(`[ ${tryParseURL.name} ]: `, urlString, error);
        return null;
    }
}
