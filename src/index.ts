import { getVolumeManager } from "./volumeManager";
import { logMessage, LOG_TYPE } from "./log";

async function main() {
  const volumeManager = getVolumeManager();

  // check for unsupported platforms
  if (volumeManager === null) {
    await logMessage(LOG_TYPE.UNSUPPORTED_PLATFORM);
    process.exit(1);
  }

  console.log("Updating all of the sessions.");
  const sessions = await volumeManager.loadSessions();
  if (sessions.length === 0) {
    console.log(`[${volumeManager.loadSessions.name}] returned empty array!`)
  }

  const playbackDevices = await volumeManager.getPlaybackDevices();
  const currentDevice = await volumeManager.getCurrentPlaybackDevice();
  // const applications = await volumeManager.getAllApplications();

  console.log(
    playbackDevices.map(item => {
      return `${item.name} (${item.deviceName})`
    })
  );
  console.log(currentDevice);

  // console.log(applications);

  // const masterVolume = await volumeManager.getMasterVolume();
  // console.log(masterVolume);

}

main();