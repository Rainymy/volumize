export class ConnectSocket {
    socket: WebSocket | null = null;

    async retryUntilConnection(): Promise<WebSocket | null> {
        let index = 1;
        while (index <= 5) {
            try {
                console.log("Connecting...", index++);
                await this.retryDelay();
                return await this.__connect__();
            } catch {
                console.log("Failed to connect retying", index);
            }
        }

        return null;
    }

    async retryDelay() {
        return new Promise((resolve) => setTimeout(resolve, 1000));
    }

    __connect__(): Promise<WebSocket> {
        return new Promise((resolve, reject) => {
            const socket = new WebSocket("ws://localhost:9001");

            socket.onerror = (_event) => {
                reject();
            };

            socket.onopen = (_event) => {
                resolve(socket);
            };
        });
    }
}