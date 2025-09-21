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
        [RUST_INVOKE.GET_APP_VOLUME]: [13112]
    };
    connection.send(JSON.stringify(command));
    // connection.send(`"${RUST_INVOKE.GET_ALL_APPLICATIONS}"`);

    connection.addEventListener("message", (event) => {
        try {
            console.log("json: ", JSON.parse(event.data));
        } catch {
            console.log("string: ", event.data)
        }
    });

}
