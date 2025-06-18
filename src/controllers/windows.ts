import path from "node:path";
import os from "node:os";
import fs from "node:fs/promises";

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
  private soundTempFile: string = path.join(__dirname, "./temp.json");

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

  async listSessions() {
    const output = await this._exec(["/sjson", this.soundTempFile]);
    if (output === null) {
      return [];
    }

    const content = await fs.readFile(this.soundTempFile);
    const tempSession = this.parseTempFile(content);
    if (tempSession === null) {
      return [];
    }

    return this.convertIntoSession(tempSession);
  }

  parseTempFile(content: Buffer): ISoundViewSession[] | null {
    let text: string = "";
    try {
      // Needed for handling BOM (Byte Order Mark)
      const decoder = new TextDecoder("utf-16le", { fatal: true });
      text = decoder.decode(content);
      return JSON.parse(text);
    }
    catch (error) {
      // logging purpose only.
      (async () => {
        // write log in order
        await logMessage(LOG_TYPE.PARSE_OR_DECODING_ERROR, error);
        await logMessage(LOG_TYPE.EMPTY, text);
      })()
      return null;
    }
  }

  convertIntoSession(sessions: ISoundViewSession[]): AudioSession[] {
    const convertPercent = (value: string) => {
      return getNumber(value.substring(0, value.length - 1)) ?? 0;
    }

    return sessions.map(item => {
      const audioSessions: AudioSession = {
        name: item.Name,
        type: item.Type as SessionType,
        deviceName: item["Device Name"],
        id: item["Command-Line Friendly ID"],
        windowTitle: item["Window Title"],
        volume: convertPercent(item["Volume dB"]),
        volumePercent: convertPercent(item["Volume Percent"]),
        muted: item.Muted !== "No",
        active: item["Device State"] === "Active"
      }

      return audioSessions;
    });
  }

  async getPlaybackDevices() {
    return [] as PlaybackDevice[];
  }

  async getCurrentPlaybackDevice() {
    return "master" as PlaybackDevice;
  }

  async getMasterVolume() {
    const out = await this._exec(["/GetPercent", "\"DefaultRenderDevice\""]); // Doesn't work
    const match = (out ?? "").match(/Volume: (\d+)/);
    return match ? parseInt(match[0], 10) : null;
  }

  // Decrease the volume of Speakers device by 5% :
  // svcl.exe /ChangeVolume "Speakers" -5
  async setMasterVolume(percent: VolumePercent) {
    await this._exec(['/SetVolume', "DefaultRenderDevice", percent.toString()]);
  }

  async muteMaster() {
    await this._exec(["/Mute", "DefaultRenderDevice"]);
  }

  async unmuteMaster() {
    await this._exec(["/Unmute", "DefaultRenderDevice"]);
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