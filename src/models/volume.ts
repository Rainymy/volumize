import { atom } from "jotai";
import { atomWithRefresh } from "jotai/utils";
import { is_desktop } from "$bridge/generic";
import { volumeController } from "$bridge/volumeManager";
import { getNumber } from "$util/generic";

export const connection_ready = atom(is_desktop());

// localStorage.clear();
const __SERVER_URL__ = "server_url" as const;
const __server_url__ = atom(
	localStorage.getItem(__SERVER_URL__) ?? "192.168.1.115",
); // 192.168.1.115
export const server_url = atom(
	(get) => get(__server_url__),
	(_, set, newValue: string) => {
		set(__server_url__, newValue);
		localStorage.setItem(__SERVER_URL__, newValue);
	},
);

const __SERVER_PORT__ = "server_port" as const;
const __server_port__ = atom(
	getNumber(localStorage.getItem(__SERVER_PORT__)) ?? 9001,
);
export const server_port = atom(
	(get) => get(__server_port__),
	(_, set, newValue: number) => {
		set(__server_port__, newValue);
		localStorage.setItem(__SERVER_PORT__, newValue.toString());
	},
);

export const selected_device_id = atom<string>();

let index = 0;
export const audio_session = atomWithRefresh(async (get) => {
	console.log("[audio_session] fetch count: ", index++);

	if (!get(connection_ready)) {
		console.log(
			"[audio_session] connection is not ready: ",
			get(connection_ready),
		);
		return [];
	}

	return await volumeController.getAllApplications();
});
