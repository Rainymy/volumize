import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { useCallback, useEffect, useRef, useState } from "react";
import { FaSearch } from "react-icons/fa";

import { AppButton } from "$base/button";
import { AppInput } from "$base/input";
import { volumeController } from "$bridge/volumeManager";
import { useStartConnection } from "$hook/useWebsocket";
import { server_port, server_url } from "$model/server_url";
import { connection_state as connection_status } from "$model/volume";
import { PORT } from "$type/constant";
import { ConnectionState } from "$type/navigation";
import { getNumber, isSocketController } from "$util/generic";
import style from "./index.module.less";

export function ServerURLComponent() {
    const start = useStartConnection();
    // const discoverServer = useDiscoverServer();
    return (
        <div className={style.box}>
            <ServerInput start={() => start(true)} />
            <hr />
            <div className={style.discover_server}>
                <AppButton onClick={() => start(false)}>Discover Servers</AppButton>
            </div>
        </div>
    );
}

export function useTryConnect() {
    const is_cancel = useRef(false);
    const set_connection_ready = useSetAtom(connection_status);

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

export function useGetServerURL() {
    const [_status, set_connection_ready] = useAtom(connection_status);

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
            const info: Info = {
                url: connect_url,
                port: connect_port,
            };
            return info;
        },
        [connect_url, connect_port, set_connection_ready],
    );
}

function parseForm(form: FormData) {
    const url = form.get("url")?.toString();
    const port_number = getNumber(form.get("port")?.toString());

    if (!url) {
        return { data: null, error: "Invalid. URL address!" };
    }

    if (port_number === undefined) {
        return { data: null, error: "Invalid. PORT address!" };
    }

    // Check if port is within valid range; PORT.MIN < port < PORT.MAX.
    if (PORT.MIN >= port_number || port_number >= PORT.MAX) {
        return {
            data: null,
            error: `Invalid. PORT address range! ${PORT.MIN}-${PORT.MAX}`,
        };
    }

    return { data: { url, port: port_number }, error: null };
}

function ServerInput({ start }: { start: () => Promise<void> }) {
    const [connect_url, set_connect_url] = useAtom(server_url);
    const [connect_port, set_connect_port] = useAtom(server_port);

    const [errorText, setErrorText] = useState("");

    async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);

        const { error, data } = parseForm(formData);

        if (error || !data) {
            setErrorText(error);
            return;
        }

        const { url, port } = data;

        set_connect_url(url);
        set_connect_port(port);
        await start();
    }

    return (
        <div className={style.server_input}>
            <form className={style.form} onSubmit={handleSubmit}>
                <AppInput
                    name="url"
                    placeholder="Enter server address"
                    defaultValue={connect_url}
                />
                <AppInput
                    type="number"
                    min={PORT.MIN}
                    max={PORT.MAX}
                    name="port"
                    placeholder="9002"
                    defaultValue={connect_port}
                />
                <AppButton type="submit">
                    <FaSearch />
                </AppButton>
            </form>
            {errorText && <div style={{ color: "red" }}>{errorText}</div>}
        </div>
    );
}
