import { ConnectSocket } from "$bridge/websocket";
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


const connection = await new ConnectSocket().retryUntilConnection();

if (connection) {
    connection.send("hello from client!");

    connection.addEventListener("message", (event) => {
        console.log(event.data)
    });

}
