export class ConnectSocket {
    public socket: WebSocket | null = null;
    private connect_URL: URL = new URL("ws://localhost:9001");
    private cancelResolve: (() => void) | null = null;

    setup(url: string, port: number) {
        this.connect_URL = new URL(`ws://${url}:${port}`);
    }

    async retryUntilConnection(): Promise<ConnectSocket | null> {
        const cancelPromise = new Promise((_, reject) => {
            this.cancelResolve = () => reject(new Error("Cancelled"));
        });

        let index = 0;
        while (index < 5) {
            try {
                console.log("Connecting...", ++index);

                await Promise.race([this.retryDelay(), cancelPromise]);
                await Promise.race([this.__connect__(), cancelPromise]);

                console.log("new connection");
                return this;
            } catch (error) {
                if ((error as Error)?.message === "Cancelled") {
                    console.log("Connection cancelled");
                    return null;
                }
                else {
                    console.log("Failed to connect retying", index);
                }
            }
        }

        console.warn("Connection failed");
        return null;
    }

    async retryDelay() {
        return new Promise<void>((resolve) => setTimeout(resolve, 1000));
    }

    close() {
        this.cancelResolve?.();
        this.socket?.close();
        this.socket = null;
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

            reject(new Error("Socket not ready"));
        });
    }
}
