import WebSocket, { type Message } from "@tauri-apps/plugin-websocket";
import { DEBOUNCE_DELAY, HEARTBEAT, PORT } from "$type/constant";
import { debounce } from "$util/debounce";
import { sleep } from "$util/generic";

type AddListener = ReturnType<WebSocket["addListener"]>;

export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: string = `ws://127.0.0.1:${PORT.DEFAULT}`;
    private listeners: AddListener[] = [];

    set_url(url: string, port: number) {
        this.connect_URL = `ws://${url}:${port}`;
    }

    async send(data: string) {
        await this.socket?.send({ type: "Text", data: data });
    }

    heartbeat = debounce(async () => {
        await this.socket?.send({ type: "Ping", data: [] });

        const waitForPong = new Promise<boolean>((resolve) => {
            const cleanup_handler = this.socket?.addListener((message) => {
                if (message.type === "Pong") {
                    resolve(true);
                    cleanup_handler?.();
                }
            });
        });

        return await Promise.race([sleep(HEARTBEAT.WAIT_FOR_BEAT, false), waitForPong]);
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
        await this.socket?.disconnect();
        this.socket = null;
    }
}
