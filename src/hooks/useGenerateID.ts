import { useMemo } from "react";

const crypto = window.crypto;

export function useGenerateID<T>(elements: T[]) {
    return useMemo(
        () => elements.map((t) => [t, crypto.randomUUID()] as [T, ReturnType<typeof crypto.randomUUID>]),
        [elements],
    );
}
