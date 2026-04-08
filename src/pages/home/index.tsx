import { useAtomValue } from "jotai";
import { useNavigate } from "react-router";

import { AppLogo } from "$base/appLogo";
import { ServerURLComponent } from "$component/serverInput";
import { ServerDiscoveryLoading } from "$component/serverLoading";
import { useAsyncSignalEffect } from "$hook/useAsyncSignalEffect";
import { connection_state } from "$model/volume";
import { ConnectionState, NavigationType } from "$type/navigation";

import style from "./index.module.less";

export default Entry;
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
