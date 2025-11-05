import { classnames } from "$util/react";
import style from "./index.module.less";

export function AppInput(props: React.InputHTMLAttributes<HTMLInputElement>) {
    const className = classnames([style.input, props.className]);

    return (
        <input
            type="text"
            {...props}
            className={className}
        />
    );
}
