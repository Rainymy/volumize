import { useCallback } from "react";
import { useGetServerURL, useTryConnect } from "$component/serverInput";

export function useStartConnection() {
    const tryConnect = useTryConnect();
    const getServerURL = useGetServerURL();

    return useCallback(
        async (manual_mode: boolean) => {
            const url = await getServerURL(manual_mode);
            if (!url) {
                return;
            }

            await tryConnect(url.url, url.port);
        },
        [getServerURL, tryConnect],
    );
}
