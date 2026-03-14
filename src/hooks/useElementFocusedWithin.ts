import { type RefObject, useEffect, useRef, useState } from "react";

type elementFocusedWithin<T extends HTMLElement> = {
    isFocusedWithin: boolean;
    ref: RefObject<T | null>;
};

export function useElementFocusedWithin<T extends HTMLElement>(
    externalRef?: RefObject<T | null>,
): elementFocusedWithin<T> {
    const [isFocusedWithin, setIsFocusedWithin] = useState(false);

    const __internalRef = useRef<T>(null);
    const ref = externalRef ?? __internalRef;

    useEffect(() => {
        function handleFocusOut(event: PointerEvent) {
            const isInside = ref.current?.contains(event.target as Node);
            setIsFocusedWithin(isInside ?? false);
        }

        document.addEventListener("click", handleFocusOut);

        return () => {
            document.removeEventListener("click", handleFocusOut);
        };
    }, [ref]);

    return { isFocusedWithin: isFocusedWithin, ref: ref };
}
