import { useAtomValue, useSetAtom } from "jotai";
import { type Dispatch, type SetStateAction, useEffect, useState } from "react";

import { volumeController } from "$bridge/volumeManager";
import { WebsocketTauriVolumeController } from "$bridge/websocket_volume";
import { server_port, server_url } from "$model/server_url";
import { connection_ready } from "$model/volume";

export function useConnect(): [boolean, Dispatch<SetStateAction<boolean>>] {
    const [isLoading, setIsLoading] = useState(false);

    const connect_url = useAtomValue(server_url);
    const connect_port = useAtomValue(server_port);
    const set_is_ready = useSetAtom(connection_ready);

    useEffect(() => {
        let cancelled = false;
        (async () => {
            if (volumeController instanceof WebsocketTauriVolumeController) {
                setIsLoading(true);
                await volumeController.setup(connect_url, connect_port);
                if (!cancelled) {
                    set_is_ready(true);
                }
                setIsLoading(false);
            }
        })();
        return () => {
            cancelled = true;
            if (volumeController instanceof WebsocketTauriVolumeController) {
                volumeController.close();
                set_is_ready(false);
            }
            setIsLoading(false);
        };
    }, [connect_url, connect_port, set_is_ready]);

    return [isLoading, setIsLoading];
}
