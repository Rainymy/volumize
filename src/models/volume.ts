import { atom } from "jotai";
import { atomWithRefresh } from "jotai/utils";
import { volumeController } from "$bridge/volumeManager";
import { ConnectionState } from "$type/navigation";
import type { AudioApplication } from "$type/volume";

// is_desktop() ? ConnectionState.CONNECTED : ConnectionState.DISCONNECTED;
export const connection_state = atom(ConnectionState.DISCONNECTED);

export const selected_device_id = atom<string>();
export const device_list = atomWithRefresh(async () => {
    return await volumeController.getPlaybackDevices();
});

export const application_ids = atomWithRefresh(async (get) => {
    const device_id = get(selected_device_id);
    if (!device_id) {
        return [];
    }

    const ids = await volumeController.getDeviceApplications(device_id);
    return ids.toSorted((a, _b) => {
        if (a < 10) return -1;
        return a;
    });
});
export const application_list = atom<AudioApplication[]>([]);
