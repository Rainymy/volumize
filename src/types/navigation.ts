export enum NavigationType {
    HOME = "/",
    MAIN = "/main",
    SETTINGS = "/settings",
}

export enum ConnectionState {
    CONNECTED = "connected",
    LOADING = "loading",
    DISCONNECTED = "disconnected",
}

export enum CONNECTION_MODE {
    DISCOVERY = "discovery",
    MANUAL = "manual",
    TAURI = "tauri",
}

export type TauriConnection = {
    kind: "tauri";
    url: string;
    port: number;
};

export type WebConnection = {
    kind: "web";
    url: string;
    port: number;
};

type Connection = TauriConnection | WebConnection;

export function isWebConnection(connection: Connection): connection is WebConnection {
    return connection.kind === "web";
}

export function isTauriConnection(connection: Connection): connection is TauriConnection {
    return connection.kind === "tauri";
}
