import { VSlider } from "$component/slider";
import wrapper from "./index.module.less";

export function Card({
    title,
    volume,
    isMuted,
    onButtonClick,
    onSlider,
}: {
    title: string;
    volume: number;
    isMuted: boolean;
    onButtonClick: () => void;
    onSlider: (value: string) => void;
}) {
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

            <button
                type="button"
                onClick={() => {
                    onButtonClick();
                    console.log("Toggle mute:", title);
                }}
            >
                {isMuted ? "Muted" : "Not Muted"}
            </button>
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
