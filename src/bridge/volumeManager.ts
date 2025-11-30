import { listen } from "@tauri-apps/api/event";
import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

const tauriController = new TauriVolumeController();
const websocketController = new WebsocketTauriVolumeController();

export const volumeController = is_desktop() ? tauriController : websocketController;

// Attach event listeners for volume changes
//
// Make 2 different listener, 1 for device and 1 for application.
// All of them listens and only apply if the update id is their own.

const listener = await listen("update", (event) => {
    console.log(event);
});

setTimeout(() => {
    listener();
}, 5000);
