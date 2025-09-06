import { type ComponentPropsWithoutRef, useId, useState } from "react";
import { getNumber } from "$util/generic";
import { classnames } from "$util/react";

import wrapper from "./index.module.less";

export function VSlider(props: ComponentPropsWithoutRef<"input">) {
    const combinbe = [props.className, wrapper.vslider];

    return <Slider {...props} className={classnames(combinbe)}></Slider>;
}

export function Slider(props: ComponentPropsWithoutRef<"input">) {
    const id = useId();

    const combineClass = [props.className, wrapper.slider_input];
    const startingValue = props.defaultValue ?? props.value;
    const [value, setValue] = useState(startingValue);

    return (
        <div className={wrapper.container}>
            <input
                type="range"
                id={id}
                {...props}
                onChange={(value) => {
                    props.onChange?.(value);
                    setValue(value.target.value);
                }}
                className={classnames(combineClass)}
            ></input>
            {parseFloat(getNumber(value)?.toFixed(2) ?? "")}
        </div>
    );
}
