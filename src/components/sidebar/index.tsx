import type { AudioDevice } from "$util/volumeType";

import wrapper from "./index.module.less";

export function Sidebar({
    devices,
    onSelectDevice,
}: {
    devices: AudioDevice[];
    onSelectDevice: (id: string) => void;
}) {
    return (
        <aside className={wrapper.container}>
            <h2 className="font-bold mb-2">Devices</h2>
            {devices.map((device) => (
                <button
                    type="button"
                    key={device.id}
                    className={wrapper.item_list}
                    onClick={() => onSelectDevice(device.id)}
                >
                    {device.name}
                </button>
            ))}
        </aside>
    );
}
