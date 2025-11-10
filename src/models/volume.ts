import { atom } from "jotai";
import { is_desktop } from "$bridge/generic";

export const connection_ready = atom(is_desktop());

export const selected_device_id = atom<string>();
