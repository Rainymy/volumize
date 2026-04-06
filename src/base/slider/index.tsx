import {
    type ChangeEventHandler,
    useCallback,
    useEffect,
    useLayoutEffect,
    useRef,
    useState,
} from "react";
import { getNumber } from "$util/generic";
import { classnames } from "$util/react";
import style from "./index.module.less";
import { Markers } from "./marker";

type SliderType = {
    className?: string;
    value?: string | number;
    min?: number;
    max?: number;
    step?: number;
    onChange?: ChangeEventHandler<HTMLInputElement>;
};

export function Slider({ min = 0, max = 100, ...props }: SliderType) {
    const ref = useRef<HTMLInputElement>(null);
    const [currentValue, setValue] = useState(getNumber(props.value) ?? min);

    const updatePct = useCallback(
        (value: number) => {
            const pct = ((value - min) / (max - min)) * 100;
            ref.current?.style.setProperty("--range-pct", `${pct}%`);
        },
        [min, max],
    );

    // Sync the external value changes with the internal state.
    useEffect(() => {
        const newValue = getNumber(props.value) ?? min;
        setValue(() => newValue);
        updatePct(newValue);
    }, [updatePct, props.value, min]);

    // Listen for slider input event to update the value and percentage.
    useLayoutEffect(() => {
        const slider = ref.current;
        if (!slider) return;

        const handleInputEvent = (e: Event) => {
            const target = e.target as HTMLInputElement;
            setValue(() => target.valueAsNumber);
            updatePct(target.valueAsNumber);
        };

        slider.addEventListener("input", handleInputEvent);
        // Initialize value percentage on mount.
        updatePct(slider.valueAsNumber);

        return () => {
            slider.removeEventListener("input", handleInputEvent);
        };
    }, [updatePct]);

    const input_class = classnames([style.input_range_style, props.className]);
    const container = classnames([style.container]);

    return (
        <div className={container}>
            <Markers ref={ref} />
            <input
                className={input_class}
                ref={ref}
                type="range"
                value={currentValue}
                min={min}
                max={max}
                step={props.step}
                onChange={props.onChange}
            />
            <span>{parseFloat(currentValue.toFixed(2))}</span>
        </div>
    );
}

export function VSlider({ className, ...props }: SliderType) {
    return <Slider {...props} className={classnames([style.vslider, className])} />;
}
