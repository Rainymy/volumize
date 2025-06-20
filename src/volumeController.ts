import type { EnumKeyFromValue } from "./utils/type";

export type AppIdentifier = string;
export type VolumePercent = number & { __brand: "VolumePercent" }

export enum SessionTypeEnum {
  Application = "Application",
  Device = "Device"
}

export enum SessionDirectionEnum {
  Render = "Render",
  Capture = "Capture"
}

type SessionType = Extract<keyof typeof SessionTypeEnum, string>;
type SessionDirectionType = Extract<keyof typeof SessionDirectionEnum, string>;
export type GetSessionType<T extends SessionTypeEnum> = EnumKeyFromValue<typeof SessionTypeEnum, T>

export interface AudioSession {
  name: string;
  type: SessionTypeEnum;
  direction: SessionDirectionEnum,
  deviceName: string;
  id: string;
  windowTitle: string;
  // volume: number;
  volumePercent: number;
  muted: boolean;
  active: boolean;
}

export abstract class VolumeController {
  abstract getPlaybackDevices(): Promise<AudioSession[]>;
  abstract getCurrentPlaybackDevice(): Promise<AudioSession | null>;
  abstract getMasterVolume(): Promise<number | null>;
  abstract setMasterVolume(percent: VolumePercent): Promise<void>;
  abstract muteMaster(): Promise<void>;
  abstract unmuteMaster(): Promise<void>;
  abstract setAppVolume(app: AppIdentifier, percent: VolumePercent): Promise<void>;
  abstract muteApp(app: AppIdentifier): Promise<void>;
  abstract unmuteApp(app: AppIdentifier): Promise<void>;
  abstract loadSessions(): Promise<AudioSession[]>;
  abstract getAllApplications(): Promise<AudioSession[]>;
}