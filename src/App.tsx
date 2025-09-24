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
