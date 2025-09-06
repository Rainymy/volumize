import { AppButton } from "$component/button";
import type { AudioDevice } from "$util/volumeType";

import wrapper from "./index.module.less";

export function Sidebar({
    devices,
    activeID,
    onSelectDevice,
}: {
    devices: AudioDevice[];
    activeID: string;
    onSelectDevice: (id: string) => void;
}) {
    return (
        <aside className={wrapper.container}>
            <h2 className="font-bold mb-2">Devices</h2>
            {devices.map((device) => (
                <AppButton
                    is_active={device.id === activeID}
                    key={device.id}
                    onClick={() => onSelectDevice(device.id)}
                >
                    {device.name}
                </AppButton>
            ))}
        </aside>
    );
}
