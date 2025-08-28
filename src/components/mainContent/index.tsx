import { DeviceApplications } from "$component/applicationDevice";
import { DeviceMaster } from "$component/deviceMaster";
import { useGenerateID } from "$hook/useGenerateID";
import type { AudioSession } from "$util/volumeType";

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

export function MainContent({ session }: { session: AudioSession }) {
    const applicationsWithId = useGenerateID(session.applications);

    return (
        <main className={wrapper.container}>
            <DeviceMaster master={session.device}></DeviceMaster>
            {applicationsWithId.map(([element, key]) => {
                return <DeviceApplications app={element} key={key} />;
            })}
        </main>
    );
}
