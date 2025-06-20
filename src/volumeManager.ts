import os from "node:os";

import { WindowsVolumeController } from "./controllers/windows";

export type AvailableControllers = NonNullable<
  ReturnType<typeof getVolumeManager>
>;

export function getVolumeManager() {
  const platform = os.platform();

  if (platform === "win32") {
    return new WindowsVolumeController();
  }

  if (platform === "linux") {
    return null;
  }

  if (platform === "darwin") {
    return null;
  }

  return null;
}