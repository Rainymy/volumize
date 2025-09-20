import { RUST_INVOKE } from "$bridge/volumeManager";
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
    const command = {
        [RUST_INVOKE.GET_DEVICE_VOLUME]: ["5", 2]
    }
    connection.send(JSON.stringify(command));

    connection.addEventListener("message", (event) => {
        console.log(event.data)
    });

}
