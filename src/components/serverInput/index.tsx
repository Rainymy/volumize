import { useAtom } from "jotai";
import { useState } from "react";

import { FaSearch } from "react-icons/fa";

import { AppButton } from "$base/button";
import { AppInput } from "$base/input";
import { server_port, server_url } from "$model/volume";
import { PORT } from "$type/globals";
import { getNumber } from "$util/generic";
import style from "./index.module.less";

export function ServerURLComponent() {
    return (
        <div className={style.box}>
            <ServerInput />
            <hr />
            <div className={style.discover_server}>
                <AppButton>Discover Servers</AppButton>
            </div>
        </div>
    );
}

function ServerInput() {
    const [connect_url, set_connect_url] = useAtom(server_url);
    const [connect_port, set_connect_port] = useAtom(server_port);
    const [errorText, setErrorText] = useState("");

    function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);

        const url = formData.get("url")?.toString();
        const port_number = getNumber(formData.get("port")?.toString());

        if (!url) {
            return setErrorText("Invalid. URL address!");
        }

        if (port_number === undefined) {
            return setErrorText("Invalid. PORT address!");
        }

        // Check if port is within valid range; PORT.MIN < port < PORT.MAX.
        if (PORT.MIN >= port_number || port_number >= PORT.MAX) {
            return setErrorText(`Invalid. PORT address range! ${PORT.MIN}-${PORT.MAX}`);
        }

        console.log("Setting new ULR and PORT address....");
        console.log("info: ", url, port_number);
        set_connect_url(url);
        set_connect_port(port_number);
    }

    return (
        <div className={style.server_input}>
            <form onSubmit={handleSubmit}>
                <AppInput
                    name="url"
                    placeholder="Enter server address"
                    defaultValue={connect_url}
                />
                <AppInput
                    type="number"
                    min={PORT.MIN}
                    max={PORT.MAX}
                    name="port"
                    placeholder="9001"
                    defaultValue={connect_port}
                />
                <AppButton type="submit">
                    <FaSearch />
                </AppButton>
            </form>
            {errorText && <div style={{ color: "red" }}>{errorText}</div>}
        </div>
    );
}
