import { useAtomValue, useSetAtom } from "jotai";
import { IoClose } from "react-icons/io5";
import { useNavigate } from "react-router-dom";

import { AppLogo } from "$base/appLogo";
import { AppButton } from "$base/button";
import { ServerURLComponent } from "$component/serverInput";
import { useAsyncSignalEffect } from "$hook/useAsyncSignalEffect";
import { connection_state } from "$model/volume";
import { ConnectionState, NavigationType } from "$type/navigation";
import style from "./index.module.less";

export function Entry() {
    const navigate = useNavigate();

    const connect_state = useAtomValue(connection_state);
    const isLoading = connect_state === ConnectionState.LOADING;

    useAsyncSignalEffect(async () => {
        if (ConnectionState.CONNECTED === connect_state) {
            await navigate(NavigationType.MAIN);
        }
    }, [connect_state, navigate]);

    return (
        <div className={style.home_container}>
            <AppLogo />
            {isLoading ? <ServerDiscoveryLoading /> : <ServerURLComponent />}
        </div>
    );
}

function ServerDiscoveryLoading() {
    const set_connect_state = useSetAtom(connection_state);

    async function cancel() {
        set_connect_state(() => ConnectionState.DISCONNECTED);
    }

    return (
        <div>
            <h2>Server discovery in progress...</h2>
            <AppButton onClick={cancel}>
                <IoClose /> Cancel
            </AppButton>
        </div>
    );
}
