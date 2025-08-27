import { useMemo } from "react";

const { randomUUID } = window.crypto;

export function useGenerateID<T>(elements: T[]) {
    return useMemo(
        () => elements.map((t) => [t, randomUUID()] as [T, ReturnType<typeof randomUUID>]),
        [elements],
    );
}
