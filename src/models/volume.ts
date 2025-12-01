import { atom } from "jotai";

import { is_desktop } from "$bridge/generic";
import { ConnectionState } from "$type/navigation";
import type { AppIdentifier, AudioApplication, AudioDevice } from "$type/volume";

export const connection_state = atom(
    is_desktop() ? ConnectionState.CONNECTED : ConnectionState.DISCONNECTED,
);

export const selected_device_id = atom<string>();
export const device_list = atom<AudioDevice[]>([]);

export const application_list = atom<AudioApplication[]>([]);
export const application_ids = (() => {
    const __internal_atom__ = atom<AppIdentifier[]>([]);

    return atom(
        async (get) => get(__internal_atom__),
        (get, set, fb: (prev: AppIdentifier[]) => AppIdentifier[]) => {
            // deduplicate pid ids.
            const prev = new Set(fb(get(__internal_atom__)));
            const sorted = [...prev].toSorted((a, _b) => (a < 10 ? -1 : 0));
            set(__internal_atom__, sorted);
        },
    );
})();
