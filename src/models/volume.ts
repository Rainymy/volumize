import { atom } from "jotai";
import { atomWithRefresh } from "jotai/utils";
import { volumeController } from "../bridge/volumeManager";

let index = 0;
export const audio_session = atomWithRefresh(async () => {
    console.log("[audio_session] fetch count: ", index++);
    return await volumeController.getAllApplications();
});

export const selected_device_id = atom<string>("");
