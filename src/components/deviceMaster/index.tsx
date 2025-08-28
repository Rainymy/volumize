import { Card } from "$component/card";
import type { AudioDevice } from "$util/volumeType";

export function DeviceMaster({ master }: { master: AudioDevice }) {
    return (
        <Card
            isMuted={master.volume.muted}
            title={master.name}
            volume={master.volume.current}
            onButtonClick={() => { }}
            onSlider={(_value) => { }}
        ></Card>
    );
}
