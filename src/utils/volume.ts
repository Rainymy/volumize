import type { VolumePercent } from "$type/volume";

export function isVolumePercent(value: number): value is VolumePercent {
    return value >= 0 && value <= 100;
}

/**
 * @param value - Is between 0-100% volume.
 * @returns
 */
export const createVolumePercent = (value: number): VolumePercent => {
    if (!isVolumePercent(value)) {
        throw new Error(`Invalid volume: ${value}. Must be between 0 and 1.`);
    }

    return value;
};
