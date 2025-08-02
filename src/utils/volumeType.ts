export type AppIdentifier = string;
export type VolumePercent = number & { __brand: "VolumePercent" };

export enum SessionType {
    Application = "Application",
    System = "System",
    Unknown = "Unknown"
}

export enum SessionDirection {
    Render = "Render",
    Capture = "Capture",
    Unknown = "Unknown"
}

export interface ProcessInfo {
    id: number;
    name: string;
    path: string | null;
}

export interface AudioVolume {
    current: VolumePercent;
    muted: boolean;
}

export interface AudioApplication {
    process: ProcessInfo;
    session_type: SessionType;
    direction: SessionDirection;
    volume: AudioVolume;
    sound_playing: boolean;
}

export interface AudioDevice {
    id: string;
    name: string;
    direction: SessionDirection;
}

export interface AudioSession {
    device: AudioDevice;
    applications: AudioApplication[];
}

export abstract class VolumeController {
    // Master Volume
    abstract getMasterVolume(): Promise<VolumePercent | null>;
    abstract setMasterVolume(percent: VolumePercent): Promise<void>;
    abstract muteMaster(): Promise<void>;
    abstract unmuteMaster(): Promise<void>;

    // Applications
    abstract getAllApplications(): Promise<AudioSession[]>;
    abstract getAppVolume(app: AppIdentifier): Promise<VolumePercent>;
    abstract setAppVolume(app: AppIdentifier, percent: VolumePercent): Promise<void>;
    abstract muteApp(app: AppIdentifier): Promise<void>;
    abstract unmuteApp(app: AppIdentifier): Promise<void>;

    // Devices
    abstract getPlaybackDevices(): Promise<AudioDevice[]>;
    abstract getCurrentPlaybackDevice(): Promise<AudioDevice | null>;
}