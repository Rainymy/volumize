import { invoke } from "@tauri-apps/api/core";
import type {
	AppIdentifier,
	AudioDevice,
	AudioSession,
	DeviceIdentifier,
	VolumePercent,
} from "$type/volume";
import { debounce } from "$util/generic";
import { isVolumePercent } from "$util/volume";
import { ATauriVolumeController, type ITauriVolumeController } from "./type";
import { BOUNCE_DELAY, RUST_INVOKE } from "./volumeManager";

export class TauriVolumeController
	extends ATauriVolumeController
	implements ITauriVolumeController
{
	getMasterVolume = debounce((device_id: DeviceIdentifier) => {
		return invoke<VolumePercent>(RUST_INVOKE.GET_DEVICE_VOLUME, {
			deviceId: device_id,
		});
	}, BOUNCE_DELAY.NORMAL);

	setMasterVolume = debounce(
		(device_id: DeviceIdentifier, percent: number) => {
			if (!isVolumePercent(percent)) {
				throw Error(`Invalid VolumePercent value: ${percent}`);
			}

			return invoke(RUST_INVOKE.SET_DEVICE_VOLUME, {
				deviceId: device_id,
				percent: percent,
			});
		},
		BOUNCE_DELAY.NORMAL,
	);

	muteMaster = debounce((device_id: DeviceIdentifier) => {
		return invoke(RUST_INVOKE.MUTE_DEVICE, { deviceId: device_id });
	}, BOUNCE_DELAY.NORMAL);

	unmuteMaster = debounce((device_id: DeviceIdentifier) => {
		return invoke(RUST_INVOKE.UNMUTE_DEVICE, { deviceId: device_id });
	}, BOUNCE_DELAY.NORMAL);

	/* =================== APPLICATIONS ===================== */
	getAllApplications = debounce(() => {
		return invoke<AudioSession[]>(RUST_INVOKE.GET_ALL_APPLICATIONS);
	}, BOUNCE_DELAY.FAST);

	getAppVolume = debounce((app: AppIdentifier) => {
		return invoke<VolumePercent>(RUST_INVOKE.GET_APP_VOLUME, {
			appIdentifier: app,
		});
	}, BOUNCE_DELAY.FAST);

	setAppVolume = debounce((app: AppIdentifier, percent: number) => {
		if (!isVolumePercent(percent)) {
			throw Error(`Invalid VolumePercent value: ${percent}`);
		}

		return invoke(RUST_INVOKE.SET_APP_VOLUME, {
			appIdentifier: app,
			volume: percent,
		});
	}, BOUNCE_DELAY.NORMAL);

	muteApp = debounce((app: AppIdentifier) => {
		return invoke(RUST_INVOKE.MUTE_APP_VOLUME, { appIdentifier: app });
	}, BOUNCE_DELAY.NORMAL);

	unmuteApp = debounce((app: AppIdentifier) => {
		return invoke(RUST_INVOKE.UNMUTE_APP_VOLUME, { appIdentifier: app });
	}, BOUNCE_DELAY.NORMAL);

	getPlaybackDevices = debounce(() => {
		return invoke<AudioDevice[]>(RUST_INVOKE.GET_PLAYBACK_DEVICES);
	}, BOUNCE_DELAY.FAST);
	getCurrentPlaybackDevice = debounce(() => {
		return invoke<AudioDevice | null>(
			RUST_INVOKE.GET_CURRENT_PLAYBACK_DEVICE,
		);
	}, BOUNCE_DELAY.FAST);
}
