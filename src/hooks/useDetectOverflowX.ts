import { useLayoutEffect, useRef, useState } from "react";

type DetectOverflow<T> = {
    isOverflowing: boolean;
    amount: number;
    ratio: number;
    ref: React.RefObject<T | null>;
};

export function useDetectOverflowX<T extends HTMLElement>(
    externRef?: React.RefObject<T | null>,
): DetectOverflow<T> {
    const __intern_ref = useRef<T>(null);
    const ref = externRef ?? __intern_ref;

    const [isOverflowing, setIsOverflowing] = useState(false);
    const [overflowAmount, setOverflowAmount] = useState(0);
    const [overflowRatio, setOverflowRatio] = useState(0);

    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            return;
        }

        function evaluateOverflow(element: T | Element) {
            const overflow = calculateOverflow(element.scrollWidth, element.clientWidth);

            setOverflowAmount(Math.max(0, overflow.overflowAmount));
            setIsOverflowing(overflow.overflowAmount > 0);
            setOverflowRatio(overflow.overflowRatio);
        }

        evaluateOverflow(span);

        const resizeObserver = new ResizeObserver((entries) => {
            evaluateOverflow(entries[0].target);
        });

        resizeObserver.observe(span);

        return () => {
            resizeObserver.unobserve(span);
        };
    }, [ref.current]);

    return {
        isOverflowing: isOverflowing,
        amount: overflowAmount,
        ratio: overflowRatio,
        ref: ref,
    };
}

export function useDetectOverflowY<T extends HTMLElement>(
    externRef?: React.RefObject<T | null>,
): DetectOverflow<T> {
    const __intern_ref = useRef<T>(null);
    const ref = externRef ?? __intern_ref;

    const [isOverflowing, setIsOverflowing] = useState(false);
    const [overflowAmount, setOverflowAmount] = useState(0);
    const [overflowRatio, setOverflowRatio] = useState(0);

    useLayoutEffect(() => {
        const span = ref.current;
        if (!span) {
            console.warn(`${useDetectOverflowX.name}: ref is null: ${span}`);
            return;
        }

        function evaluateOverflow(element: T | Element) {
            const overflow = calculateOverflow(
                element.scrollHeight,
                element.clientHeight,
            );

            setOverflowAmount(Math.max(0, overflow.overflowAmount));
            setIsOverflowing(overflow.overflowAmount > 0);
            setOverflowRatio(overflow.overflowRatio);
        }

        evaluateOverflow(span);

        const resizeObserver = new ResizeObserver((entries) => {
            evaluateOverflow(entries[0].target);
        });

        resizeObserver.observe(span);

        return () => {
            resizeObserver.unobserve(span);
        };
    }, [ref.current]);

    return {
        isOverflowing: isOverflowing,
        amount: overflowAmount,
        ratio: overflowRatio,
        ref: ref,
    };
}

function calculateOverflow(scrollSize: number, elementSize: number) {
    return {
        overflowAmount: scrollSize - elementSize,
        overflowRatio: scrollSize / elementSize,
    };
}
