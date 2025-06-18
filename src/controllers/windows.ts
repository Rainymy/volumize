import path from "node:path";
import os from "node:os";

import { spawnExecFile } from "../shell";
import { logMessage, LOG_TYPE } from "../log";

import {
  VolumeController,
  type VolumePercent,
  type PlaybackDevice,
  type AppIdentifier,
  type ISoundViewSession,
  type AudioSession,
  type SessionType
} from "../volumeController";

import SoundView64 from "../../bin/SoundVolumeView-64x.exe";
import SoundView32 from "../../bin/SoundVolumeView-32x.exe";

/**
* Parse `any` into `number`. Returns `undefined` if it fails.
*/
export function getNumber(num: unknown) {
  const number = Number(num);
  return Number.isInteger(number) ? number : undefined;
}

export class WindowsVolumeController extends VolumeController {
  private svvPath: string;
  constructor() {
    super();
    const svvPath = os.arch() === "x64" ? SoundView64 : SoundView32;
    this.svvPath = path.join(__dirname, svvPath);
  }

  async _exec(args: string[]): Promise<string | null> {
    const output = await spawnExecFile(this.svvPath, args);

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