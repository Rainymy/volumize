import type { VolumePercent } from "./volumeType";

export function getNumber(num: unknown) {
    const number = Number(num);
    return Number.isFinite(number) ? number : undefined;
}

export async function sleep(timeMs: number) {
    return new Promise((resolve, _reject) => {
        setTimeout(resolve, timeMs, true);
    });
}

export function isEnumValue<T extends string>(
    enumObject: Record<string, T>,
    value: unknown,
): value is T {
    return Object.values(enumObject).includes(value as T);
}

export function getIncludes<T extends readonly unknown[]>(
    enumArray: T,
    value: unknown,
): T[number] | null {
    if (!enumArray.includes(value as T[number])) {
        return null;
    }
    return value as T[number];
}

export function getEnumIncludes<E extends Record<string, string>>(
    enumObject: E,
    value: unknown,
): E[keyof E] | null {
    if (!isEnumValue(enumObject, value as E[keyof E])) {
        return null;
    }
    return value as E[keyof E];
}

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

export function centerText(text: string, width: number) {
    const paddingAmount = Math.max(width - text.length, 0);
    const leftPadding = Math.floor(paddingAmount / 2);
    return text.padStart(text.length + leftPadding, " ").padEnd(width, " ");
}
