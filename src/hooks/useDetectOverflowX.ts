import { useLayoutEffect, useState } from "react";

type DetectOverflow = {
    overflowAmount: number;
    isOverflowing: boolean;
    overflowRatio: number;
};

/**
 * @param ref
 * @param overflowPadding -
 *      The padding percentage to use for overflow detection. 1.1 by default.
 * @returns
 */
export function useDetectOverflowX<T extends HTMLElement>(
    ref: React.RefObject<T | null>,
): DetectOverflow {
    const [isOverflowing, setIsOverflowing] = useState(false);
    const [overflowAmount, setOverflowAmount] = useState(0);
    const [overflowRatio, setOverflowRatio] = useState(0);

    // useLayoutEffect(() => {}, []);
    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            console.warn(`${useDetectOverflowX.name}: ref is null: ${span}`);
            return;
        }

        const overflowAmountX = span.scrollWidth - span.clientWidth;
        const overflowRatio = span.scrollWidth / span.clientWidth;

        setOverflowAmount(Math.max(0, overflowAmountX));
        setIsOverflowing(overflowAmountX > 0);
        setOverflowRatio(overflowRatio);
    }, [ref.current]);

    return {
        isOverflowing: isOverflowing,
        overflowAmount: overflowAmount,
        overflowRatio: overflowRatio,
    };
}

export function useDetectOverflowY<T extends HTMLElement>(
    ref: React.RefObject<T | null>,
): DetectOverflow {
    const [isOverflowing, setIsOverflowing] = useState(false);
    const [overflowAmount, setOverflowAmount] = useState(0);
    const [overflowRatio, setOverflowRatio] = useState(0);

    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            console.warn(`${useDetectOverflowX.name}: ref is null: ${span}`);
            return;
        }

        const overflowAmountX = span.scrollHeight - span.clientHeight;
        const overflowRatio = span.scrollHeight / span.clientHeight;

        setOverflowAmount(Math.max(0, overflowAmountX));
        setIsOverflowing(overflowAmountX > 0);
        setOverflowRatio(overflowRatio);
    }, [ref.current]);

    return {
        isOverflowing: isOverflowing,
        overflowAmount: overflowAmount,
        overflowRatio: overflowRatio,
    };
}
