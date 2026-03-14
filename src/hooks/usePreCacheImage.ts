import { type ReactNode, useEffect, useState } from "react";

/**
 * @param src - The source URL of the image to pre-cache.
 * @returns Whether the image has been successfully pre-cached.
 */
export function usePreCacheImage(src: string | ReactNode) {
    const [isIconValid, setIsValid] = useState(false);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        if (typeof src !== "string") return;

        setIsLoading(true);

        const img = new Image();
        img.onload = () => {
            setIsValid(true);
            setIsLoading(false);
        };
        img.onerror = () => {
            setIsValid(false);
            setIsLoading(false);
        };

        setIsLoading(true);
        img.src = src;

        return () => {
            img.onload = null;
            img.onerror = null;
            img.remove();
        };
    }, [src]);

    return { isValid: isIconValid, isLoading };
}
