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

  console.log(await volumeManager.getPlaybackDevices());

  // const masterVolume = await volumeManager.getMasterVolume();
  // console.log(masterVolume);

}

main();