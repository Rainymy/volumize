import { invoke } from "@tauri-apps/api/core";
import type {
    AppIdentifier,
    AudioDevice,
    AudioSession,
    VolumePercent,
} from "$type/volume";
import { debounce } from "$util/generic";
import { isVolumePercent } from "$util/volume";
import { RUST_INVOKE } from "./volumeManager";

// In milliseconds
enum BOUNCE_DELAY {
    NORMAL = 100,
    SLOW = 200,
    FAST = 70,
    SUPER_FAST = 50
}

export const debouncedAppVolume = debounce(
    (app: AppIdentifier, percent: number) => {
        if (!isVolumePercent(percent)) {
            throw Error(`Invalid VolumePercent value: ${percent}`);
        }

        return invoke(RUST_INVOKE.SET_APP_VOLUME, {
            appIdentifier: app,
            volume: percent,
        });
    },
    BOUNCE_DELAY.NORMAL,
);

export const debouncedsetMasterVolume = debounce((percent: number) => {
    if (!isVolumePercent(percent)) {
        throw Error(`Invalid VolumePercent value: ${percent}`);
    }

    return invoke(RUST_INVOKE.SET_MASTER_VOLUME, { percent: percent });
}, BOUNCE_DELAY.NORMAL);


export const debouncedMasterVolume = debounce(() => {
    return invoke(RUST_INVOKE.GET_MASTER_VOLUME);
}, BOUNCE_DELAY.NORMAL);

export const debouncedMuteMaster = debounce(() => {
    return invoke(RUST_INVOKE.MUTE_MASTER);
}, BOUNCE_DELAY.NORMAL);

export const debouncedUnmuteMaster = debounce(() => {
    return invoke(RUST_INVOKE.UNMUTE_MASTER);
}, BOUNCE_DELAY.NORMAL);

export const debouncedGetAllApplications = debounce(() => {
    return invoke<AudioSession[]>(RUST_INVOKE.GET_ALL_APPLICATIONS);
}, BOUNCE_DELAY.FAST);

export const debouncedGetAppVolume = debounce((app: AppIdentifier) => {
    return invoke<VolumePercent>(RUST_INVOKE.GET_APP_VOLUME, {
        appIdentifier: app,
    });
}, BOUNCE_DELAY.FAST);

export const debouncedMuteApp = debounce((app: AppIdentifier) => {
    return invoke(RUST_INVOKE.MUTE_APP_VOLUME, { appIdentifier: app });
}, BOUNCE_DELAY.NORMAL);

export const debouncedUnmuteApp = debounce((app: AppIdentifier) => {
    return invoke(RUST_INVOKE.UNMUTE_APP_VOLUME, { appIdentifier: app });
}, BOUNCE_DELAY.NORMAL);

export const debouncedGetPlaybackDevices = debounce(() => {
    return invoke<AudioDevice[]>(RUST_INVOKE.GET_PLAYBACK_DEVICES);
}, BOUNCE_DELAY.FAST);

export const debouncedGetCurrentPlaybackDevice = debounce(() => {
    return invoke<AudioDevice | null>(RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE);
}, BOUNCE_DELAY.FAST);
