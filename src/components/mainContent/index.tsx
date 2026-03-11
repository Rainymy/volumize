import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { useEffect } from "react";

import { volumeController } from "$bridge/volumeManager";
import { DeviceApplications, DeviceMaster } from "$component/device";
import { Heartbeat } from "$component/heartbeat";
import { useGenerateID } from "$hook/useGenerateID";
import { application_ids, device_list, selected_device_id } from "$model/volume";
import { UPDATE_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import {
    isAppIdentifier,
    isAudioVolumeChange,
    isDeviceIdentifier,
    isStateChange,
    type UpdateChange,
} from "$type/update";

import style from "./index.module.less";

/*                          UI Design
 * [   static    ][                    Carousel                    ]
 * |-------------||-------------|-------------|-------------|------|
 * |             ||             |             |             |      |
 * | Device Info ||    App 1    |    App 2    |    App 3    |    Ap|
 * |             ||             |             |             |      |
 * |             ||             |             |             |      |
 * |             ||             |             |             |      |
 * |-------------||-------------|-------------|-------------|------|
 */

export function MainContent() {
    const setAppIds = useSetAtom(application_ids);
    const setDeviceList = useSetAtom(device_list);
    const device_id = useAtomValue(selected_device_id);

    useEffect(() => {
        volumeController.getPlaybackDevices().then((devices) => setDeviceList(devices));
    }, [setDeviceList]);

    useEffect(() => {
        if (device_id === undefined) return;
        volumeController
            .getDeviceApplications(device_id)
            .then((apps) => setAppIds(() => apps));
    }, [setAppIds, device_id]);

    return (
        <div className={style.container}>
            <Heartbeat />
            <DeviceListener />
            <ApplicationsListener />
        </div>
    );
}

function DeviceListener() {
    const [selected_id, _refreshSessions] = useAtom(selected_device_id);
    const [devices, setDevices] = useAtom(device_list);

    const master = devices.find((device) => device.id === selected_id);

    useEffect(() => {
        function updateHandle(event: EventType<UpdateChange>) {
            const data = event.detail;
            if (data === undefined) return;

            if (isDeviceIdentifier(data.id) && isAudioVolumeChange(data.change)) {
                const volume = {
                    current: data.change.volume,
                    muted: data.change.mute,
                };
                setDevices((prev) => {
                    return prev.map((device) => {
                        if (device.id === data.id.content) {
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

    return <DeviceMaster master={master} key={master.id} />;
}

function ApplicationsListener() {
    const [app_ids, setAppIds] = useAtom(application_ids);
    const elementsWithId = useGenerateID(app_ids);

    useEffect(() => {
        function handleUpdateEvent(event: EventType<UpdateChange>) {
            const data = event.detail;
            if (data === undefined) return;

            if (isAppIdentifier(data.id) && isStateChange(data.change)) {
                const id = data.id.content;
                if (data.change.state === "created") {
                    setAppIds((prev) => [...prev, id]);
                }
                if (data.change.state === "disconnect") {
                    setAppIds((prev) => prev.filter((item) => item !== id));
                }
            }
        }

        document.body.addEventListener(UPDATE_EVENT, handleUpdateEvent);
        return () => {
            document.body.removeEventListener(UPDATE_EVENT, handleUpdateEvent);
        };
    }, [setAppIds]);

    return elementsWithId.map(({ element, id: key }) => {
        return <DeviceApplications id={element} key={key} />;
    });
}
