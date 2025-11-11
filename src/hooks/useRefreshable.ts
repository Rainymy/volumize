import { useState } from "react";

export function useRefreshable(): [number, () => void] {
    const [token, setToken] = useState<number>(0);

    function refresh() {
        setToken((prev) => prev + 1);
    }

    return [token, refresh];
}
