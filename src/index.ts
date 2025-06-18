import { getVolumeManager } from "./volumeManager";
import { logMessage, LOG_TYPE } from "./log";

async function main() {
  const volumeManager = getVolumeManager();

  // check for unsupported platforms
  if (volumeManager === null) {
    await logMessage(LOG_TYPE.UNSUPPORTED_PLATFORM);
    process.exit(1);
  }

  volumeManager;
}

main();