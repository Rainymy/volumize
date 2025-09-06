import { Card } from "$component/card";
import type { AudioDevice } from "$util/volumeType";

export function DeviceMaster({ master }: { master: AudioDevice }) {
    console.log("current: ", master);

    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current}
            onButtonClick={() => { }}
            onSlider={(_value) => { }}
        ></Card>
    );
}
