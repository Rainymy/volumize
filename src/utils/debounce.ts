type PendingResolvers<TReturn> = Array<(value: Awaited<TReturn>) => void>;

export function debounce<TArgs extends readonly unknown[], TReturn>(
    func: (...args: TArgs) => TReturn | Promise<TReturn>,
    delay: number,
): (...args: TArgs) => Promise<Awaited<TReturn>> {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    let pendingResolvers: PendingResolvers<TReturn> = [];

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

type TKey = string | number;

export function debouncePerKey<TArgs extends readonly unknown[], TReturn>(
    fn: (...args: TArgs) => TReturn | Promise<TReturn>,
    wait: number,
): (key: TKey, ...args: TArgs) => Promise<Awaited<TReturn>> {
    const timers = new Map<TKey, ReturnType<typeof setTimeout>>();
    const pendingResolvers = new Map<TKey, PendingResolvers<TReturn>>();

    return (key: TKey, ...args: TArgs) => {
        const promise = new Promise<Awaited<TReturn>>((resolve) => {
            const arr = pendingResolvers.get(key) ?? [];
            arr.push(resolve);
            pendingResolvers.set(key, arr);
        });

        if (timers.has(key)) {
            clearTimeout(timers.get(key));
        }

        const id = setTimeout(async () => {
            timers.delete(key);
            const resolvers = pendingResolvers.get(key) ?? [];
            pendingResolvers.delete(key);

            const result = await fn(...args);
            for (const resolve of resolvers) {
                resolve(result);
            }
        }, wait);

        timers.set(key, id);
        return promise;
    };
}
