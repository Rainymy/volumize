import { atom } from "jotai";
import type { SimplePrimitives } from "$type/generic";
import { serializeToString } from "$util/serialize";

export function proxyStorageAtom<T extends SimplePrimitives>(value: T, key: string) {
    const __inner__ = atom(value);

    return atom(
        (get) => get(__inner__),
        (_, set, newValue: T) => {
            set(__inner__, newValue);
            localStorage.setItem(key, serializeToString(newValue));
        },
    );
}
