import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

const tauriController = new TauriVolumeController();
const websocketController = new WebsocketTauriVolumeController();

export const volumeController = is_desktop() ? tauriController : websocketController;
