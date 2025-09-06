import type { ButtonHTMLAttributes } from "react";
import { classnames } from "$util/react";
import style from "./index.module.less";

type AppButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
    is_active?: boolean;
};

export function AppButton(props: AppButtonProps) {
    const { is_active, className, children, ...rest } = props;

    const combineclass = classnames([
        style.button,
        className,
        is_active ? style.isActive : "",
    ]);

    return (
        <button type="button" className={combineclass} {...rest}>
            {children}
        </button>
    );
}
