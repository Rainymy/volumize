import { is_desktop } from "./generic";
import { TauriVolumeController } from "./tauri_volume";
import { WebsocketTauriVolumeController } from "./websocket_volume";

export const volumeController = is_desktop()
    ? new TauriVolumeController()
    : new WebsocketTauriVolumeController();
