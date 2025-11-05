import { useAtom, useAtomValue } from "jotai";
import { useEffect, useState } from "react";

import { AppButton } from "$base/button";
import { volumeController } from "$bridge/volumeManager";
import { WebsocketTauriVolumeController } from "$bridge/websocket_volume";

import { MainContent } from "$component/mainContent";
import { Sidebar } from "$component/sidebar";
import { connection_ready, server_port, server_url } from "$model/volume";
import { getNumber } from "$util/generic";

import wrapper from "./index.module.less";

export function AudioMixer() {
    const connect_url = useAtomValue(server_url);
    const connect_port = useAtomValue(server_port);
    const [is_ready, set_is_ready] = useAtom(connection_ready);

    useEffect(() => {
        let cancelled = false;
        (async () => {
            if (volumeController instanceof WebsocketTauriVolumeController) {
                await volumeController.setup(connect_url, connect_port);
                if (!cancelled) {
                    set_is_ready(true);
                }
            }
        })();
        return () => {
            cancelled = true;
            if (volumeController instanceof WebsocketTauriVolumeController) {
                volumeController.close();
                set_is_ready(false);
            }
        };
    }, [connect_url, connect_port, set_is_ready]);

    return (
        <div className={wrapper.container}>
            {!is_ready ? (
                <ServerURLComponent />
            ) : (
                <>
                    <Sidebar />
                    <MainContent />
                </>
            )}
        </div>
    );
}

function ServerURLComponent() {
    const [connect_url, set_connect_url] = useAtom(server_url);
    const [connect_port, set_connect_port] = useAtom(server_port);
    const [errorText, setErrorText] = useState("");

    function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);

        const url = formData.get("url")?.toString();
        const port = formData.get("port")?.toString();

        if (!url) {
            return setErrorText("Invalid. URL address!");
        }

        const port_number = getNumber(port);
        if (!port || port_number === undefined) {
            return setErrorText("Invalid. PORT address!");
        }
        const is_port_valid_range = 0 < port_number && port_number < 2 ** 16;
        if (!is_port_valid_range) {
            return setErrorText("Invalid. PORT address range!");
        }

        console.log("Setting new ULR and PORT address....");
        console.log("info: ", url, port);
        set_connect_url(url);
        set_connect_port(port_number);
    }

    return (
        <div style={{ margin: "auto" }}>
            <form onSubmit={handleSubmit}>
                <div>{errorText}</div>
                <input type="text" name="url" defaultValue={connect_url} />
                <input type="number" name="port" defaultValue={connect_port} />
                <AppButton type="submit">Connect</AppButton>
            </form>
        </div>
    );
}
