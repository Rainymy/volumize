import { useEffect, useState } from "react";
import { classnames } from "$util/react";

import wrapper from "./index.module.less";

type SliderType = {
    className?: string;
    value?: string | number;
    min?: number;
    max?: number;
    step?: number;
    onChange?: React.ChangeEventHandler<HTMLInputElement> | undefined;
};

export function VSlider(props: SliderType) {
    return (
        <Slider
            {...props}
            className={classnames([props.className, wrapper.vslider])}
        ></Slider>
    );
}

export function Slider(props: SliderType) {
    const combineClass = [props.className, wrapper.slider_input];
    const [currentValue, setValue] = useState(Number(props.value));

    useEffect(() => {
        setValue(Number(props.value));
    }, [props.value]);

    return (
        <div className={wrapper.container}>
            <input
                type="range"
                min={props.min}
                max={props.max}
                step={props.step}
                value={currentValue}
                onChange={(event) => {
                    setValue(event.target.valueAsNumber);
                    props.onChange?.(event);
                }}
                className={classnames(combineClass)}
            ></input>
            {parseFloat(currentValue.toFixed(2))}
        </div>
    );
}
