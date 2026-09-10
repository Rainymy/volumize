export type AppIdentifier = number;
export type DeviceIdentifier = string;
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
