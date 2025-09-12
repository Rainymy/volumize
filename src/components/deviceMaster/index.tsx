import { useSetAtom } from "jotai";
import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { audio_session } from "$model/volume";
import type { AudioDevice } from "$type/volume";

export function DeviceMaster({ master }: { master: AudioDevice }) {
    const refreshable = useSetAtom(audio_session);

    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current}
            onButtonClick={async () => {
                await volumeController.toggleMuteMaster(master.id, master.volume.muted);
                refreshable();
            }}
            onSlider={async (value) => {
                volumeController.setMasterVolume(master.id, value);
                console.log("Master: ", master);
            }}
        ></Card>
    );
}
