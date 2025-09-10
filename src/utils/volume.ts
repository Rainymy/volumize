import type { VolumePercent } from "$type/volume";

/**
 * @param value - Is between 0-100% volume.
 * @returns
 */
export function isVolumePercent(value: number): value is VolumePercent {
    if (typeof value !== "number") {
        return false;
    }

    return 0 <= value && value <= 1;
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
