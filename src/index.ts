import { getVolumeManager } from "./volumeManager";
import { logMessage, LOG_TYPE } from "./log";
import { sleep } from "./utils/generic";

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

  // const playbackDevices = await volumeManager.getPlaybackDevices();
  // console.log(
  //   playbackDevices.map(item => {
  //     return `${item.name} (${item.deviceName})`
  //   })
  // );

  // const currentDevice = await volumeManager.getCurrentPlaybackDevice();
  // console.log(currentDevice);

  const masterVolume = await volumeManager.getMasterVolume();
  console.log("masterVolume:", masterVolume);

  // while (true) {
  //   console.log("Sleeping");
  //   await sleep(1000);
  // }
}

main();