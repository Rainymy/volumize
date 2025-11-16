type UUID = ReturnType<typeof window.crypto.randomUUID>;

export function uuid(): UUID {
    return window.crypto.randomUUID();
}
