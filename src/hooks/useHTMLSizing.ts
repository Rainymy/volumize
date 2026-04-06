import { type RefObject, useEffect, useState } from "react";

export function useHTMLSizing<T extends HTMLElement>(ref: RefObject<T | null>) {
    const [rect, setRect] = useState<DOMRectReadOnly>(DOMRectReadOnly.fromRect());

    useEffect(() => {
        const target = ref?.current;
        if (!target) return;

        const observer = new ResizeObserver(([entry]) => {
            setRect(() => entry.contentRect);
        });

        observer.observe(target);
        return () => observer.disconnect();
    }, [ref]);

    return rect;
}
