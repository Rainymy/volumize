import type {
    AppIdentifier,
    AudioDevice,
    AudioSession,
    DeviceIdentifier,
    VolumePercent,
} from "$type/volume";
import { debounce, try_json } from "$util/generic";
import { isVolumePercent } from "$util/volume";
import { ATauriVolumeController, type ITauriVolumeController } from "./type";
import { BOUNCE_DELAY, RUST_INVOKE } from "./volumeManager";
import { ConnectSocket } from "./websocket";

export type T_RUST_INVOKE = `${RUST_INVOKE}`;

export class WebsocketTauriVolumeController
    extends ATauriVolumeController
    implements ITauriVolumeController {
    private eventListenerHandler = new EventTarget();
    private event_name = "main_channel";

    async setup() {
        const connection = await new ConnectSocket().retryUntilConnection();

        this.eventListenerHandler.addEventListener(
            this.event_name,
            (event: Event & CustomEventInit) => {
                const detail = event.detail as {
                    channel: string;
                    data: string;
                };

                connection.addEventListener(
                    "message",
                    (event) => {
                        const custom_event = new CustomEvent(detail.channel, {
                            detail: event.data,
                        });
                        this.eventListenerHandler.dispatchEvent(custom_event);
                    },
                    { once: true },
                );

                connection.send(detail.data);
            },
        );
        return this;
    }

    private sendEvent<T>(action: T_RUST_INVOKE, data?: string) {
        return new Promise<T>((resolve) => {
            this.eventListenerHandler.addEventListener(
                action,
                (event: Event & CustomEventInit) => {
                    // console.log("[send event]: ", event.detail);
                    resolve(try_json<T>(event.detail));
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
        const { app, device_id, percent } = data ?? {};
        const params = [];

        if (app) params.push(app);
        if (device_id) params.push(device_id);
        if (percent) params.push(percent);

        return JSON.stringify(params.length ? { [action]: params } : action);
    }

    getMasterVolume: ITauriVolumeController["getMasterVolume"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_DEVICE_VOLUME, {
                device_id,
            });
            return await this.sendEvent(RUST_INVOKE.GET_DEVICE_VOLUME, data);
        },
        BOUNCE_DELAY.FAST,
    );

    setMasterVolume: ITauriVolumeController["setMasterVolume"] = debounce(
        async (device_id: DeviceIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }
            const data = this.parse_params(RUST_INVOKE.SET_DEVICE_VOLUME, {
                device_id,
                percent,
            });
            return await this.sendEvent(RUST_INVOKE.SET_DEVICE_VOLUME, data);
        },
        BOUNCE_DELAY.FAST,
    );

    muteMaster: ITauriVolumeController["muteMaster"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            return await this.sendEvent(RUST_INVOKE.MUTE_DEVICE, data);
        },
        BOUNCE_DELAY.FAST,
    );

    unmuteMaster: ITauriVolumeController["unmuteMaster"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            return await this.sendEvent(RUST_INVOKE.UNMUTE_DEVICE, data);
        },
        BOUNCE_DELAY.FAST,
    );

    getAllApplications: ITauriVolumeController["getAllApplications"] = debounce(
        async () => {
            const data = this.parse_params(RUST_INVOKE.GET_ALL_APPLICATIONS);
            return await this.sendEvent<AudioSession[]>(
                RUST_INVOKE.GET_ALL_APPLICATIONS,
                data,
            );
        },
        BOUNCE_DELAY.FAST,
    );

    getAppVolume: ITauriVolumeController["getAppVolume"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_APP_VOLUME, { app });
            return await this.sendEvent<VolumePercent>(
                RUST_INVOKE.GET_APP_VOLUME,
                data,
            );
        },
        BOUNCE_DELAY.FAST,
    );

    setAppVolume: ITauriVolumeController["setAppVolume"] = debounce(
        async (app: AppIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }
            const data = this.parse_params(RUST_INVOKE.SET_APP_VOLUME, {
                app,
                percent,
            });
            return await this.sendEvent(RUST_INVOKE.SET_APP_VOLUME, data);
        },
        BOUNCE_DELAY.FAST,
    );

    muteApp: ITauriVolumeController["muteApp"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
            return await this.sendEvent<VolumePercent>(
                RUST_INVOKE.MUTE_APP_VOLUME,
                data,
            );
        },
        BOUNCE_DELAY.FAST,
    );

    unmuteApp: ITauriVolumeController["unmuteApp"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
            return await this.sendEvent<VolumePercent>(
                RUST_INVOKE.UNMUTE_APP_VOLUME,
                data,
            );
        },
        BOUNCE_DELAY.FAST,
    );

    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(
        async () => {
            return await this.sendEvent<AudioDevice[]>(
                RUST_INVOKE.GET_PLAYBACK_DEVICES,
            );
        },
        BOUNCE_DELAY.FAST,
    );

    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debounce(async () => {
            return await this.sendEvent<AudioDevice | null>(
                RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
            );
        }, BOUNCE_DELAY.FAST);
}
