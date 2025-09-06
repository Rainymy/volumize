import { FaVolumeMute, FaVolumeUp } from "react-icons/fa";
import { AppButton } from "$component/button";

import style from "./index.module.less";

export function ToggleableMuteIcon({ is_mute }: { is_mute: boolean }) {
    // onClick={() => {
    //     onButtonClick();
    //     console.log("Toggle mute:", title);
    // }}

    return (
        <div>
            <AppButton type="button">
                {is_mute ? <FaVolumeMute /> : <FaVolumeUp />}
            </AppButton>
        </div>
    );
}
