export class ConnectSocket {
    public socket: WebSocket | null = null;

    async retryUntilConnection(): Promise<WebSocket> {
        let index = 1;
        while (index <= 5) {
            try {
                console.log("Connecting...", index++);
                await this.retryDelay();
                await this.__connect__();
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
                console.log("new connection")
                const socket = new WebSocket("ws://localhost:9001");

                socket.onerror = (_event) => {
                    this.socket = null;
                    reject();
                };

                socket.onopen = (_event) => {
                    this.socket = socket;
                    resolve();
                };

                socket.addEventListener("message", (event) => {
                    console.log("webosocket:", event);
                });

                return;
            }

            if (this.socket.readyState === this.socket.OPEN) {
                return resolve();
            }

            reject();
        });
    }
}