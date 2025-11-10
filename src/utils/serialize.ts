export function serializeToString<T>(value: T): string {
    if (typeof value === "string") {
        return value;
    }

    // Primitives
    if (
        value === null ||
        typeof value === "undefined" ||
        typeof value === "number" ||
        typeof value === "boolean" ||
        typeof value === "bigint"
    )
        return `${value}`;

    // Date
    if (value instanceof Date) {
        return value.toISOString();
    }

    // Try value.toString if it’s customized
    const custom_string = (value as string)?.toString?.();
    if (typeof custom_string === "string" && custom_string !== "[object Object]") {
        return custom_string;
    }

    // Fallback to JSON
    try {
        const s = JSON.stringify(value);
        if (typeof s === "string") {
            return s;
        }
    } catch {
        // ignore
    }

    // Final fallback e.g. "[object Object]"
    return `${value}`;
}
