type UUID = ReturnType<typeof window.crypto.randomUUID>;

export function uuid(): UUID {
    return window.crypto.randomUUID();
}

export function random_u32(): number {
    const bytes = new Uint32Array(1);
    window.crypto.getRandomValues(bytes);
    return bytes[0];
}
