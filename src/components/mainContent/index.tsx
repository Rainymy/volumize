import { useAtomValue } from "jotai";
import { useEffect } from "react";
import { volumeController } from "$bridge/volumeManager";
import { DeviceApplications, DeviceMaster } from "$component/device";
import { useGenerateID } from "$hook/useGenerateID";
import { useLogout } from "$hook/useLogout";
import { application_ids } from "$model/volume";
import { HEARTBEAT } from "$type/constant";
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
    const app_ids = useAtomValue(application_ids);
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

    return (
        <div className={style.container}>
            <DeviceMaster />
            {elementsWithId.map(({ element, id: key }) => {
                return <DeviceApplications id={element} key={key} />;
            })}
        </div>
    );
}
