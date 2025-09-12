export type AppIdentifier = number;
export type VolumePercent = number & { __brand: "VolumePercent" };

export enum SessionType {
    Application = "Application",
    Device = "Device",
    System = "System",
    Unknown = "Unknown",
}

export enum SessionDirection {
    Render = "Render",
    Capture = "Capture",
    Unknown = "Unknown",
}

export interface ProcessInfo {
    id: AppIdentifier;
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
    friendly_name: string;
    direction: SessionDirection;
    is_default: boolean;
    volume: AudioVolume
}

export interface AudioSession {
    device: AudioDevice;
    applications: AudioApplication[];
}
