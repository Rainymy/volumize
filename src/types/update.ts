import type { ChangeType, Identifier } from "./bindings";
import { TAURI_UPDATE_EVENT } from "./constant";

export function isAppIdentifier(id: Identifier) {
    return id.type === "app";
}

export function isDeviceIdentifier(id: Identifier) {
    return id.type === "device";
}

export function isAudioVolumeChange(change: ChangeType) {
    return change.kind === "audioVolume";
}

export function isIconPathChange(change: ChangeType) {
    return change.kind === "iconPathChange";
}

export function isStateChange(change: ChangeType) {
    return change.kind === "stateChange";
}

export type UpdatePayload = { id: Identifier; change: ChangeType };
export type UpdateEvent = {
    event: typeof TAURI_UPDATE_EVENT;
    payload: UpdatePayload;
};

export type DataEvent = { type: string; data: object };
export type RequestAcceptedEvent = { type: string; data: "REQUEST ACCEPTED" };
export type ResponseEvent = {
    type: "CommandResponse";
    id: number;
    response: object;
};

export function isResponse(event: unknown): event is ResponseEvent {
    const data = event as ResponseEvent;

    const is_type = data.type === "CommandResponse";
    const is_id = typeof data.id === "number";
    const is_response = typeof data.response === "object";

    return is_type && is_id && is_response;
}

export function isDataEvent(event: unknown): event is DataEvent {
    const data = event as DataEvent;
    return typeof data.type === "string" && typeof data.data === "object";
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
