import { useLayoutEffect, useState } from "react";

const OVERFLOW_PADDING = 1.1;
export function useDetectOverflowX<T extends HTMLElement>(
    ref: React.RefObject<T | null>,
) {
    const [isOverflowing, setIsOverflowing] = useState(false);

    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            console.warn(`${useDetectOverflowX.name}: ref is null: ${span}`);
            return;
        }

        setIsOverflowing(span.scrollWidth > span.clientWidth * OVERFLOW_PADDING);
    }, [ref.current]);

    return isOverflowing;
}

export function useDetectOverflowY<T extends HTMLElement>(
    ref: React.RefObject<T | null>,
) {
    const [isOverflowing, setIsOverflowing] = useState(false);

    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            console.warn(`${useDetectOverflowY.name}: ref is null: ${span}`);
            return;
        }

        setIsOverflowing(span.scrollHeight > span.clientHeight);
    }, [ref.current]);

    return [ref, isOverflowing];
}
