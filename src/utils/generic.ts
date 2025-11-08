export function debounceSync<T extends (...args: never[]) => unknown>(
    func: T,
    delay: number,
): (...args: Parameters<T>) => void {
    let timeoutId: number | null = null;

    return (...args: Parameters<T>) => {
        if (timeoutId !== null) {
            clearTimeout(timeoutId);
        }

        timeoutId = setTimeout(func, delay, ...args);
    };
}

export function debounce<TArgs extends readonly unknown[], TReturn>(
    func: (...args: TArgs) => TReturn | Promise<TReturn>,
    delay: number,
): (...args: TArgs) => Promise<Awaited<TReturn>> {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    let pendingResolvers: Array<(value: Awaited<TReturn>) => void> = [];

    return (...args: TArgs): Promise<Awaited<TReturn>> => {
        if (timeoutId !== null) {
            clearTimeout(timeoutId);
        }

        return new Promise<Awaited<TReturn>>((resolve) => {
            pendingResolvers.push(resolve);

            timeoutId = setTimeout(async () => {
                const result = await func(...args);

                for (const resolver of pendingResolvers) {
                    resolver(result);
                }
                pendingResolvers = [];
                timeoutId = null;
            }, delay);
        });
    };
}

/**
 * Try to turn any input into a number.
 *
 * - If it can be converted (like "42" or 3.14), return that number.
 * - If it cannot be converted, return undefined.
 * - Also return undefined for NaN, Infinity, and -Infinity.
 */
export function getNumber(num: unknown): number | undefined {
    // Some reason; Number(null) = 0
    if (num === null) {
        return undefined;
    }

    const number = Number(num);
    return Number.isFinite(number) ? number : undefined;
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
