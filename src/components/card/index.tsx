import { VSlider } from "$component/slider";
import { ToggleableMuteIcon } from "$component/toggleMuteIcon";
import type { MaybeAsync } from "$type/generic";
import { getNumber } from "$util/generic";
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
                defaultValue={volume * 100}
                max={100}
                min={0}
                step={0.1}
                onChange={async (val) => {
                    const slide_value = getNumber(val.currentTarget.value);

                    if (typeof slide_value === "number") {
                        await onSlider?.(slide_value / 100);
                    }

                }}
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
