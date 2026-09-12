import { UPDATE_EVENT_NAME, type UpdateChange } from "$type/bindings";
import { UPDATE_CENTER_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

export const volumeController = is_desktop()
    ? new TauriVolumeController()
    : new WebsocketTauriVolumeController();

// ============== SETUP UPDATE EVENT LISTENER ==============
type TEventData = {
    event: typeof UPDATE_EVENT_NAME;
    id: number;
    payload: UpdateChange;
};

// Centralize event listener for audio/state changes. To easily propagate changes.
function central_update_handler(event: EventType<TEventData>) {
    // console.log("central_update_handler", event);
    if (!event.detail) {
        throw new Error("Event detail is undefined", event.detail);
    }
    const data = new CustomEvent(UPDATE_EVENT_NAME, { detail: event.detail.payload });
    document.body.dispatchEvent(data);
}

// ================ REGISTER EVENT LISTENER ================
document.body.addEventListener(UPDATE_CENTER_EVENT, central_update_handler);

// ================= CLEANUP EVENT LISTENER ================
window.addEventListener("beforeunload", cleanup); // When user refreshes the page.

if (import.meta.hot) {
    function hot_reload() {
        cleanup();
        import.meta.hot?.off("vite:beforeUpdate", hot_reload);
    }
    import.meta.hot.on("vite:beforeUpdate", hot_reload);
}

function cleanup() {
    document.body.removeEventListener(UPDATE_CENTER_EVENT, central_update_handler);
    window.removeEventListener("beforeunload", cleanup);
    volumeController.close();
}
