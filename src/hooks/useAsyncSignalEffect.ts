import { type DependencyList, useEffect } from "react";
import type { MaybeAsync } from "$type/generic";
import { awaitAbortSignal } from "$util/generic";

// biome-ignore lint/suspicious/noConfusingVoidType: ignore void warning
type CleanupFunction = (() => void) | void;

/**
 * Runs an async effect that can be cancelled.
 * - Cancel effect runs when component unmounts.
 * - When that happens, your cleanup function will not run.
 */
export function useAsyncSignalEffect(
    effect: MaybeAsync<(signal: AbortSignal) => CleanupFunction>,
    deps: DependencyList,
) {
    useEffect(() => {
        const abortController = new AbortController();
        let cleanup: CleanupFunction | null = null;

        (async () => {
            try {
                cleanup = await Promise.race([
                    awaitAbortSignal(abortController.signal),
                    effect(abortController.signal),
                ]);
            } catch (error) {
                console.error("One of the effects threw an error: ", error);
            }
        })();

        return () => {
            cleanup?.();
            abortController.abort(`[ ${useAsyncSignalEffect.name} ]: Unmounted`);
        };
        // biome-ignore lint/correctness/useExhaustiveDependencies: This is fine.
    }, deps);
}
