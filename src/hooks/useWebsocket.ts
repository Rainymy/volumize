import { useAtomValue, useSetAtom } from "jotai";
import { useCallback, useEffect, useRef } from "react";

import { volumeController } from "$bridge/volumeManager";
import { server_port, server_url } from "$model/server_url";
import { connection_state } from "$model/volume";
import { ConnectionState } from "$type/navigation";
import { isSocketController } from "$util/generic";

export function useStartConnection() {
    const tryConnect = useTryConnect();
    const getServerURL = useGetServerURL();

    return useCallback(
        async (manual_mode: boolean) => {
            const url = await getServerURL(manual_mode);
            if (!url) {
                console.error("Failed to get server URL");
                return;
            }

            await tryConnect(url.url, url.port);
        },
        [getServerURL, tryConnect],
    );
}

function useTryConnect() {
    const is_cancel = useRef(false);
    const set_connection_ready = useSetAtom(connection_state);

    useEffect(() => {
        return () => {
            is_cancel.current = true;
        };
    }, []);

    return useCallback(
        async (url: string, port: number) => {
            if (!isSocketController(volumeController)) {
                return;
            }

            set_connection_ready(() => ConnectionState.LOADING);
            await volumeController.close();

            if (!is_cancel.current) {
                set_connection_ready(() => ConnectionState.DISCONNECTED);
                return;
            }

            await volumeController.setup(url, port);
            set_connection_ready(() => ConnectionState.CONNECTED);
        },
        [set_connection_ready],
    );
}

function useGetServerURL() {
    const set_connection_ready = useSetAtom(connection_state);

    const connect_url = useAtomValue(server_url);
    const connect_port = useAtomValue(server_port);

    type ReturnValue = ReturnType<typeof volumeController.discoverServer>;
    type Info = NonNullable<Awaited<ReturnValue>>;

    return useCallback(
        async (manual_input: boolean) => {
            console.log("Manual mode: ", manual_input);

            if (!manual_input) {
                set_connection_ready(() => ConnectionState.LOADING);
                console.log("Discover server from MDNS");
                const info = await volumeController.discoverServer();
                set_connection_ready(() => ConnectionState.DISCONNECTED);
                return info;
            }
            return {
                url: connect_url,
                port: connect_port,
            } satisfies Info;
        },
        [connect_url, connect_port, set_connection_ready],
    );
}
