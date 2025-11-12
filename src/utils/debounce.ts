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
type PendingResolvers2<T> = {
    resolve: (value: Awaited<T>) => void;
    reject: (reason: unknown) => void;
};

export function debouncePerKey<TArgs extends readonly unknown[], TReturn>(
    fn: (...args: TArgs) => TReturn | Promise<TReturn>,
    wait: number,
): (...args: TArgs) => Promise<Awaited<TReturn>> {
    const timers = new Map<TKey, ReturnType<typeof setTimeout>>();
    const pendingResolvers = new Map<TKey, PendingResolvers2<TReturn>[]>();

    return (...args: TArgs) => {
        const key: TKey = args[0] as TKey;

        const promise = new Promise<Awaited<TReturn>>((p_resolve, p_reject) => {
            const arr = pendingResolvers.get(key) ?? ([] as PendingResolvers2<TReturn>[]);
            arr.push({ resolve: p_resolve, reject: p_reject });
            pendingResolvers.set(key, arr);
        });

        if (timers.has(key)) {
            clearTimeout(timers.get(key));
        }

        const id = setTimeout(async () => {
            timers.delete(key);
            const resolvers = pendingResolvers.get(key) ?? [];
            pendingResolvers.delete(key);

            try {
                const result = await fn(...args);
                for (const { resolve } of resolvers) {
                    resolve(result);
                }
            } catch (error) {
                for (const { reject } of resolvers) {
                    reject(error);
                }
            }
        }, wait);

        timers.set(key, id);
        return promise;
    };
}
