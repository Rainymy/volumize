import WebSocket, { type Message } from "@tauri-apps/plugin-websocket";
import { DEBOUNCE_DELAY } from "$type/constant";
import { debounce } from "$util/debounce";
import { sleep } from "$util/generic";

type AddListener = ReturnType<WebSocket["addListener"]>;

export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: string = "ws://localhost:9001";
    private listeners: AddListener[] = [];

    set_url(url: string, port: number) {
        this.connect_URL = `ws://${url}:${port}`;
    }

    async send(data: string) {
        await this.socket?.send({ type: "Text", data: data });
    }

    heartbeat = debounce(async () => {
        await this.socket?.send({ type: "Ping", data: [] });

        let cleanup_handler: AddListener | undefined;

        const waitForPong = new Promise<boolean>((resolve, _) => {
            cleanup_handler = this.socket?.addListener((message) => {
                if (message.type === "Pong") {
                    resolve(true);
                }
            });
        });

        const result = await Promise.race([sleep(2000), waitForPong]);
        cleanup_handler?.();

        return result ?? false;
    }, DEBOUNCE_DELAY.FAST);

    addListener(cb: (arg: Message) => void) {
        const removeListener = this.socket?.addListener(cb);
        if (removeListener) {
            this.listeners.push(removeListener);
        }
    }

    parse_data(data: Message): { channel: string; data: string } | null {
        if (data.type !== "Text") {
            return null;
        }
        try {
            type UpdateEvent = { event: string; payload: object };
            type DataEvent = { type: string; data: string };

            const data2: DataEvent | UpdateEvent = JSON.parse(data.data) as DataEvent;
            // console.log("parse_data:", data2);
            return {
                channel: data2.type,
                data: data2.data,
            };
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
