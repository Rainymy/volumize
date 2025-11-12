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

export class WebsocketTauriVolumeController
    extends ATauriVolumeController
    implements ITauriVolumeController
{
    private eventListenerHandler = new EventTarget();
    private connection: ConnectSocket = new ConnectSocket();

    async setup(url: string, port: number) {
        this.connection.set_url(url, port);
        await this.connection.connect();
        console.log("We have a connection!");

        this.connection.addListener((event) => {
            const data = this.connection.parse_data(event);
            if (data === null) {
                return;
            }

            const custom_event = new CustomEvent(data.channel, {
                detail: data.data,
            });
            this.eventListenerHandler.dispatchEvent(custom_event);
        });
        return this;
    }

    async close() {
        await this.connection?.close();
    }

    private async sendEvent<T>(
        action: T_RUST_INVOKE,
        data: string,
        timeoutMs: number = 5_000,
    ): Promise<T> {
        let eventListener: ((event: EventType) => void) | null = null;

        const cleanup = () => {
            if (eventListener) {
                this.eventListenerHandler.removeEventListener(action, eventListener);
                eventListener = null;
            }
        };

        const timer = new Promise<never>((_, reject) => {
            setTimeout(() => {
                cleanup();
                reject(new Error(`Event '${action}' timed out after ${timeoutMs}ms`));
            }, timeoutMs);
        });

        const waitFor = new Promise<T>((resolve) => {
            eventListener = (event: EventType) => {
                cleanup();
                console.log("[send event]: ", event.detail);
                resolve(event.detail as T);
            };

            this.eventListenerHandler.addEventListener(action, eventListener);
        });

        await this.connection.send(data);

        return await Promise.race([waitFor, timer]);
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

    /* ============== DEVICES ============== */
    getAllDevices: ITauriVolumeController["getAllDevices"] = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.GET_ALL_DEVICES);
        try {
            return await this.sendEvent<DeviceIdentifier[]>(
                RUST_INVOKE.GET_ALL_DEVICES,
                data,
            );
        } catch (error) {
            console.log(`[${this.getAllDevices.name}]: `, error);
            return [];
        }
    }, BOUNCE_DELAY.NORMAL);

    getDeviceVolume: ITauriVolumeController["getDeviceVolume"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_DEVICE_VOLUME, {
                device_id,
            });
            try {
                return await this.sendEvent<VolumePercent>(
                    RUST_INVOKE.GET_DEVICE_VOLUME,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getDeviceVolume.name}]: `, error);
                return 0.0 as VolumePercent;
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    setDeviceVolume: ITauriVolumeController["setDeviceVolume"] = debounce(
        async (device_id: DeviceIdentifier, percent: number) => {
            if (!isVolumePercent(percent)) {
                throw Error(`Invalid VolumePercent value: ${percent}`);
            }
            const data = this.parse_params(RUST_INVOKE.SET_DEVICE_VOLUME, {
                device_id,
                percent,
            });
            try {
                return await this.sendEvent(RUST_INVOKE.SET_DEVICE_VOLUME, data);
            } catch (error) {
                console.log(`[${this.setDeviceVolume.name}]: `, error);
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    muteDevice: ITauriVolumeController["muteDevice"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            try {
                return await this.sendEvent(RUST_INVOKE.MUTE_DEVICE, data);
            } catch (error) {
                console.log(`[${this.muteDevice.name}]: `, error);
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    unmuteDevice: ITauriVolumeController["unmuteDevice"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, {
                device_id,
            });
            try {
                return await this.sendEvent(RUST_INVOKE.UNMUTE_DEVICE, data);
            } catch (error) {
                console.log(`[${this.unmuteDevice.name}]: `, error);
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    /* ============== DEVICES ============== */
    getApplicationIcon: ITauriVolumeController["getApplicationIcon"] = debouncePerKey(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_APPLICATION_ICON, {
                app,
            });
            try {
                return await this.sendEvent<Uint8Array | null>(
                    RUST_INVOKE.GET_APPLICATION_ICON,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getApplicationIcon.name}]: `, error);
                return null;
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    getDeviceApplications: ITauriVolumeController["getDeviceApplications"] = debounce(
        async (device_id: DeviceIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_DEVICE_APPLICATIONS, {
                device_id,
            });
            try {
                return await this.sendEvent<AppIdentifier[]>(
                    RUST_INVOKE.GET_DEVICE_APPLICATIONS,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getDeviceApplications.name}]: `, error);
                return [];
            }
        },
        BOUNCE_DELAY.NORMAL,
    );

    findApplicationWithId: ITauriVolumeController["findApplicationWithId"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.FIND_APPLICATION_WITH_ID, {
                app,
            });
            try {
                return await this.sendEvent<AudioApplication>(
                    RUST_INVOKE.FIND_APPLICATION_WITH_ID,
                    data,
                );
            } catch (error) {
                console.log(`[${this.findApplicationWithId.name}]: `, error);
                return null;
            }
        },
        BOUNCE_DELAY.SLOW,
    );

    getApplicationDevice: ITauriVolumeController["getApplicationDevice"] = debounce(
        async (app: AppIdentifier) => {
            const data = this.parse_params(RUST_INVOKE.GET_APPLICATION_DEVICE, {
                app,
            });
            try {
                return await this.sendEvent<AudioDevice>(
                    RUST_INVOKE.GET_APPLICATION_DEVICE,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getApplicationDevice.name}]: `, error);
                return null;
            }
        },
        BOUNCE_DELAY.NORMAL,
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
        BOUNCE_DELAY.NORMAL,
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
        BOUNCE_DELAY.NORMAL,
    );

    muteApp: ITauriVolumeController["muteApp"] = debounce(async (app: AppIdentifier) => {
        const data = this.parse_params(RUST_INVOKE.MUTE_DEVICE, { app });
        try {
            return await this.sendEvent<VolumePercent>(RUST_INVOKE.MUTE_APP_VOLUME, data);
        } catch (error) {
            console.log(`[${this.muteApp.name}]: `, error);
        }
    }, BOUNCE_DELAY.NORMAL);

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
        BOUNCE_DELAY.NORMAL,
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
        BOUNCE_DELAY.NORMAL,
    );

    getCurrentPlaybackDevice: ITauriVolumeController["getCurrentPlaybackDevice"] =
        debounce(async () => {
            const data = this.parse_params(RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE);
            try {
                return await this.sendEvent<AudioDevice | null>(
                    RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
                    data,
                );
            } catch (error) {
                console.log(`[${this.getCurrentPlaybackDevice.name}]: `, error);
                return null;
            }
        }, BOUNCE_DELAY.NORMAL);

    discoverServer: ITauriVolumeController["discoverServer"] = debounce(async () => {
        const data = this.parse_params(RUST_INVOKE.DISCOVER_SERVER_ADDRESS);
        try {
            const address = await this.sendEvent<string>(
                RUST_INVOKE.DISCOVER_SERVER_ADDRESS,
                data,
            );
            // TEMP FIX: find a way to handle parse IP or URL address without protocol.
            const url = new URL(`https://${address}`);
            const port = getNumber(url.port);

            if (port === undefined) {
                return null;
            }

            return { url: url.hostname, port: port };
        } catch (error) {
            console.log(`[${this.discoverServer.name}]: `, error);
            return null;
        }
    }, BOUNCE_DELAY.NORMAL);
}
