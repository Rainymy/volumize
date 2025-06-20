import { getNumber } from "../controllers/windows";
import { getEnumIncludes } from "./generic";

import {
  SessionTypeEnum,
  type AudioSession,
  type SessionType,
  type GetSessionType
} from "../volumeController";

export enum WindowsTypeEnum {
  Application = "Application",
  Device = "Device",
}

export interface ISoundViewSession {
  "Name": string;
  "Type": string;
  "Direction": string;
  "Device Name": string;
  "Default": string;
  "Default Multimedia": string;
  "Default Communications": string;
  "Device State": string;
  "Muted": string;
  "Volume dB": string;
  "Volume Percent": string;
  "Min Volume dB": string;
  "Max Volume dB": string;
  "Volume Step": string;
  "Channels Count": string;
  "Channels dB": string;
  "Channels Percent": string;
  "Item ID": string;
  "Command-Line Friendly ID": string;
  "Process Path": string;
  "Process ID": string;
  "Window Title": string;
  "Registry Key": string;
  "Speakers Config": string;
  "Default Format": string;
}

const WindowsToSessionTypeMap: Record<WindowsTypeEnum, SessionType> = {
  [WindowsTypeEnum.Application]: SessionTypeEnum[SessionTypeEnum.Application] as SessionType,
  [WindowsTypeEnum.Device]: SessionTypeEnum[SessionTypeEnum.Device] as SessionType,
}

function convertPercent(value: string) {
  return getNumber(value.substring(0, value.length - 1)) ?? 0;
}

export function convertIntoSession(sessions: ISoundViewSession[]): AudioSession[] {
  return sessions.map(item => {
    const type = getEnumIncludes(WindowsTypeEnum, item.Type);
    if (type === null) return null;

    const audioSessions: AudioSession = {
      name: item.Name,
      type: WindowsToSessionTypeMap[type],
      deviceName: item["Device Name"],
      id: item["Command-Line Friendly ID"],
      windowTitle: item["Window Title"],
      volume: convertPercent(item["Volume dB"]),
      volumePercent: convertPercent(item["Volume Percent"]),
      muted: item.Muted !== "No",
      active: item["Device State"] === "Active"
    }

    return audioSessions;
  })
    .filter(item => item !== null);
}