export function awaitAbortSignal(signal: AbortSignal) {
    if (signal.aborted) {
        return Promise.resolve();
    }
    return new Promise<void>((resolve) => {
        signal.addEventListener("abort", () => resolve(), { once: true });
    });
}

/**
 * Try to turn any input into a number.
 *
 * - If it can be converted (like "42" or 3.14), return that number.
 * - If it cannot be converted, return undefined.
 * - Also return undefined for NaN, Infinity, and -Infinity.
 */
export function getNumber(value: unknown): number | undefined {
    const number = Number(value);
    // Some reason; Number(null | "" | undefined) = 0
    if (
        value === null ||
        value === undefined ||
        value === "" ||
        !Number.isFinite(number)
    ) {
        return undefined;
    }

    return number;
}

export async function sleep(timeMs: number) {
    return new Promise((resolve) => setTimeout(resolve, timeMs, true));
}

export function centerText(text: string, width: number) {
    const paddingAmount = Math.max(width - text.length, 0);
    const leftPadding = Math.floor(paddingAmount / 2);
    return text.padStart(text.length + leftPadding, " ").padEnd(width, " ");
}

export function try_json<T>(data: string): T {
    try {
        return JSON.parse(data);
    } catch {
        return data as T;
    }
}
