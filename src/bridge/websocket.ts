export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: URL;

    constructor(url: string, port: number) {
        this.connect_URL = new URL(`ws://${url}:${port}`);
    }

    async retryUntilConnection(): Promise<WebSocket> {
        let index = 1;
        while (index <= 5) {
            try {
                console.log("Connecting...", index++);
                await this.retryDelay();
                await this.__connect__();
                console.log("new connection");
                return this.socket as WebSocket;
            } catch {
                console.log("Failed to connect retying", index);
            }
        }

        throw Error("Connection failed");
    }

    async retryDelay() {
        return new Promise<void>((resolve) => setTimeout(resolve, 1000));
    }

    async __connect__() {
        return new Promise<void>((resolve, reject) => {
            if (!this.socket) {
                const socket = new WebSocket(this.connect_URL);

                socket.onerror = (_event) => {
                    this.socket = null;
                    reject();
                };

                socket.onopen = (_event) => {
                    this.socket = socket;
                    resolve();
                };

                return;
            }

            if (this.socket.readyState === this.socket.OPEN) {
                return resolve();
            }

            reject();
        });
    }
}
