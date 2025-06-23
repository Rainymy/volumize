export type AppIdentifier = string;
export type VolumePercent = number & { __brand: "VolumePercent" }

export enum SessionType {
  Application = "Application",
  Device = "Device"
}

export enum SessionDirection {
  Render = "Render",
  Capture = "Capture",
  NOOP = "Noop"
}

export interface AudioSession {
  name: string;
  type: SessionType;
  direction: SessionDirection;
  deviceOutput: SessionDirection;
  deviceName: string;
  id: string;
  windowTitle: string;
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