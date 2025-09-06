export function isEnumValue<T extends string>(
    enumObject: Record<string, T>,
    value: unknown,
): value is T {
    return Object.values(enumObject).includes(value as T);
}

export function getIncludes<T extends readonly unknown[]>(
    enumArray: T,
    value: unknown,
): T[number] | null {
    if (!enumArray.includes(value as T[number])) {
        return null;
    }
    return value as T[number];
}

export function getEnumIncludes<E extends Record<string, string>>(
    enumObject: E,
    value: unknown,
): E[keyof E] | null {
    if (!isEnumValue(enumObject, value as E[keyof E])) {
        return null;
    }
    return value as E[keyof E];
}