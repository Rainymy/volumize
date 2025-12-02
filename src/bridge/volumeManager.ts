import { window as tauri_window } from "@tauri-apps/api";
import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";

import { TAURI_UPDATE_EVENT, UPDATE_CENTER_EVENT, UPDATE_EVENT } from "$type/constant";
import type { EventType } from "$type/generic";
import type { UpdateChange, UpdateEvent } from "$type/update";
import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

const tauriController = new TauriVolumeController();
const websocketController = new WebsocketTauriVolumeController();

export const volumeController = is_desktop() ? tauriController : websocketController;

// ============== SETUP UPDATE EVENT LISTENER ==============
type TEventData = { event: typeof UPDATE_EVENT; id: number; payload: UpdateChange };

// Centralize event listener for audio audio/state changes. To easily propagate changes.
function central_update_handler(event: EventType<TEventData>) {
    if (!event.detail) {
        throw new Error("Event detail is undefined", event.detail);
    }
    console.log("central_update_handler", event.detail);
    const data = new CustomEvent(UPDATE_EVENT, { detail: event.detail.payload });
    document.body.dispatchEvent(data);
}

// ================ REGISTER EVENT LISTENER ================
// Websocket Listener
document.body.addEventListener(UPDATE_CENTER_EVENT, central_update_handler);

/**
 * ------- MOVE ALL TAURI EVENT INTO *tauriController* -------
 *  - as `setup` function.
 */

// Tauri Listener.
const listener = await listen<UpdateEvent>(
    TAURI_UPDATE_EVENT,
    (event) => {
        if (!is_desktop()) {
            // No need to handle this event on non-desktop.
            return;
        }
        console.log("listener", event);
        const data = new CustomEvent(UPDATE_CENTER_EVENT, { detail: event });
        document.body.dispatchEvent(data);
    },
    { target: { kind: "WebviewWindow", label: "main" } },
);

// Detect webview window close event. Cleanup listeners and close window.
const close_tauri_listener: UnlistenFn = await tauri_window
    .getCurrentWindow()
    .listen(TauriEvent.WINDOW_CLOSE_REQUESTED, async () => {
        cleanup();
        await tauri_window.getCurrentWindow().close();
    });

// When user refreshes the page.
window.addEventListener("beforeunload", cleanup);

// ================= CLEANUP EVENT LISTENER ================
function cleanup() {
    console.log("cleanup");
    listener();
    close_tauri_listener();
    document.body.removeEventListener(UPDATE_CENTER_EVENT, central_update_handler);
    window.removeEventListener("beforeunload", cleanup);
}
