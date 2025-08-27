import { useQuery } from "@tanstack/react-query";
import { AudioMixer } from "$component/audioMixer";
import { Container } from "$component/container";
import { volumeController } from "./bridge/volumeManager";

export default function App() {
    const { data: sessions, error } = useQuery({
        queryFn: () => volumeController.getAllApplications(),
        queryKey: [],
        initialData: [],
    });

    if (error) {
        console.log(error);
        return <p>Something went wrong</p>;
    }

    return (
        <Container>
            <h1>Welcome to Volumize</h1>
            <p>Look around and see</p>
            <AudioMixer sessions={sessions} />
        </Container>
    );
}
