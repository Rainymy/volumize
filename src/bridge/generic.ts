import { platform } from "@tauri-apps/plugin-os";

export function is_desktop() {
    switch (platform()) {
        case "android":
        case "ios":
            return false;
        default:
            return true;
    }
}
