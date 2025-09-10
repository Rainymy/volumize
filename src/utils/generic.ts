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

export function debounce<T extends (...args: never[]) => unknown>(
    func: T,
    delay: number,
): (...args: Parameters<T>) => Promise<ReturnType<T>> {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    let pendingResolvers: Array<(value: ReturnType<T>) => void> = [];

    return (...args: Parameters<T>): Promise<ReturnType<T>> => {
        if (timeoutId !== null) {
            clearTimeout(timeoutId);
        }

        return new Promise<ReturnType<T>>((resolve) => {
            pendingResolvers.push(resolve);

            timeoutId = setTimeout(async () => {
                const result = await func(...args) as ReturnType<T>;

                for (const resolver of pendingResolvers) { resolver(result); }
                pendingResolvers = [];
                timeoutId = null;
            }, delay);
        });
    };
}

export function getNumber(num: unknown): number | undefined {
    const number = Number(num);
    return Number.isFinite(number) ? number : undefined;
}

export async function sleep(timeMs: number) {
    return new Promise((resolve, _reject) => {
        setTimeout(resolve, timeMs, true);
    });
}

export function centerText(text: string, width: number) {
    const paddingAmount = Math.max(width - text.length, 0);
    const leftPadding = Math.floor(paddingAmount / 2);
    return text.padStart(text.length + leftPadding, " ").padEnd(width, " ");
}
