import { getEnumIncludes, getNumber, isEnumValue } from "./generic";

import {
  SessionType,
  SessionDirection,
  type AudioSession,
} from "../volumeController";

enum WindowsTypeEnum {
  Application = "Application",
  Device = "Device",
}

enum WindowsDirectionEnum {
  Capture = "Capture",
  Render = "Render",
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
  [WindowsTypeEnum.Application]: SessionType.Application,
  [WindowsTypeEnum.Device]: SessionType.Device,
}

const WindowsToDirectionTypeMap: Record<WindowsDirectionEnum, SessionDirection> = {
  [WindowsDirectionEnum.Render]: SessionDirection.Render,
  [WindowsDirectionEnum.Capture]: SessionDirection.Capture,
}

function convertPercent(value: string) {
  return getNumber(value.substring(0, value.length - 1)) ?? 0;
}

export function convertIntoSession(sessions: ISoundViewSession[]): AudioSession[] {
  return sessions.map(item => {
    const type = getEnumIncludes(WindowsTypeEnum, item.Type);
    const directionType = getEnumIncludes(WindowsDirectionEnum, item.Direction);
    if (type === null || directionType === null) {
      return null
    }

    const currentAudioDevice = isEnumValue(WindowsDirectionEnum, item.Default)
      ? WindowsToDirectionTypeMap[item.Default]
      : SessionDirection.NOOP

    const audioSessions: AudioSession = {
      name: item.Name,
      type: WindowsToSessionTypeMap[type],
      direction: WindowsToDirectionTypeMap[directionType],
      deviceOutput: currentAudioDevice,
      deviceName: item["Device Name"],
      id: item["Command-Line Friendly ID"],
      windowTitle: item["Window Title"],
      volumePercent: convertPercent(item["Volume Percent"]),
      muted: item.Muted !== "No",
      active: item["Device State"] === "Active"
    }

    return audioSessions;
  })
    .filter(item => item !== null);
}