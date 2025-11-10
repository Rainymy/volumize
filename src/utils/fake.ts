import { type AudioDevice, SessionDirection, type VolumePercent } from "$type/volume";

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
