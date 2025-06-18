export type PlaybackDevice = string;
export type AppIdentifier = string;

export type VolumePercent = number & { __brand: "VolumePercent" }

export interface IVolumeController {
  getPlaybackDevices(): Promise<PlaybackDevice[]>;
  getCurrentPlaybackDevice(): Promise<PlaybackDevice>;
  getMasterVolume(): Promise<number | null>;
  setMasterVolume(percent: VolumePercent): Promise<void>;
  muteMaster(): Promise<void>;
  unmuteMaster(): Promise<void>;
  setAppVolume(app: AppIdentifier, percent: VolumePercent): Promise<void>;
  muteApp(app: AppIdentifier): Promise<void>;
  unmuteApp(app: AppIdentifier): Promise<void>;
}