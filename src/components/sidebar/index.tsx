import { useAtom } from "jotai";
import { useEffect } from "react";
import { CiSpeaker } from "react-icons/ci";

import { AppButton } from "$base/button";
import { useGenerateID } from "$hook/useGenerateID";
import { NavbarState, navbar_state } from "$model/nav";
import { audio_session, selected_device_id } from "$model/volume";
import { classnames } from "$util/react";
import style from "./index.module.less";

export function Sidebar() {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const [audio_devices, refreshable] = useAtom(audio_session);

    useEffect(() => {
        if (typeof selected_device === "undefined" && audio_devices.length) {
            // set either default device or the first device as "selected".
            const find_default =
                audio_devices.find((val) => val.device.is_default) ?? audio_devices[0];
            set_device_id(find_default.device.id);
        }
    }, [selected_device, audio_devices, set_device_id]);

    return (
        <aside className={style.container}>
            <h2>Devices</h2>
            {audio_devices
                .map((val) => val.device)
                .map((device) => (
                    <AppButton
                        is_active={device.id === selected_device}
                        key={device.id}
                        onClick={() => {
                            set_device_id(() => device.id);
                            // SOULD NOT CALL "refreshable" ON EVERYCLICK.
                            // TODO: Optimize this call, it's freezing the UI.
                            refreshable();
                        }}
                    >
                        {device.name}
                    </AppButton>
                ))}
        </aside>
    );
}

export function SidebarDevices() {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const [audio_devices, _refreshable] = useAtom(audio_session);
    const audio_devices_ids = useGenerateID(audio_devices);

    const [navbarState, _setNavbarState] = useAtom(navbar_state);

    const collapsed = navbarState === NavbarState.EXPANDED;

    const nav_item = classnames([style.navbar_title, !collapsed ? style.collapsed : ""]);

    useEffect(() => {
        if (typeof selected_device === "undefined" && audio_devices.length) {
            // set either default device or the first device as "selected".
            const find_default =
                audio_devices.find((val) => val.device.is_default) ?? audio_devices[0];
            set_device_id(find_default.device.id);
        }
    }, [selected_device, audio_devices, set_device_id]);

    return (
        <div className={style.navbar_devices}>
            {collapsed ? <h3>Devices</h3> : <h4>Devices</h4>}
            {audio_devices_ids.map((audio_device) => {
                const device = audio_device.element.device;
                return (
                    <div key={audio_device.id}>
                        <AppButton
                            is_active={device.id === selected_device}
                            className={nav_item}
                            onClick={() => set_device_id(() => device.id)}
                        >
                            <CiSpeaker />
                            <span>{device.name}</span>
                        </AppButton>
                        {!collapsed && <span>{device.name}</span>}
                    </div>
                );
            })}
        </div>
    );
}
