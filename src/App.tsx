import { AudioMixer } from "$component/audioMixer";
import { Container } from "$component/container";

export default function App() {
    return (
        <Container>
            <h1>Welcome to Volumize</h1>
            <p>Look around and see</p>
            <AudioMixer />
        </Container>
    );
}

class ConnectSocket {
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

const connection = await new ConnectSocket().retryUntilConnection();

if (connection) {
    connection.send("hello");
}
