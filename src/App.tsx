import { useAtom } from "jotai";
import { AudioMixer } from "$component/audioMixer";
import { Container } from "$component/container";
import { audio_session } from "./models/volume";

export default function App() {
    const [sessions, _refreshSessions] = useAtom(audio_session);

    return (
        <Container>
            <h1>Welcome to Volumize</h1>
            <p>Look around and see</p>
            <AudioMixer sessions={sessions} />
        </Container>
    );
}
