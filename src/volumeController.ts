import type { EnumKeyFromValue } from "./utils/type";

export type PlaybackDevice = string;
export type AppIdentifier = string;

export type VolumePercent = number & { __brand: "VolumePercent" }

export enum SessionTypeEnum {
  Application,
  Device,
  Program
}

export type SessionType = Extract<keyof typeof SessionTypeEnum, string>;
export type GetSessionType<T extends SessionTypeEnum> = EnumKeyFromValue<typeof SessionTypeEnum, T>

export interface AudioSession {
  name: string;
  type: SessionType;
  deviceName: string;
  id: string;
  windowTitle: string;
  volume: number;
  volumePercent: number;
  muted: boolean;
  active: boolean;
}

export abstract class VolumeController {
  abstract getPlaybackDevices(): Promise<PlaybackDevice[]>;
  abstract getCurrentPlaybackDevice(): Promise<PlaybackDevice>;
  abstract getMasterVolume(): Promise<number | null>;
  abstract setMasterVolume(percent: VolumePercent): Promise<void>;
  abstract muteMaster(): Promise<void>;
  abstract unmuteMaster(): Promise<void>;
  abstract setAppVolume(app: AppIdentifier, percent: VolumePercent): Promise<void>;
  abstract muteApp(app: AppIdentifier): Promise<void>;
  abstract unmuteApp(app: AppIdentifier): Promise<void>;
  abstract listSessions(): Promise<AudioSession[]>;
}