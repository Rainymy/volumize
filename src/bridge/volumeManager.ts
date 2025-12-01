import { window as tauri_window } from "@tauri-apps/api";
import { listen, TauriEvent } from "@tauri-apps/api/event";

import { TAURI_UPDATE_EVENT, UPDATE_CENTER_EVENT, UPDATE_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import type { UpdateChange, UpdateEvent } from "$type/update";
import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

const tauriController = new TauriVolumeController();
const websocketController = new WebsocketTauriVolumeController();

export const volumeController = is_desktop() ? tauriController : websocketController;

type TEventData = {
    event: typeof UPDATE_EVENT;
    id: number;
    payload: UpdateChange;
};
/**
 * Centralize event listener for audio audio/state changes. To easily propagate changes.
 */
function central_update_handler(event: EventType<TEventData>) {
    if (!event.detail) {
        throw new Error("Event detail is undefined", event.detail);
    }
    const data = event.detail;
    document.body.dispatchEvent(new CustomEvent(UPDATE_EVENT, { detail: data.payload }));
}
document.body.addEventListener(UPDATE_CENTER_EVENT, central_update_handler);

// Tauri Listener.
const listener = await listen<UpdateEvent>(TAURI_UPDATE_EVENT, (event) => {
    document.body.dispatchEvent(new CustomEvent(UPDATE_CENTER_EVENT, { detail: event }));
});

/**
 * Detect webview window close event. Cleanup listeners and close window.
 */
const close_tauri_listener = await tauri_window
    .getCurrentWindow()
    .listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
        listener();
        close_tauri_listener();
        document.body.removeEventListener(UPDATE_CENTER_EVENT, central_update_handler);
        tauri_window.getCurrentWindow().destroy();
    });
