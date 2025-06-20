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
  await volumeManager.loadSessions()

  const playbackDevices = await volumeManager.getPlaybackDevices();
  // const currentDevice = await volumeManager.getCurrentPlaybackDevice();
  // const applications = await volumeManager.getAllApplications();

  // console.log(
  //   playbackDevices.map(item => {
  //     return `${item.name} (${item.deviceName})`
  //   })
  // );
  console.log(playbackDevices);

  // console.log(applications);

  // const masterVolume = await volumeManager.getMasterVolume();
  // console.log(masterVolume);

}

main();