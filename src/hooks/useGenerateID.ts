import { useMemo } from "react";

const uuid: Crypto = window.crypto;
type UUID = ReturnType<typeof uuid.randomUUID>;

type IDPair<T> = { id: UUID; element: T };

export function useGenerateID<T>(elements: T[]): IDPair<T>[] {
    return useMemo(
        () => elements.map((t) => ({ id: uuid.randomUUID(), element: t }) as IDPair<T>),
        [elements],
    );
}
