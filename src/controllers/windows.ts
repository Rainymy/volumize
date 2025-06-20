import path from "node:path";
import os from "node:os";
import fs from "node:fs/promises";

import { spawnExecFile } from "../shell";
import { logMessage, LOG_TYPE } from "../log";

import {
  SessionTypeEnum,
  VolumeController,
  type VolumePercent,
  type AppIdentifier,
  type AudioSession,
  SessionDirectionEnum
} from "../volumeController";

import SoundView64 from "../../bin/SoundVolumeView-64x.exe";
import SoundView32 from "../../bin/SoundVolumeView-32x.exe";
import { convertIntoSession, type ISoundViewSession } from "../utils/windows";

export class WindowsVolumeController extends VolumeController {
  readonly svvPath: string;
  readonly soundTempFile: string;

  private audioSessions: AudioSession[] = [];

  constructor() {
    super();
    const svvPath = os.arch() === "x64" ? SoundView64 : SoundView32;
    this.svvPath = path.join(__dirname, svvPath);
    this.soundTempFile = path.join(__dirname, "./temp.json");
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

  filterSessoinsBy(type: SessionTypeEnum) {
    return this.audioSessions.filter(item => {
      // make sure to filter device. (audio playable)
      const direction = item.direction === SessionDirectionEnum.Render;
      return item.type === type && direction && item.active;
    });
  }

  async loadSessions() {
    const output = await this._exec(["/sjson", this.soundTempFile]);
    if (output === null) return [];

    const content = await fs.readFile(this.soundTempFile);
    this.audioSessions = convertIntoSession(parseTempFile(content));
    return this.audioSessions;
  }

  async getAllApplications() {
    return this.filterSessoinsBy(SessionTypeEnum.Application);
  }

  async getPlaybackDevices() {
    return this.filterSessoinsBy(SessionTypeEnum.Device);
  }

  async getCurrentPlaybackDevice() {
    for (const session of this.audioSessions) {
      if (session.active) {
        return session;
      }
    }

    return null;
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

function parseTempFile(content: Buffer): ISoundViewSession[] {
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
  }
  return [];
}