import { type ChangeEventHandler, useEffect, useRef, useState } from "react";
import { getNumber } from "$util/generic";
import { classnames } from "$util/react";
import style from "./index.module.less";

type SliderType = {
    className?: string;
    value?: string | number;
    min?: number;
    max?: number;
    step?: number;
    onChange?: ChangeEventHandler<HTMLInputElement>;
};

export function Slider(props: SliderType) {
    const input_class = classnames([style.input_range_style, props.className]);
    const container = classnames([style.container]);

    const ref = useRef<HTMLInputElement>(null);
    const [currentValue, setValue] = useState(Number(props.value));

    useEffect(() => {
        setValue(Number(props.value));
    }, [props.value]);

    useEffect(() => {
        const slider = ref.current;
        if (!slider) return;

        const handleInput = (el: HTMLInputElement) => {
            const min = getNumber(el.min) || (props.min ?? 0);
            const max = getNumber(el.max) || (props.max ?? 100);
            const pct = ((el.valueAsNumber - min) / (max - min)) * 100;
            el.style.setProperty("--range-pct", `${pct}%`);
        };
        const handleInputEvent = (e: Event) => handleInput(e.target as HTMLInputElement);
        slider.addEventListener("input", handleInputEvent);
        handleInput(slider);

        return () => {
            slider.removeEventListener("input", handleInputEvent);
        };
    }, [props.min, props.max]);

    return (
        <div className={container}>
            <input
                className={input_class}
                ref={ref}
                type="range"
                min={props.min}
                max={props.max}
                step={props.step}
                value={currentValue}
                onChange={(event) => {
                    setValue(event.target.valueAsNumber);
                    props.onChange?.(event);
                }}
            />
            <span>{parseFloat(currentValue.toFixed(2))}</span>
        </div>
    );
}

export function VSlider({ className, ...props }: SliderType) {
    return <Slider {...props} className={classnames([style.vslider, className])} />;
}
