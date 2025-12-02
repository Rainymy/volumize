import { useAtom } from "jotai";
import { useEffect, useMemo, useState } from "react";
import { MdOutlineSpeaker } from "react-icons/md";

import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { useAsyncSignalEffect } from "$hook/useAsyncSignalEffect";
import { useURLObjectIcon } from "$hook/useURLObjectIcon";
import { device_list, selected_device_id } from "$model/volume";
import { UPDATE_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import {
    isAppIdentifier,
    isAudioVolumeChange,
    isDeviceIdentifier,
    type UpdateChange,
} from "$type/update";
import type { AppIdentifier, AudioApplication } from "$type/volume";

export function DeviceMaster() {
    const [selected_id, _refreshSessions] = useAtom(selected_device_id);
    const [devices, setDevices] = useAtom(device_list);

    const master = useMemo(() => {
        return devices.find((device) => device.id === selected_id);
    }, [selected_id, devices]);

    useEffect(() => {
        function updateHandle(event: EventType<UpdateChange>) {
            const data = event.detail;
            if (data && isDeviceIdentifier(data.id) && isAudioVolumeChange(data.change)) {
                const id = data.id.content;
                const volume = {
                    current: data.change.volume,
                    muted: data.change.mute,
                };
                setDevices((prev) => {
                    return prev.map((device) => {
                        if (device.id === id) {
                            return {
                                ...device,
                                volume: { ...volume },
                            };
                        }
                        return device;
                    });
                });
            }
        }

        document.body.addEventListener(UPDATE_EVENT, updateHandle);
        return () => {
            document.body.removeEventListener(UPDATE_EVENT, updateHandle);
        };
    }, [setDevices]);

    if (!master) {
        return null;
    }

    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current}
            icon={<MdOutlineSpeaker />}
            onButtonClick={() => {
                return volumeController.toggleMuteMaster(master.id, master.volume.muted);
            }}
            onSlider={(value) => {
                return volumeController.deviceSetVolume(master.id, value);
            }}
        ></Card>
    );
}

export function DeviceApplications({ id }: { id: AppIdentifier }) {
    const [app, setApp] = useState<AudioApplication | null>(null);
    const base64 = useURLObjectIcon(app?.process.id);

    useAsyncSignalEffect(
        async (signal) => {
            const data = await volumeController.getApplication(id);
            if (!signal.aborted) {
                setApp(() => data);
            }
        },
        [id],
    );

    useEffect(() => {
        function updateHandle(event: EventType<UpdateChange>) {
            const data = event.detail;
            if (data === undefined) return;

            if (isAppIdentifier(data.id) && isAudioVolumeChange(data.change)) {
                if (data.id.content !== app?.process.id) {
                    return;
                }
                const volume = {
                    current: data.change.volume,
                    muted: data.change.mute,
                };
                setApp((prev) => {
                    if (prev === null) return null;
                    return { ...prev, volume };
                });
            }
        }

        document.body.addEventListener(UPDATE_EVENT, updateHandle);
        return () => {
            document.body.removeEventListener(UPDATE_EVENT, updateHandle);
        };
    }, [app?.process.id]);

    if (!app) {
        return null;
    }

    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current}
            icon={base64 ? base64 : undefined}
            onButtonClick={() => {
                volumeController.toggleMuteApp(app.process.id, app.volume.muted);
            }}
            onSlider={(value) => {
                volumeController.applicationSetVolume(app.process.id, value);
            }}
        ></Card>
    );
}
