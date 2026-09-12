import { memo, useEffect, useState } from "react";
import { MdOutlineSpeaker } from "react-icons/md";

import { volumeController } from "$bridge/volumeManager";
import { Card } from "$component/card";
import { useAsyncSignalEffect } from "$hook/useAsyncSignalEffect";
import { useURLObjectIcon } from "$hook/useURLObjectIcon";
import {
    type AudioApplication,
    type AudioDevice,
    UPDATE_EVENT_NAME,
    type UpdateChange,
} from "$type/bindings";
import type { EventType } from "$type/generic";
import { isAppIdentifier, isAudioVolumeChange } from "$type/update";
import type { AppIdentifier } from "$type/volume";

function DeviceMaster__({ master }: { master: AudioDevice }) {
    return (
        <Card
            isMuted={master.volume.muted}
            title={master.friendly_name}
            volume={master.volume.current ?? 0}
            icon={<MdOutlineSpeaker />}
            onButtonClick={() => {
                return volumeController.toggleMuteMaster(master.id, master.volume.muted);
            }}
            onSlider={(value) => {
                return volumeController.deviceSetVolume(master.id, value);
            }}
        />
    );
}

function DeviceApplications__({ id }: { id: AppIdentifier }) {
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

        document.body.addEventListener(UPDATE_EVENT_NAME, updateHandle);
        return () => {
            document.body.removeEventListener(UPDATE_EVENT_NAME, updateHandle);
        };
    }, [app?.process.id]);

    if (!app) {
        return null;
    }

    return (
        <Card
            isMuted={app.volume.muted}
            title={app.process.name}
            volume={app.volume.current ?? 0}
            icon={base64}
            onButtonClick={() => {
                volumeController.toggleMuteApp(app.process.id, app.volume.muted);
            }}
            onSlider={(value) => {
                volumeController.applicationSetVolume(app.process.id, value);
            }}
        />
    );
}

export const DeviceApplications = memo(DeviceApplications__);
export const DeviceMaster = memo(DeviceMaster__);
