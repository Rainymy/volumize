import { getVolumeManager } from "./volumeManager";
import { logMessage, LOG_TYPE } from "./log";

async function main() {
  const volumeManager = getVolumeManager();

  // check for unsupported platforms
  if (volumeManager === null) {
    await logMessage(LOG_TYPE.UNSUPPORTED_PLATFORM);
    process.exit(1);
  }

  console.log("Listing all of the sessions:");
  console.log(await volumeManager.loadSessions());

  // const masterVolume = await volumeManager.getMasterVolume();
  // console.log(masterVolume);

}

main();