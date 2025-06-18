export type PlaybackDevice = string;
export type AppIdentifier = string;

export type VolumePercent = number & { __brand: "VolumePercent" }

export type SessionType = "Application" | "Device";

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
