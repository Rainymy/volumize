import { useDetectOverflowX } from "react-detect-overflow";
import { classnames } from "$util/react";

import style from "./index.module.less";

type TitleProps = {
    title: string;
    className?: string;
    animate?: boolean;
    animationDuration?: number;
};

export function BouncyTitle({
    title,
    className,
    animate = true,
    animationDuration = 3,
}: TitleProps) {
    const { isOverflowing, amount, ratio, ref } = useDetectOverflowX<HTMLDivElement>();

    const textclass = classnames([
        style.text_content,
        isOverflowing && animate ? style.bounce : null,
    ]);

    return (
        <div ref={ref} title={title} className={classnames([style.title, className])}>
            <span
                className={textclass}
                data-overflow={amount}
                data-animation-duration={animationDuration * ratio}
            >
                {title}
            </span>
        </div>
    );
}
