import {
    type ChangeType,
    type CommandResponse,
    type Identifier,
    type Response,
    UPDATE_EVENT_NAME,
    type UpdateChange,
    type UpdateChangeEvent,
} from "./bindings";

export type RequestAcceptedEvent = Extract<Response, { type: "a_c_k" }>;
export type DataEvent = Exclude<Response, RequestAcceptedEvent>;

export function isResponse(event: unknown): event is CommandResponse {
    const data = event as CommandResponse;

    const is_type = data.type === "CommandResponse";
    const is_id = typeof data.id === "number";
    const is_response = typeof data.response.type === "string";

    return is_type && is_id && is_response;
}

export function isDataEvent(event: unknown): event is DataEvent {
    const data = event as DataEvent;
    return typeof data.type === "string" && typeof data.data === "object";
}

export function isRequestAcceptedEvent(event: unknown): event is RequestAcceptedEvent {
    const data = event as RequestAcceptedEvent;
    return data.type === "a_c_k";
}

// --------------------- UPDATE CHANGE ----------------------
export function isUpdateEvent(event: unknown): event is UpdateChangeEvent {
    const data = event as UpdateChangeEvent;
    return data.event === UPDATE_EVENT_NAME && isUpdatePayload(data.payload);
}

export function isUpdatePayload(payload: unknown): payload is UpdateChange {
    const data = payload as UpdateChange;

    const isUpdateChange = [
        isAudioVolumeChange(data.change),
        isIconPathChange(data.change),
        isStateChange(data.change),
        isNameChange(data.change),
    ];

    return isIdentifier(data.id) && isUpdateChange.some((a) => a === true);
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

export function isNameChange(change: ChangeType) {
    return change.kind === "nameChange";
}

// ----------------------- IDENTIFIER -----------------------
export function isIdentifier(data: unknown): data is Identifier {
    const data2 = data as Identifier;
    return isAppIdentifier(data2) || isDeviceIdentifier(data2);
}

export function isAppIdentifier(id: Identifier) {
    return id.type === "app" && typeof id.content === "number";
}

export function isDeviceIdentifier(id: Identifier) {
    return id.type === "device" && typeof id.content === "string";
}
