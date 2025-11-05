import { FaVolumeMute, FaVolumeUp } from "react-icons/fa";
import { AppButton } from "$base/button";

import style from "./index.module.less";

type ToggleIconType = {
    is_mute: boolean;
    onClick?: React.MouseEventHandler<HTMLButtonElement>;
};

export function ToggleableMuteIcon({ is_mute, onClick }: ToggleIconType) {
    return (
        <div>
            <AppButton
                type="button"
                className={style.toggle_button}
                onClick={onClick}
            >
                {is_mute ? <FaVolumeMute /> : <FaVolumeUp />}
            </AppButton>
        </div>
    );
}
