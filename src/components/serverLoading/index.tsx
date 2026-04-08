import { useSetAtom } from "jotai";
import { useRef } from "react";
import { IoClose } from "react-icons/io5";
import { AppButton } from "$base/button";
import { useHTMLSizing } from "$hook/useHTMLSizing";
import { connection_state } from "$model/volume";
import { ConnectionState } from "$type/navigation";
import style from "./index.module.less";

export function ServerDiscoveryLoading() {
    const set_connect_state = useSetAtom(connection_state);

    const ref = useRef<HTMLButtonElement>(null);
    const { height } = useHTMLSizing(ref);

    async function cancel() {
        set_connect_state(() => ConnectionState.DISCONNECTED);
    }

    return (
        <div className={style.loading_container}>
            <h2>Server discovery in progress...</h2>
            <AppButton ref={ref} className={style.cancel_button} onClick={cancel}>
                <IoClose />
                <hr className={style.divider} data-height={height / 1.5} />
                <span>Cancel</span>
            </AppButton>
        </div>
    );
}
