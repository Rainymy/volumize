import type { HTMLAttributes } from "react";
import { FaVolumeMute, FaVolumeUp } from "react-icons/fa";
import { AppButton } from "$base/button";
import style from "./index.module.less";

type ToggleIconType = {
    is_mute: boolean;
    onClick?: React.MouseEventHandler<HTMLButtonElement>;
};
type HTMLDivAttributes = HTMLAttributes<HTMLDivElement> | null;

export function ToggleableMuteIcon({
    is_mute,
    onClick,
    ...attributes
}: ToggleIconType & HTMLDivAttributes) {
    return (
        <div {...attributes}>
            <AppButton type="button" className={style.toggle_button} onClick={onClick}>
                {is_mute ? <FaVolumeMute /> : <FaVolumeUp />}
            </AppButton>
        </div>
    );
}
