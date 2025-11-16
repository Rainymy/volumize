import { useMemo } from "react";
import { uuid } from "$util/uuid";

type IDPair<T> = {
    id: ReturnType<typeof window.crypto.randomUUID>;
    element: T;
};

export function useGenerateID<T>(elements: T[]): IDPair<T>[] {
    return useMemo(
        () => elements.map((t) => ({ id: uuid(), element: t }) as IDPair<T>),
        [elements],
    );
}
