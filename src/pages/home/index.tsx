import { FaSearch } from "react-icons/fa";
import { IoClose } from "react-icons/io5";

import { AppLogo } from "$base/appLogo";
import { AppButton } from "$base/button";
import { AppInput } from "$base/input";

import style from "./index.module.less";

export function Entry() {
    const isLoading = false;

    return (
        <div className={style.box}>
            <AppLogo />
            {isLoading ? <ServerDiscoveryLoading /> : <ServerDiscoveryInput />}
        </div>
    );
}

function ServerInput() {
    const style = {
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "0.75rem 0",
        gap: "0.5rem",

        width: "100%",
    };

    return (
        <div style={style}>
            <AppInput placeholder="Enter server address" />
            <AppButton type="submit">
                <FaSearch />
            </AppButton>
        </div>
    );
}

function ServerDiscoveryInput() {
    const style = {
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "0.75rem 0",
        gap: "0.5rem",

        width: "100%",
    };

    return (
        <div>
            <ServerInput />
            <hr />
            <div style={style}>
                <AppButton>Discover Servers</AppButton>
            </div>
        </div>
    );
}

function ServerDiscoveryLoading() {
    return (
        <div>
            <h2>Server discovery in progress...</h2>
            <AppButton>
                <IoClose /> Cancel
            </AppButton>
        </div>
    );
}
