import path from "node:path";
import os from "node:os";

import { spawnExecFile } from "../shell";
import { logMessage, LOG_TYPE } from "../log";

import type {
  IVolumeController,
  VolumePercent,
  PlaybackDevice,
  AppIdentifier
} from "../volumeController";

export class WindowsVolumeController implements IVolumeController {
  #svvPath: string = path.join(__dirname, this.getSVVPath());

  getSVVPath() {
    const arch = os.arch() === "x64" ? "x64" : "x32";
    return `./SoundVolumeView-${arch}.exe`;
  }

  async _exec(args: string[]): Promise<string | null> {
    const output = await spawnExecFile(this.#svvPath, args);

    if (typeof output === "string") {
      return output;
    }

    await logMessage(LOG_TYPE.EXEC_ERROR, output);
    console.log(output);

    return null;
  }

  async getPlaybackDevices() {
    return [] as PlaybackDevice[];
  }

  async getCurrentPlaybackDevice() {
    return "master" as PlaybackDevice;
  }

  async getMasterVolume() {
    const out = await this._exec(['/GetVolume', 'Master Volume']);
    const match = (out ?? "").match(/Volume: (\d+)/);
    return match ? parseInt(match[0], 10) : null;
  }

  async setMasterVolume(percent: VolumePercent) {
    await this._exec(['/SetVolume', 'Master Volume', percent.toString()]);
  }

  async muteMaster() {
    await this._exec(['/Mute', 'Master Volume']);
  }

  async unmuteMaster() {
    await this._exec(['/Unmute', 'Master Volume']);
  }

  async setAppVolume(app: AppIdentifier, percent: VolumePercent) {
    await this._exec(['/SetVolume', app, percent.toString()]);
  }

  async muteApp(app: AppIdentifier) {
    await this._exec(['/Mute', app]);
  }

  async unmuteApp(app: AppIdentifier) {
    await this._exec(['/Unmute', app]);
  }
}