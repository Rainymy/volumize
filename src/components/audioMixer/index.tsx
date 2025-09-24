import { useAtom } from "jotai";
import { useEffect } from "react";
import { volumeController } from "$bridge/volumeManager";
import { WebsocketTauriVolumeController } from "$bridge/websocket_volume";
import { MainContent } from "$component/mainContent";
import { Sidebar } from "$component/sidebar";
import { connection_ready, server_port, server_url } from "$model/volume";
import wrapper from "./index.module.less";

export function AudioMixer() {
    const [connect_url, set_connect_url] = useAtom(server_url);
    const [connect_port, set_connect_port] = useAtom(server_port);
    const [is_ready, set_is_ready] = useAtom(connection_ready);

    useEffect(() => {
        (async () => {
            if (volumeController instanceof WebsocketTauriVolumeController) {
                await volumeController.setup(connect_url, connect_port);
                set_is_ready(true);
            }
        })()

        return () => {
            if (volumeController instanceof WebsocketTauriVolumeController) {
                volumeController.close();
            }
            set_is_ready(false);
        };
    }, [connect_url, connect_port, set_is_ready]);

    return (
        <div className={wrapper.container}>
            {
                !is_ready && <div>In put URL PlEASE</div>
            }
            <Sidebar />
            <MainContent />
        </div>
    );
}
