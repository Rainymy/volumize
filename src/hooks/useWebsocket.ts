import { useAtomValue, useSetAtom } from "jotai";
import { useCallback, useEffect } from "react";

import { volumeController } from "$bridge/volumeManager";
import { server_port, server_url } from "$model/server_url";
import { connection_state } from "$model/volume";
import { CONNECTION_MODE, ConnectionState } from "$type/navigation";

export function useStartConnection() {
    const getServerURL = useGetServerURL();
    const set_connection_ready = useSetAtom(connection_state);

    useEffect(() => {
        return () => {
            // volumeController.close();
        };
    }, []);

    return useCallback(
        async (mode: CONNECTION_MODE) => {
            set_connection_ready(() => ConnectionState.LOADING);
            const url = await getServerURL(mode);
            if (!url) {
                console.error("Failed to get server URL");
                set_connection_ready(() => ConnectionState.DISCONNECTED);
                return;
            }

            await volumeController.close();
            await volumeController.setup(url.url, url.port);
            set_connection_ready(ConnectionState.CONNECTED);
        },
        [getServerURL, set_connection_ready],
    );
}

function useGetServerURL() {
    const connect_url = useAtomValue(server_url);
    const connect_port = useAtomValue(server_port);

    type ReturnValue = ReturnType<typeof volumeController.discoverServer>;
    type Info = NonNullable<Awaited<ReturnValue>>;

    return useCallback(
        async (manual_input: CONNECTION_MODE) => {
            console.log("Manual mode: ", manual_input);

            if (manual_input === CONNECTION_MODE.MANUAL) {
                return {
                    kind: "web",
                    url: connect_url,
                    port: connect_port,
                } satisfies Info;
            }

            return await volumeController.discoverServer();
        },
        [connect_url, connect_port],
    );
}
