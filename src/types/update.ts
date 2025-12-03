import { TAURI_UPDATE_EVENT } from "./constant";
import type { AppIdentifier, DeviceIdentifier, VolumePercent } from "./volume";

const appId = "app";
const deviceId = "device";

export type Identifier =
    | {
          type: typeof appId;
          content: AppIdentifier;
      }
    | {
          type: typeof deviceId;
          content: DeviceIdentifier;
      };

type IsAppIdentifier = Extract<Identifier, { type: typeof appId }>;
export function isAppIdentifier(id: Identifier): id is IsAppIdentifier {
    return id.type === appId;
}

type IsDeviceIdentifier = Extract<Identifier, { type: typeof deviceId }>;
export function isDeviceIdentifier(id: Identifier): id is IsDeviceIdentifier {
    return id.type === deviceId;
}

export type EntityType = "device" | "application";
export type EntityState = "disconnect" | "created";

const iconPathChange = "iconPathChange";
const audioVolume = "audioVolume";
const stateChange = "stateChange";

export type ChangeType =
    | {
          kind: typeof audioVolume;
          volume: VolumePercent;
          mute: boolean;
      }
    | {
          kind: typeof iconPathChange;
          path: string;
      }
    | {
          kind: typeof stateChange;
          state: EntityState;
      };

export interface UpdateChange {
    id: Identifier;
    change: ChangeType;
}

type isAudioVolumeChange = Extract<ChangeType, { kind: typeof audioVolume }>;
export function isAudioVolumeChange(change: ChangeType): change is isAudioVolumeChange {
    return change.kind === audioVolume;
}

type isIconPathChange = Extract<ChangeType, { kind: typeof iconPathChange }>;
export function isIconPathChange(change: ChangeType): change is isIconPathChange {
    return change.kind === iconPathChange;
}

type isStateChange = Extract<ChangeType, { kind: typeof stateChange }>;
export function isStateChange(change: ChangeType): change is isStateChange {
    return change.kind === stateChange;
}

export type UpdatePayload = { id: Identifier; change: ChangeType };
export type UpdateEvent = {
    event: typeof TAURI_UPDATE_EVENT;
    payload: UpdatePayload;
};

export type DataEvent = { type: string; data: object };
export type RequestAcceptedEvent = { type: string; data: "REQUEST ACCEPTED" };
export type ResponseEvent = { channel: string; data: object };

export function isDataEvent(event: unknown): event is DataEvent {
    const data = event as DataEvent;
    if (typeof data.type !== "string" || typeof data.data !== "object") {
        return false;
    }
    return true;
}

export function isUpdateEvent(event: unknown): event is UpdateEvent {
    const data = event as UpdateEvent;
    return data.event === TAURI_UPDATE_EVENT && isUpdatePayload(data.payload);
}

export function isRequestAcceptedEvent(event: unknown): event is RequestAcceptedEvent {
    const data = event as RequestAcceptedEvent;
    return typeof data.type === "string" && data.data === "REQUEST ACCEPTED";
}

export function isUpdatePayload(payload: unknown): payload is UpdatePayload {
    const data = payload as UpdatePayload;

    const isUpdateChange = [
        isAudioVolumeChange(data.change),
        isIconPathChange(data.change),
        isStateChange(data.change),
    ];

    return isIdentifier(data.id) && isUpdateChange.some((a) => a === true);
}
export function isIdentifier(data: unknown): data is Identifier {
    const data2 = data as Identifier;
    return isAppIdentifier(data2) || isDeviceIdentifier(data2);
}
