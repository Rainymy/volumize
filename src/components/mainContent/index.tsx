import { useAtomValue } from "jotai";

import { DeviceApplications, DeviceMaster } from "$component/device";
import { useGenerateID } from "$hook/useGenerateID";
import { application_ids } from "$model/volume";

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

    return (
        <div className={style.container}>
            <DeviceMaster />
            {elementsWithId.map(({ element, id: key }) => {
                return <DeviceApplications id={element} key={key} />;
            })}
        </div>
    );
}
