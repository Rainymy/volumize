import { useAtom, useAtomValue } from "jotai";
import { useEffect } from "react";
import { volumeController } from "$bridge/volumeManager";
import { DeviceApplications, DeviceMaster } from "$component/device";
import { useGenerateID } from "$hook/useGenerateID";
import { useLogout } from "$hook/useLogout";
import { application_ids, device_list, selected_device_id } from "$model/volume";
import { HEARTBEAT, UPDATE_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import { isAppIdentifier, isStateChange, type UpdateChange } from "$type/update";
import { isSocketController } from "$util/generic";
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
    const [app_ids, setAppIds] = useAtom(application_ids);
    const [_deviceList, setDeviceList] = useAtom(device_list);
    const device_id = useAtomValue(selected_device_id);
    const elementsWithId = useGenerateID(app_ids);
    const logout = useLogout();

    useEffect(() => {
        let retryCount = 0;

        const interval_id = setInterval(async () => {
            if (isSocketController(volumeController)) {
                const hasHeartbeat = await volumeController.heartbeat();
                // const hasHeartbeat = Math.random() < 0.1;

                console.log("Received heartbeat:", hasHeartbeat);
                retryCount = hasHeartbeat ? 0 : retryCount + 1;

                if (!hasHeartbeat) {
                    console.log(
                        "Heartbeat failure, retries left:",
                        HEARTBEAT.MAX_RETRY_COUNT - retryCount,
                    );
                }
                if (retryCount > HEARTBEAT.MAX_RETRY_COUNT) {
                    await logout();
                }
            }
        }, HEARTBEAT.CHECK_DELAY_MS);

        return () => {
            clearInterval(interval_id);
        };
    }, [logout]);

    useEffect(() => {
        volumeController.getPlaybackDevices().then((devices) => {
            setDeviceList(devices);
        });
    }, [setDeviceList]);

    useEffect(() => {
        if (device_id === undefined) return;
        volumeController.getDeviceApplications(device_id).then((apps) => {
            setAppIds(() => apps);
        });
    }, [setAppIds, device_id]);

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

    return (
        <div className={style.container}>
            <DeviceMaster />
            {elementsWithId.map(({ element, id: key }) => {
                return <DeviceApplications id={element} key={key} />;
            })}
        </div>
    );
}
