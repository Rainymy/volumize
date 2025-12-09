import WebSocket, { type Message } from "@tauri-apps/plugin-websocket";
import { DEBOUNCE_DELAY, HEARTBEAT, PORT } from "$type/constant";
import { debounce } from "$util/debounce";

type AddListener = ReturnType<WebSocket["addListener"]>;

type PayloadOf<T extends Message["type"]> = Extract<Message, { type: T }>["data"];

export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: string = `ws://127.0.0.1:${PORT.DEFAULT}`;
    private listeners: AddListener[] = [];

    set_url(url: string, port: number) {
        this.connect_URL = `ws://${url}:${port}`;
    }

    async send<T extends Message["type"] = "Text">(
        data: PayloadOf<T>,
        type: T = "Text" as T,
    ) {
        if (this.socket === null) {
            return false;
        }
        try {
            await this.socket.send({ type, data } as Message);
            return true;
        } catch (error) {
            console.error(`Error sending message [${type}]:`, error, data);
        }
        return false;
    }

    heartbeat = debounce(async () => {
        const did_send = await this.send([], "Ping");
        if (!did_send) {
            console.error("Failed to send heartbeat");
            return false;
        }

        return await new Promise<boolean>((resolve) => {
            const cleanup_handler = this.socket?.addListener((message) => {
                if (message.type === "Pong") {
                    resolve(true);
                    cleanup_handler?.();
                }
            });

            setTimeout(() => {
                resolve(false);
                cleanup_handler?.();
            }, HEARTBEAT.WAIT_FOR_BEAT);
        });
    }, DEBOUNCE_DELAY.FAST);

    addListener(cb: (arg: Message) => void) {
        const removeListener = this.socket?.addListener(cb);
        if (removeListener) {
            this.listeners.push(removeListener);
        }
    }

    parse_data(data: Message): object | null {
        if (data.type !== "Text") {
            return null;
        }
        try {
            return JSON.parse(data.data);
        } catch (err) {
            console.log(err);
        }
        return null;
    }

    async connect() {
        this.socket = await WebSocket.connect(this.connect_URL);
    }

    async close() {
        for (const listener of this.listeners) listener();
        try {
            // Internally disconnect uses send to notify the server.
            // This causes issue when the socket is already closed or server closed connection.
            await this.socket?.disconnect();
        } catch (_) {}
        this.socket = null;
    }
}
