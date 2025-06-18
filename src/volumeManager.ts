import os from "node:os";

import { WindowsVolumeController } from "./controllers/windows";
import { LinuxVolumeController } from "./controllers/linux";
import type { VolumeController } from "./volumeController";

export function getVolumeManager(): VolumeController | null {
  const platform = os.platform();

  if (platform === "win32") {
    return new WindowsVolumeController();
  }

  if (platform === "linux") {
    return new LinuxVolumeController();
  }

  if (platform === "darwin") {
    return null;
  }

  return null;
}