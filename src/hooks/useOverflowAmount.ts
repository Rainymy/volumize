import { type RefObject, useLayoutEffect, useState } from "react";

export function useOverflowAmountX<T extends HTMLElement>(ref: RefObject<T | null>) {
    const [overflowAmount, setOverflowAmount] = useState(0);

    useLayoutEffect(() => {
        const rect = ref.current;
        if (!rect) {
            console.warn(`${useOverflowAmountX.name}: ref is null: ${rect}`);
            return;
        }

        const overflowAmount = rect.clientWidth - rect.scrollWidth;

        setOverflowAmount(Math.min(0, overflowAmount));
    }, [ref.current]);

    return overflowAmount;
}
