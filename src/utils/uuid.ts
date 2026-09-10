type UUID = ReturnType<typeof window.crypto.randomUUID>;

export function uuid(): UUID {
    return window.crypto.randomUUID();
}

export function uuid_number(): number {
    // uuid 8 * 8 = 64bit
    const bytes = new Uint8Array(8);
    window.crypto.getRandomValues(bytes);

    let hex = "0x";
    for (const byte of bytes) {
        hex += byte.toString(16).padStart(2, "0");
    }
    return Number(hex);
}
