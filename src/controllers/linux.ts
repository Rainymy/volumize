import { VolumeController } from "../volumeController";

import { spawnExec } from "../shell";
import { logMessage, LOG_TYPE } from "../log";

export class LinuxVolumeController extends VolumeController {
  async _exec(cmd: string): Promise<string | null> {
    const output = await spawnExec(cmd);

    if (typeof output === "string") {
      return output;
    }

    await logMessage(LOG_TYPE.EXEC_ERROR, output);
    console.log(output);

    return null;
  }

  async listSessions() {
    return []
  }

  async getPlaybackDevices() {
    return [];
  }

  async getCurrentPlaybackDevice() {
    return "master"
  }

  async getMasterVolume() {
    const out = await this._exec('pactl get-sink-volume @DEFAULT_SINK@');
    const match = (out ?? "").match(/Front.*?\/\s*(\d+)%/);
    return match ? parseInt(match[0], 10) : null;
  }

  async setMasterVolume(percent: number) {
    await this._exec(`pactl set-sink-volume @DEFAULT_SINK@ ${percent}%`);
  }

  async muteMaster() {
    await this._exec(`pactl set-sink-mute @DEFAULT_SINK@ 1`);
  }

  async unmuteMaster() {
    await this._exec(`pactl set-sink-mute @DEFAULT_SINK@ 0`);
  }

  async setAppVolume(app: string, percent: number) {
    const out = await this._exec('pactl list sink-inputs');
    const regex = new RegExp(`Sink Input #([0-9]+)[\\s\\S]*?application.name = "${app}"`, 'gi');
    const matches = [...(out ?? "").matchAll(regex)];
    if (matches.length === 0) throw new Error(`No sink-input found for app: ${app}`);
    for (const match of matches) {
      const index = match[1];
      await this._exec(`pactl set-sink-input-volume ${index} ${percent}%`);
    }
  }

  async muteApp(app: string) {
    const out = await this._exec('pactl list sink-inputs');
    const regex = new RegExp(`Sink Input #([0-9]+)[\\s\\S]*?application.name = "${app}"`, 'gi');
    const matches = [...(out ?? "").matchAll(regex)];
    if (matches.length === 0) throw new Error(`No sink-input found for app: ${app}`);
    for (const match of matches) {
      const index = match[1];
      await this._exec(`pactl set-sink-input-mute ${index} 1`);
    }
  }

  async unmuteApp(app: string) {
    const out = await this._exec('pactl list sink-inputs');
    const regex = new RegExp(`Sink Input #([0-9]+)[\\s\\S]*?application.name = "${app}"`, 'gi');
    const matches = [...(out ?? "").matchAll(regex)];
    if (matches.length === 0) throw new Error(`No sink-input found for app: ${app}`);
    for (const match of matches) {
      const index = match[1];
      await this._exec(`pactl set-sink-input-mute ${index} 0`);
    }
  }
}