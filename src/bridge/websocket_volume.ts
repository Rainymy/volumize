import type {
    AppIdentifier,
    AudioDevice,
    AudioSession,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce } from "$util/generic";
import { ITauriVolumeController } from "./type";
import { BOUNCE_DELAY, RUST_INVOKE } from "./volumeManager";
import { ConnectSocket } from "./websocket";

export type T_RUST_INVOKE = `${RUST_INVOKE}`;

export class WebsocketTauriVolumeController extends ITauriVolumeController {
    private eventListenerHandler: EventTarget = new EventTarget();
    private event_name = "main_channel";
    private connection: ConnectSocket = new ConnectSocket();

    async setup() {
        await this.connection.retryUntilConnection();

        this.eventListenerHandler.addEventListener(
            this.event_name,
            (event: Event & CustomEventInit) => {
                const detail = event.detail as {
                    channel: string;
                    data: string;
                };

                this.connection.socket?.addEventListener(
                    "message",
                    (event) => {
                        console.log("socket: ", event);
                        const custom_event = new CustomEvent(detail.channel, {
                            detail: event.data,
                        });
                        this.eventListenerHandler.dispatchEvent(custom_event);
                    },
                );

                this.connection.socket?.send(detail.data);
            },
        );
        return this;
    }

    private sendEvent<T>(action: T_RUST_INVOKE, data?: string) {
        return new Promise<T>((resolve) => {
            this.eventListenerHandler.addEventListener(
                action,
                (event: Event & CustomEventInit) => {
                    console.log("[send event]: ", event.detail);
                    resolve(event.detail as T);
                },
                { once: true },
            );

            const custom_event = new CustomEvent(this.event_name, {
                detail: { channel: action, data: data },
            });
            this.eventListenerHandler.dispatchEvent(custom_event);
        });
    }

    private parse_params(
        action: T_RUST_INVOKE,
        data?: {
            device_id?: string;
            app?: AppIdentifier;
            percent?: VolumePercent;
        },
    ) {
        const params = [];
        if (data?.app) {
            params.push(data?.app);
        }
        if (data?.device_id) {
            params.push(data.device_id);
        }
        if (data?.percent) {
            params.push(data?.percent);
        }

        if (!params.length) {
            return action;
        }

        return JSON.stringify({ [action]: params });
    }

    getMasterVolume = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.GET_DEVICE_VOLUME);
        return await this.sendEvent(RUST_INVOKE.GET_DEVICE_VOLUME, data);
    }, BOUNCE_DELAY.FAST);

    setMasterVolume = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.SET_DEVICE_VOLUME);
        return await this.sendEvent(RUST_INVOKE.SET_DEVICE_VOLUME, data);
    }, BOUNCE_DELAY.FAST);

    muteMaster = debounce(async (device_id: DeviceIdentifier) => {
        const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { device_id });
        return await this.sendEvent(RUST_INVOKE.MUTE_DEVICE, data);
    }, BOUNCE_DELAY.FAST);

    unmuteMaster = debounce(async (device_id: DeviceIdentifier) => {
        const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { device_id });
        return await this.sendEvent(RUST_INVOKE.UNMUTE_DEVICE, data);
    }, BOUNCE_DELAY.FAST);

    getAllApplications = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.GET_ALL_APPLICATIONS);
        return await this.sendEvent<AudioSession[]>(
            RUST_INVOKE.GET_ALL_APPLICATIONS,
            data,
        );
    }, BOUNCE_DELAY.FAST);

    getAppVolume = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.GET_APP_VOLUME);
        return await this.sendEvent<VolumePercent>(
            RUST_INVOKE.GET_APP_VOLUME,
            data,
        );
    }, BOUNCE_DELAY.FAST);

    setAppVolume = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.SET_APP_VOLUME);
        return await this.sendEvent(RUST_INVOKE.SET_APP_VOLUME, data);
    }, BOUNCE_DELAY.FAST);

    muteApp = debounce(async (app: AppIdentifier) => {
        const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
        return await this.sendEvent<VolumePercent>(
            RUST_INVOKE.MUTE_APP_VOLUME,
            data,
        );
    }, BOUNCE_DELAY.FAST);

    unmuteApp = debounce(async (app: AppIdentifier) => {
        const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
        return await this.sendEvent<VolumePercent>(
            RUST_INVOKE.UNMUTE_APP_VOLUME,
            data,
        );
    }, BOUNCE_DELAY.FAST);

    getPlaybackDevices = debounce(async () => {
        return await this.sendEvent<AudioDevice[]>(
            RUST_INVOKE.GET_PLAYBACK_DEVICES,
        );
    }, BOUNCE_DELAY.FAST);

    getCurrentPlaybackDevice = debounce(async () => {
        return await this.sendEvent<AudioDevice | null>(
            RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
        );
    }, BOUNCE_DELAY.FAST);
}
