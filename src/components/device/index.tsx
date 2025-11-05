import { useSetAtom } from "jotai";
import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { audio_session } from "$model/volume";
import type { AudioApplication, AudioDevice } from "$type/volume";

export function DeviceMaster({ master }: { master: AudioDevice }) {
    const refreshable = useSetAtom(audio_session);

    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current}
            onButtonClick={async () => {
                await volumeController.toggleMuteMaster(
                    master.id,
                    master.volume.muted,
                );
                refreshable();
            }}
            onSlider={async (value) => {
                await volumeController.setMasterVolume(master.id, value);
            }}
        ></Card>
    );
}

export function DeviceApplications({ app }: { app: AudioApplication }) {
    const refreshable = useSetAtom(audio_session);

    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current}
            onButtonClick={async () => {
                await volumeController.toggleMuteApp(
                    app.process.id,
                    app.volume.muted,
                );
                refreshable();
            }}
            onSlider={async (value) => {
                volumeController.setAppVolume(app.process.id, value);
                console.log("application: ", app);
            }}
        ></Card>
    );
}
