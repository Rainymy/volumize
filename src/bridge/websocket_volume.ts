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
    private connection: ConnectSocket = new ConnectSocket();

    async setup(url: string, port: number) {
        this.connection.setup(url, port);
        await this.connection.connect();

        this.connection.addListener((event) => {
            const data = this.connection.parse_data(event);
            if (data === null) {
                return;
            }

            const custom_event = new CustomEvent(data.type, {
                detail: data.data,
            });
            this.eventListenerHandler.dispatchEvent(custom_event);
        });
        return this;
    }

    close() {
        this.connection?.close();
    }

    private sendEvent<T>(
        action: T_RUST_INVOKE,
        data: string,
        timeoutMs: number = 5_000,
    ) {
        return new Promise<T>((resolve, reject) => {
            let eventListener:
                | ((event: Event & CustomEventInit) => void)
                | null = null;
            let timeoutId: NodeJS.Timeout | null = null;

            const cleanup = () => {
                if (eventListener) {
                    this.eventListenerHandler.removeEventListener(
                        action,
                        eventListener,
                    );
                    eventListener = null;
                }
                if (timeoutId) {
                    clearTimeout(timeoutId);
                    timeoutId = null;
                }
            };

            timeoutId = setTimeout(() => {
                cleanup();
                reject(
                    new Error(
                        `Event '${action}' timed out after ${timeoutMs}ms`,
                    ),
                );
            }, timeoutMs);

            eventListener = (event: Event & CustomEventInit) => {
                cleanup();
                // console.log("[send event]: ", event.detail);
                resolve(try_json<T>(event.detail));
            };

            this.eventListenerHandler.addEventListener(action, eventListener, {
                once: true,
            });

            const custom_event = new CustomEvent(this.event_name, {
                detail: { channel: action, data: data },
            });
            this.eventListenerHandler.dispatchEvent(custom_event);

            this.connection.send(data);
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
            try {
                return await this.sendEvent(
                    RUST_INVOKE.GET_DEVICE_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getMasterVolume.name}]: `, error);
                return 0.0 as VolumePercent;
            }
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
            try {
                return await this.sendEvent(
                    RUST_INVOKE.SET_DEVICE_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.setMasterVolume.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    muteMaster: ITauriVolumeController["muteMaster"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            try {
                return await this.sendEvent(RUST_INVOKE.MUTE_DEVICE, data);
            } catch (error) {
                console.log(`[${this.muteMaster.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    unmuteMaster: ITauriVolumeController["unmuteMaster"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            try {
                return await this.sendEvent(RUST_INVOKE.UNMUTE_DEVICE, data);
            } catch (error) {
                console.log(`[${this.unmuteMaster.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    getAllApplications: ITauriVolumeController["getAllApplications"] = debounce(
        async () => {
            const data = this.parse_params(RUST_INVOKE.GET_ALL_APPLICATIONS);
            try {
                return await this.sendEvent<AudioSession[]>(
                    RUST_INVOKE.GET_ALL_APPLICATIONS,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getAllApplications.name}]: `, error);
                return [];
            }
        },
        BOUNCE_DELAY.FAST,
    );

    getAppVolume: ITauriVolumeController["getAppVolume"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_APP_VOLUME, { app });
            try {
                return await this.sendEvent<VolumePercent>(
                    RUST_INVOKE.GET_APP_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getAppVolume.name}]: `, error);
                return 0.0 as VolumePercent;
            }
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
            try {
                return await this.sendEvent(RUST_INVOKE.SET_APP_VOLUME, data);
            } catch (error) {
                console.log(`[${this.setAppVolume.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    muteApp: ITauriVolumeController["muteApp"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
            try {
                return await this.sendEvent<VolumePercent>(
                    RUST_INVOKE.MUTE_APP_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.muteApp.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    unmuteApp: ITauriVolumeController["unmuteApp"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
            try {
                return await this.sendEvent<VolumePercent>(
                    RUST_INVOKE.UNMUTE_APP_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.unmuteApp.name}]: `, error);
            }
        },
        BOUNCE_DELAY.FAST,
    );

    getPlaybackDevices: ITauriVolumeController["getPlaybackDevices"] = debounce(
        async () => {
            const data = this.parse_params(RUST_INVOKE.GET_PLAYBACK_DEVICES);
            try {
                return await this.sendEvent<AudioDevice[]>(
                    RUST_INVOKE.GET_PLAYBACK_DEVICES,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getPlaybackDevices.name}]: `, error);
                return [];
            }
        },
        BOUNCE_DELAY.FAST,
    );

    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debounce(async () => {
            const data = this.parse_params(
                RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
            );
            try {
                return await this.sendEvent<AudioDevice | null>(
                    RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getCurrentPlaybackDevice.name}]: `, error);
                return null;
            }
        }, BOUNCE_DELAY.FAST);
}
