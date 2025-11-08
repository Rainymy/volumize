import { useAtomValue } from "jotai";
import { useEffect } from "react";
import { IoClose } from "react-icons/io5";
import { useNavigate } from "react-router-dom";

import { AppLogo } from "$base/appLogo";
import { AppButton } from "$base/button";
import { ServerURLComponent } from "$component/serverInput";
import { useConnect } from "$hook/useWebsocket";
import { connection_ready } from "$model/volume";
import { NavigationType } from "$type/navigation";

import style from "./index.module.less";

export function Entry() {
    const navigate = useNavigate();
    const [isLoading, setIsLoading] = useConnect();

    const is_ready = useAtomValue(connection_ready);

    useEffect(() => {
        if (is_ready) {
            navigate(NavigationType.MAIN);
        }
    }, [is_ready, navigate]);

    return (
        <div className={style.box}>
            <AppLogo />
            {isLoading ? (
                <ServerDiscoveryLoading cancel={() => setIsLoading(false)} />
            ) : (
                <ServerURLComponent />
            )}
        </div>
    );
}

function ServerDiscoveryLoading(props: { cancel: () => void }) {
    return (
        <div>
            <h2>Server discovery in progress...</h2>
            <AppButton onClick={() => props.cancel()}>
                <IoClose /> Cancel
            </AppButton>
        </div>
    );
}
