import { atom } from "jotai";
import { atomWithRefresh } from "jotai/utils";
import { is_desktop } from "$bridge/generic";
import { volumeController } from "$bridge/volumeManager";

export const connection_ready = atom(is_desktop());
export const server_url = atom("localhost"); // 192.168.1.115
export const server_port = atom(9001);

export const selected_device_id = atom<string>();

let index = 0;
export const audio_session = atomWithRefresh(async (get) => {
    console.log("[audio_session] fetch count: ", index++);

    if (!get(connection_ready)) {
        console.log(
            "[audio_session] connection is not ready: ",
            get(connection_ready),
        );
        return [];
    }

    return await volumeController.getAllApplications();
});
