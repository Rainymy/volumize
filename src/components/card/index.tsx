import type { ReactNode } from "react";
import { BouncyTitle } from "$base/bouncyTitle";
import { CardIcon } from "$base/cardIcon";
import { VSlider } from "$base/slider";
import { ToggleableMuteIcon } from "$base/toggleMuteIcon";
import { useElementFocusedWithin } from "$hook/useElementFocusedWithin";
import type { MaybeAsync } from "$type/generic";

import style from "./index.module.less";

type CardProps = {
    title: string;
    volume: number;
    isMuted: boolean;
    icon?: string | ReactNode;
    onButtonClick?: MaybeAsync<() => void>;
    onSlider?: MaybeAsync<(value: number) => void>;
};

/**
 * The `onButtonClick` function is called when the user clicks on the toggle mute icon.
 *
 * The `onSlider` function is called when the slider value changes. `NaN` value is ignored.
 *
 * The `volume` prop must be a number between 0 and 1 (inclusive).
 * @param props
 * @returns
 */
export function Card(props: CardProps) {
    const { title, volume, isMuted, onButtonClick, onSlider, icon } = props;
    const { isFocusedWithin, ref } = useElementFocusedWithin<HTMLDivElement>();

    return (
        <div className={style.container} ref={ref}>
            <BouncyTitle
                animate={isFocusedWithin}
                className={"flex-grow-1"}
                title={title}
            />
            <CardIcon className={"flex-grow-1"} alt={`${title} icon`} icon={icon} />

            <VSlider
                className={"flex-grow-10"}
                value={volume * 100}
                max={100}
                min={0}
                step={0.1}
                onChange={async (val) => {
                    if (Number.isNaN(val.currentTarget.valueAsNumber)) {
                        console.error("Invalid volume value", val.currentTarget);
                        return;
                    }
                    await onSlider?.(val.currentTarget.valueAsNumber / 100);
                }}
            />

            <ToggleableMuteIcon
                className={"flex-grow-1"}
                is_mute={isMuted}
                onClick={onButtonClick}
            />
        </div>
    );
}
