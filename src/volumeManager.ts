import os from "node:os";

import { WindowsVolumeController } from "./controllers/windows";
import { LinuxVolumeController } from "./controllers/linux";
import type { IVolumeController } from "./volumeController";

export function getVolumeManager(): IVolumeController | null {
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