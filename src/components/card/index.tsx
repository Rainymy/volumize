import { VSlider } from "$component/slider";
import { ToggleableMuteIcon } from "$component/toggleMuteIcon";
import wrapper from "./index.module.less";

type CardType = {
    title: string;
    volume: number;
    isMuted: boolean;
    onButtonClick?: () => void;
    onSlider: (value: string) => void;
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
                onChange={(val) => {
                    onSlider(val.currentTarget.value);
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
