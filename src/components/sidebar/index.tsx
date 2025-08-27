import type { AudioDevice } from "$util/volumeType";

export function Sidebar({
    devices,
    onSelectDevice,
}: {
    devices: AudioDevice[];
    onSelectDevice: (id: string) => void;
}) {
    return (
        <aside className="col-span-1 border-r p-4 space-y-2">
            <h2 className="font-bold mb-2">Devices</h2>
            {devices.map((device) => (
                <button
                    type="button"
                    key={device.id}
                    className="w-full justify-start"
                    onClick={() => onSelectDevice(device.id)}
                >
                    {device.name}
                </button>
            ))}
        </aside>
    );
}
