import WebSocket, { type Message } from "@tauri-apps/plugin-websocket";

export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: string = "ws://localhost:9001";
    private listeners: (() => void)[] = [];

    setup(url: string, port: number) {
        this.connect_URL = `ws://${url}:${port}`;
    }

    async send(data: string) {
        this.socket?.send({ type: "Text", data: data });
    }

    addListener(cb: (arg: Message) => void) {
        const removeListener = this.socket?.addListener(cb);
        if (removeListener) {
            this.listeners.push(removeListener);
        }
    }

    parse_data(data: Message): { type: string; data: string } | null {
        if (data.type === "Text") {
            return data;
        }
        if (data.type === "Binary") {
            return {
                type: data.type,
                data: Buffer.from(data.data).toString(),
            };
        }
        return null;
    }

    async connect() {
        this.socket = await WebSocket.connect(this.connect_URL);
    }

    async close() {
        for (const listener of this.listeners) listener();
        await this.socket?.disconnect();
        this.socket = null;
    }
}
