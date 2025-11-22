import { useAtom } from "jotai";
import { useState } from "react";
import { FaSearch } from "react-icons/fa";

import { AppButton } from "$base/button";
import { AppInput } from "$base/input";
import { useStartConnection } from "$hook/useWebsocket";
import { server_port, server_url } from "$model/server_url";
import { PORT } from "$type/constant";
import { getNumber } from "$util/generic";

import { classnames } from "$util/react";
import { tryParseURL } from "$util/temp";
import style from "./index.module.less";

export function ServerURLComponent() {
    const start = useStartConnection();

    return (
        <div className={style.input_container}>
            <ServerInput start={() => start(true)} />
            <hr className={style.divider} />
            <div className={style.discover_server}>
                <AppButton onClick={() => start(false)}>Discover Servers</AppButton>
            </div>
        </div>
    );
}

function ServerInput({ start }: { start: () => Promise<void> }) {
    const [connect_url, set_connect_url] = useAtom(server_url);
    const [connect_port, set_connect_port] = useAtom(server_port);

    const [errorText, setErrorText] = useState("");

    async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);

        const { error, data } = parseForm(formData);

        if (error || !data) {
            setErrorText(error);
            return;
        }

        const { url, port } = data;

        set_connect_url(url);
        set_connect_port(port);
        await start();
    }

    return (
        <div className={style.server_input}>
            <form className={style.form} onSubmit={handleSubmit}>
                <AppInput
                    name="url"
                    placeholder="Enter server address"
                    defaultValue={connect_url}
                    className={classnames([style.form_input, "flex-grow-4"])}
                    onClick={() => setErrorText("")}
                />
                <AppInput
                    type="number"
                    min={PORT.MIN}
                    max={PORT.MAX}
                    name="port"
                    placeholder="9002"
                    defaultValue={connect_port}
                    className={classnames([style.form_input, "flex-grow-2"])}
                    onClick={() => setErrorText("")}
                />
                <AppButton
                    type="submit"
                    className={classnames([style.search_button, "flex-grow-1"])}
                >
                    <FaSearch />
                </AppButton>
            </form>
            {errorText && <div style={{ color: "red" }}>{errorText}</div>}
        </div>
    );
}

function parseForm(form: FormData) {
    const form_url = form.get("url")?.toString() ?? "";
    const port_number = getNumber(form.get("port")?.toString());

    const data = tryParseURL(`${form_url}:1000`);

    if (!data) {
        return { data: null, error: "Invalid. URL address!" };
    }

    if (port_number === undefined) {
        return { data: null, error: "Invalid. PORT address!" };
    }

    // Check if port is within valid range; PORT.MIN < port < PORT.MAX.
    if (PORT.MIN >= port_number || port_number >= PORT.MAX) {
        return {
            data: null,
            error: `Invalid. PORT address range! ${PORT.MIN}-${PORT.MAX}`,
        };
    }

    return { data: { url: data.url, port: port_number }, error: null };
}
