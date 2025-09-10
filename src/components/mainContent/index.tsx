import { useAtom } from "jotai";
import { DeviceApplications } from "$component/applicationDevice";
import { DeviceMaster } from "$component/deviceMaster";
import { useGenerateID } from "$hook/useGenerateID";
import { audio_session, selected_device_id } from "$model/volume";
import wrapper from "./index.module.less";

/*                          The vision
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
    const [session, _refreshSessions] = useAtom(audio_session);
    const [selectedDevice, _setSelectedDevice] = useAtom(selected_device_id);

    const defaultSession = session.find((val) => val.device.id === selectedDevice);
    const applicationsWithId = useGenerateID(defaultSession?.applications ?? []);

    if (!defaultSession) {
        return null;
    }

    return (
        <main className={wrapper.container}>
            <DeviceMaster master={defaultSession.device}></DeviceMaster>
            {applicationsWithId.map(([element, key]) => {
                return <DeviceApplications app={element} key={key} />;
            })}
        </main>
    );
}
