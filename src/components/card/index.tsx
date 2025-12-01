import { type ReactNode, useRef } from "react";

import { CardIcon } from "$base/cardIcon";
import { VSlider } from "$base/slider";
import { ToggleableMuteIcon } from "$base/toggleMuteIcon";
import { useDetectOverflowX } from "$hook/useDetectOverflowX";
import { useElementFocusedWithin } from "$hook/useElementFocusedWithin";
import { useOverflowAmountX } from "$hook/useOverflowAmount";
import type { MaybeAsync } from "$type/generic";
import { classnames } from "$util/react";

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
 * The `volume` prop must be a number between 0 and 1.
 * @param props
 * @returns
 */
export function Card(props: CardProps) {
    const { title, volume, isMuted, onButtonClick, onSlider, icon } = props;

    const ref = useRef<HTMLDivElement>(null);
    const isParentFocused = useElementFocusedWithin(ref);

    return (
        <div className={style.container} ref={ref}>
            <CardBouncyTitle
                isParentFocused={isParentFocused}
                className={classnames([style.title, "flex-grow-1"])}
                title={title}
            />
            <CardIcon className={"flex-grow-1"} icon={icon} />

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

type TitleProps = {
    title: string;
    className?: string;
    isParentFocused?: boolean;
};

export function CardBouncyTitle({ title, className, isParentFocused }: TitleProps) {
    const ref = useRef<HTMLDivElement>(null);

    const isOverflowing = useDetectOverflowX(ref);
    const overFlowAmount = useOverflowAmountX(ref);

    const animate = isOverflowing && isParentFocused;

    const textclass = classnames([
        style.text_content,
        animate ? style.bounce : null,
        animate && Math.abs(overFlowAmount) < 50 ? style.high_speed : null,
    ]);

    return (
        <div ref={ref} title={title} className={className}>
            <span className={textclass} data-overflow={overFlowAmount}>
                {title}
            </span>
        </div>
    );
}
