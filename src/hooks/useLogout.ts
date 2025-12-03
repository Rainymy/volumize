import { useSetAtom } from "jotai";
import { useCallback } from "react";
import { useNavigate } from "react-router-dom";

import { volumeController } from "$bridge/volumeManager";
import { connection_state } from "$model/volume";
import { ConnectionState, NavigationType } from "$type/navigation";

export function useLogout() {
    const navigate = useNavigate();
    const connection = useSetAtom(connection_state);

    return useCallback(async () => {
        connection(() => ConnectionState.DISCONNECTED);
        await volumeController.close();
        await navigate(NavigationType.HOME);
    }, [navigate, connection]);
}
