import { useRef } from "react";
import { useDetectOverflowX } from "$hook/useDetectOverflowX";
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
    const ref = useRef<HTMLDivElement>(null);
    const { isOverflowing, overflowAmount, overflowRatio } = useDetectOverflowX(ref);

    const textclass = classnames([
        style.text_content,
        isOverflowing && animate ? style.bounce : null,
    ]);

    return (
        <div ref={ref} title={title} className={classnames([style.title, className])}>
            <span
                className={textclass}
                data-overflow={overflowAmount}
                data-animation-duration={animationDuration * overflowRatio}
            >
                {title}
            </span>
        </div>
    );
}
