import { type RefObject, useEffect, useState } from "react";

export function useElementFocusedWithin<T extends HTMLElement>(
    ref: RefObject<T | null>,
): boolean {
    const [isFocusedWithin, setIsFocusedWithin] = useState(false);

    useEffect(() => {
        function handleFocusOut(event: PointerEvent) {
            const isInside = ref.current?.contains(event.target as Node);
            setIsFocusedWithin(isInside ?? false);
        }

        document.addEventListener("click", handleFocusOut);

        return () => {
            document.removeEventListener("click", handleFocusOut);
        };
    }, [ref.current]);

    return isFocusedWithin;
}
