import type { ReactNode } from "react";

import { CardIcon } from "$base/cardIcon";
import { VSlider } from "$base/slider";
import { ToggleableMuteIcon } from "$base/toggleMuteIcon";
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
 * The `onSlider` function is called when the slider value changes. May return `NaN`.
 *
 * The `volume` prop must be a number between 0 and 1.
 * @param props
 * @returns
 */
export function Card(props: CardProps) {
    const { title, volume, isMuted, onButtonClick, onSlider, icon } = props;

    return (
        <div className={style.container}>
            <CardTitle title={title} />
            <CardIcon icon={icon} />

            <VSlider
                value={volume * 100}
                max={100}
                min={0}
                step={0.1}
                onChange={(val) => {
                    if (Number.isNaN(val.currentTarget.valueAsNumber)) {
                        console.error("Invalid volume value", val.currentTarget);
                        return;
                    }
                    onSlider?.(val.currentTarget.valueAsNumber / 100);
                }}
            />

            <ToggleableMuteIcon is_mute={isMuted} onClick={onButtonClick} />
        </div>
    );
}

export function CardTitle({ title }: { title: string }) {
    return (
        <span title={title} className={style.title}>
            {title}
        </span>
    );
}
