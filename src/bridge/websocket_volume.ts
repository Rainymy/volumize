import {
    type AudioApplication,
    type AudioDevice,
    type Command,
    type CommandRequest,
    commands,
    type Identifier,
} from "$type/bindings";
import { DEBOUNCE_DELAY, UPDATE_CENTER_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import type { WebConnection } from "$type/navigation";
import {
    isDataEvent,
    isRequestAcceptedEvent,
    isResponse,
    isUpdateEvent,
} from "$type/update";
import type { AppIdentifier, DeviceIdentifier, VolumePercent } from "$type/volume";
import { debounce, debouncePerKey } from "$util/debounce";
import { tryParseURL } from "$util/temp";
import { random_u32 } from "$util/uuid";
import { isVolumePercent } from "$util/volume";
import {
    ATauriVolumeController,
    type ITauriVolumeController,
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
                console.warn("Encountered parse error: ", event);
                return;
            }

            if (!isResponse(data)) {
                console.warn("Is Not correct response: ", data);
                return;
            }

            const response = data.response;

            if (isRequestAcceptedEvent(response)) {
                // console.log("Accepted Response: ", response);
                const channel = data.id.toString();
                this.listener.dispatchEvent(new CustomEvent(channel));
                return;
            }

            if (isDataEvent(response)) {
                // TODO: Validate incoming id against cached ids.
                const channel = data.id.toString();
                const payload = { detail: response.data };
                this.listener.dispatchEvent(new CustomEvent(channel, payload));
                return;
            }

            console.log("Update Event: ", data);
            if (isUpdateEvent(response)) {
                const payload = { detail: data };
                const __evt__ = new CustomEvent(UPDATE_CENTER_EVENT, payload);
                document.body.dispatchEvent(__evt__);
                return;
            }

            console.log("Received unknown event:", data);
        });
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
        const waitFor = new Promise<T | null>((resolve, reject) => {
            const listener = (event: EventType<T>) => {
                this.listener.removeEventListener(action.request_id, listener);
                resolve(event.detail ?? null);
            };
            this.listener.addEventListener(action.request_id, listener);

            setTimeout(() => {
                this.listener.removeEventListener(action.request_id, listener);
                reject(`Event '${action.action}' timed out after ${timeoutMs}ms`);
            }, timeoutMs);
        });

        const did_send = await this.connection.send(action.data);
        if (!did_send) {
            return null;
        }

        try {
            return await waitFor;
        } catch (error) {
            console.warn(`[ ${this.sendEvent.name} / ${action.action} ]:`, error);
        }

        return null;
    }

    private parse_params(param: Command): SEND_ACTION {
        const unique_id = random_u32();
        // This is the actual data to send
        const request: CommandRequest = {
            id: unique_id,
            command: param,
        };
        return {
            action: param.type, // This is for debugging purposes only
            request_id: unique_id.toString(), // Channel id
            data: JSON.stringify({ type: "CommandRequest", ...request }),
        };
    }

    private id_device(id: DeviceIdentifier): Identifier {
        return { type: "device", content: id };
    }
    private id_app(id: AppIdentifier): Identifier {
        return { type: "app", content: id };
    }

    /* ============== DEVICES ============== */
    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debouncePerKey(
        async () => {
            const data = this.parse_params({ type: "get_playback_devices" });
            const devices = await this.sendEvent<AudioDevice[]>(data);
            return devices ?? [];
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceGetVolume: ITauriVolumeController["deviceGetVolume"] = debounce(
        async (id: DeviceIdentifier) => {
            const data = this.parse_params({
                type: "get_volume",
                data: { id: this.id_device(id) },
            });
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
            const data = this.parse_params({
                type: "set_volume",
                data: { id: this.id_device(id), volume },
            });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceMute: ITauriVolumeController["deviceMute"] = debouncePerKey(
        async (id: DeviceIdentifier) => {
            const data = this.parse_params({
                type: "set_mute",
                data: { id: this.id_device(id), mute: true },
            });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    deviceUnmute: ITauriVolumeController["deviceUnmute"] = debounce(
        async (id: DeviceIdentifier) => {
            const data = this.parse_params({
                type: "set_mute",
                data: { id: this.id_device(id), mute: false },
            });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    /* ============== APPLICATIONS ============== */
    getApplication: ITauriVolumeController["getApplication"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const data = this.parse_params({
                type: "get_application",
                data: { id },
            });
            return await this.sendEvent<AudioApplication>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationGetIcon: ITauriVolumeController["applicationGetIcon"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const data = this.parse_params({
                type: "get_icon",
                data: { id: this.id_app(id) },
            });
            return await this.sendEvent<Uint8Array | null>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationGetVolume: ITauriVolumeController["applicationGetVolume"] = debounce(
        async (id: AppIdentifier) => {
            const data = this.parse_params({
                type: "get_volume",
                data: { id: this.id_app(id) },
            });
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
            const data = this.parse_params({
                type: "set_volume",
                data: { id: this.id_app(id), volume },
            });
            return await this.sendEvent(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationMute: ITauriVolumeController["applicationMute"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const data = this.parse_params({
                type: "set_mute",
                data: { id: this.id_app(id), mute: true },
            });
            return await this.sendEvent<VolumePercent>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    applicationUnmute: ITauriVolumeController["applicationUnmute"] = debouncePerKey(
        async (id: AppIdentifier) => {
            const data = this.parse_params({
                type: "set_mute",
                data: { id: this.id_app(id), mute: false },
            });
            return await this.sendEvent<VolumePercent>(data);
        },
        DEBOUNCE_DELAY.NORMAL,
    );

    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] =
        debouncePerKey(async (id: DeviceIdentifier) => {
            const data = this.parse_params({
                type: "get_applications",
                data: { id },
            });

            type Response_Device = {
                id: string;
                apps: AppIdentifier[];
            };
            const applications_ids = await this.sendEvent<Response_Device>(data);
            return applications_ids?.apps ?? [];
        }, DEBOUNCE_DELAY.NORMAL);

    discoverServer: ITauriVolumeController["discoverServer"] = debounce(async () => {
        const parsedURL = tryParseURL(await commands.discoverServerAddress());
        if (!parsedURL) {
            return null;
        }
        return {
            kind: "web",
            port: parsedURL.port,
            url: parsedURL.url,
        } satisfies WebConnection;
    }, DEBOUNCE_DELAY.NORMAL);
}
