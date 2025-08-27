import { atomWithRefresh } from "jotai/utils";
import { volumeController } from "../bridge/volumeManager";

export const audio_session = atomWithRefresh(async () => {
    return await volumeController.getAllApplications();
});
