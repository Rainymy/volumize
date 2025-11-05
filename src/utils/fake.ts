import {
    type AudioApplication,
    type AudioDevice,
    type AudioSession,
    SessionDirection,
    SessionType,
    type VolumePercent,
} from "$type/volume";

export function generate_random_device(): AudioDevice {
    const id = Math.random().toString(36).substring(2, 8);
    const name = `Device ${id}`;
    const friendly_name = `Device ${id}`;
    const volume = {
        current: Math.random() as VolumePercent,
        muted: Math.random() < 0.25,
    };
    const is_default = Math.random() < 0.1;

    return {
        id,
        direction: SessionDirection.Render,
        name,
        friendly_name,
        volume,
        is_default,
    };
}

export function generate_random_application(): AudioApplication {
    const id = Math.floor(Math.random() * 10_000);

    const volume = {
        current: Math.random() as VolumePercent,
        muted: Math.random() < 0.25,
    };

    return {
        process: {
            id,
            name: `Application ${id}`,
            path: null,
        },
        direction: SessionDirection.Render,
        session_type: SessionType.Application,
        sound_playing: Math.random() < 0.75,
        volume,
    };
}

export function generate_random_session(count: number): AudioSession {
    return {
        device: generate_random_device(),
        applications: [
            ...Array.from({ length: count }, () => generate_random_application()),
        ],
    };
}
