import os from "node:os";

import { WindowsVolumeController } from "./controllers/windows";
import { LinuxVolumeController } from "./controllers/linux";

export type AvailableControllers = NonNullable<
  ReturnType<typeof getVolumeManager>
>;

export function getVolumeManager() {
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