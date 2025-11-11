import { atom } from "jotai";
import { atomWithRefresh } from "jotai/utils";
import { is_desktop } from "$bridge/generic";
import { volumeController } from "$bridge/volumeManager";
import type { AudioApplication } from "$type/volume";

export const connection_ready = atom(is_desktop());

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
