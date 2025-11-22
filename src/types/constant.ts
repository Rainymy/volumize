export enum PORT {
    MAX = 2 ** 16, // 65 536
    MIN = 2 ** 10, // 1024
}

export enum HEARTBEAT {
    CHECK_DELAY_MS = 1000,
    MAX_RETRY_COUNT = 3,
}

// In milliseconds
export enum DEBOUNCE_DELAY {
    NORMAL = 100,
    SLOW = 200,
    FAST = 70,
    SUPER_FAST = 50,
}
