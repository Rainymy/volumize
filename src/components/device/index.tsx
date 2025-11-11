import { useAtom } from "jotai";
import { useMemo, useState } from "react";
import { MdOutlineSpeaker } from "react-icons/md";

import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { useAsyncSignalEffect } from "$hook/useAsyncSignalEffect";
import { useRefreshable } from "$hook/useRefreshable";
import { device_list, selected_device_id } from "$model/volume";
import type { AppIdentifier, AudioApplication } from "$type/volume";

export function DeviceMaster() {
    const [selected_id, _refreshSessions] = useAtom(selected_device_id);
    const [devices, refreshable] = useAtom(device_list);

    const master = useMemo(() => {
        return devices.find((device) => device.id === selected_id);
    }, [selected_id, devices]);

    if (!master) {
        return null;
    }

    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current}
            icon={<MdOutlineSpeaker />}
            onButtonClick={async () => {
                await volumeController.toggleMuteMaster(master.id, master.volume.muted);
                refreshable();
            }}
            onSlider={async (value) => {
                await volumeController.setDeviceVolume(master.id, value);
            }}
        ></Card>
    );
}

export function DeviceApplications({ id }: { id: AppIdentifier }) {
    const [app, setApp] = useState<AudioApplication | null>(null);
    // This is used as a trigger to refresh the application data.
    // - As force rerender of this component.
    const [token, refreshable] = useRefreshable();

    useAsyncSignalEffect(
        async (signal) => {
            token;
            const data = await volumeController.findApplicationWithId(id, id);
            if (!signal.aborted) {
                setApp(() => data);
            }
        },
        [id, token],
    );

    if (!app) {
        return null;
    }

    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current}
            icon={app.process.path}
            onButtonClick={async () => {
                await volumeController.toggleMuteApp(app.process.id, app.volume.muted);
                // Force rerender of this component.
                refreshable();
            }}
            onSlider={async (value) => {
                volumeController.setAppVolume(app.process.id, value);
            }}
        ></Card>
    );
}
