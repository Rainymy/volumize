import { getVolumeManager } from "./components/volumeManager";
import { logMessage, LOG_TYPE } from "./components/log";
import { sleep } from "./components/utils/generic";

async function main() {
  const volumeManager = getVolumeManager();

  // check for unsupported platforms
  if (volumeManager === null) {
    await logMessage(LOG_TYPE.UNSUPPORTED_PLATFORM);
    process.exit(1);
  }

  const sessions = await volumeManager.loadSessions();
  if (sessions.length === 0) {
    console.log(`[${volumeManager.loadSessions.name}] returned empty array!`)
  }

  // const currentDevice = await volumeManager.getCurrentPlaybackDevice();
  // console.log(currentDevice);

  // const masterVolume = await volumeManager.getMasterVolume();
  // console.log("masterVolume:", masterVolume);
}

(async () => {
  try {
    await main();
  }
  catch (error) {
    console.log(error)
  }

  while (true) {
    console.log("Sleeping");
    await sleep(1000);
  }
})()