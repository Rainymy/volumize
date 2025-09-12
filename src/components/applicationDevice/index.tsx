import { useSetAtom } from "jotai";
import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { audio_session } from "$model/volume";
import type { AudioApplication } from "$type/volume";

export function DeviceApplications({ app }: { app: AudioApplication }) {
    const refreshable = useSetAtom(audio_session);

    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current}
            onButtonClick={async () => {
                await volumeController.toggleMuteApp(app.process.id, app.volume.muted);
                refreshable();
            }}
            onSlider={async (value) => {
                volumeController.setAppVolume(app.process.id, value);
                console.log("application: ", app);
            }}
        ></Card>
    );
}
