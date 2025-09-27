import WebSocket from "@tauri-apps/plugin-websocket";

export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: string = "ws://localhost:9001";

    setup(url: string, port: number) {
        this.connect_URL = `ws://${url}:${port}`;
    }

    async close() {
        await this.socket?.disconnect();
        this.socket = null;
    }

    async connect() {
        this.socket = await WebSocket.connect(this.connect_URL);
    }

    async send(data: string) {
        this.socket?.send({ type: "Text", data: data })
    }
}
