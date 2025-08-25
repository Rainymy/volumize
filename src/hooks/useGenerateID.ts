import { randomUUID, type UUID } from "node:crypto";
import { useMemo } from "react";

export function useGenerateID<T>(elements: T[]) {
    return useMemo(
        () =>
            elements.map((t) => [t, randomUUID()] as [T, UUID]),
        [elements],
    );
}
