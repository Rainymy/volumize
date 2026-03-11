import { useEffect } from "react";
import { volumeController } from "$bridge/volumeManager";
import { useLogout } from "$hook/useLogout";
import { HEARTBEAT } from "$type/constant";

export function Heartbeat() {
    const logout = useLogout();

    useEffect(() => {
        let retryCount = 0;

        const interval_id = setInterval(async () => {
            const heartbeat = await volumeController.heartbeat();
            // const heartbeat = Math.random() < 0.1;

            console.log("Received heartbeat:", heartbeat);
            retryCount = heartbeat ? 0 : retryCount + 1;

            if (!heartbeat) {
                console.log(
                    "Heartbeat failure, retries left:",
                    HEARTBEAT.MAX_RETRY_COUNT - retryCount,
                );
            }
            if (retryCount >= HEARTBEAT.MAX_RETRY_COUNT) {
                console.log("Max retries reached, logging out...");
                clearInterval(interval_id);
                await logout();
            }
        }, HEARTBEAT.CHECK_DELAY_MS);

        return () => {
            clearInterval(interval_id);
        };
    }, [logout]);

    return null;
}
