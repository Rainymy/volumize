import { Card } from "$component/card";
import type { AudioApplication } from "$type/volume";

export function DeviceApplications({ app }: { app: AudioApplication }) {
    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current}
            onButtonClick={() => { }}
            onSlider={(_value) => { }}
        ></Card>
    );
}
