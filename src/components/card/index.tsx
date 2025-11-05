import { VSlider } from "$base/slider";
import { ToggleableMuteIcon } from "$base/toggleMuteIcon";
import type { MaybeAsync } from "$type/generic";

import wrapper from "./index.module.less";

type CardType = {
    title: string;
    volume: number;
    isMuted: boolean;
    onButtonClick?: MaybeAsync<() => void>;
    onSlider?: MaybeAsync<(value: number) => void>;
};

export function Card({
    title,
    volume,
    isMuted,
    onButtonClick,
    onSlider,
}: CardType) {
    return (
        <div className={wrapper.container}>
            <CardTitle title={title}></CardTitle>

            <VSlider
                value={volume * 100}
                max={100}
                min={0}
                step={0.1}
                onChange={(val) => onSlider?.(val.currentTarget.valueAsNumber / 100)}
            />

            <ToggleableMuteIcon is_mute={isMuted} onClick={onButtonClick} />
        </div>
    );
}

export function CardTitle({ title }: { title: string }) {
    return (
        <span title={title} className={wrapper.title}>
            {title}
        </span>
    );
}
