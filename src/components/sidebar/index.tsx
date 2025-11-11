import { useAtom, useAtomValue } from "jotai";
import { useEffect } from "react";
import { CiSpeaker } from "react-icons/ci";

import { AppButton } from "$base/button";
import { useGenerateID } from "$hook/useGenerateID";
import { NavbarState, navbar_state } from "$model/nav";
import { device_list, selected_device_id } from "$model/volume";
import { classnames } from "$util/react";

import style from "./index.module.less";

export function SidebarDevices() {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const audio_devices = useAtomValue(device_list);
    const navbarState = useAtomValue(navbar_state);

    const audio_devices_ids = useGenerateID(audio_devices);

    const collapsed = navbarState !== NavbarState.EXPANDED;
    const nav_item = classnames([style.navbar_title, collapsed ? style.collapsed : ""]);

    useEffect(() => {
        if (typeof selected_device === "undefined" && audio_devices.length) {
            // set either default device or the first device as "selected".
            const find_default =
                audio_devices.find((val) => val.is_default) ?? audio_devices[0];
            set_device_id(() => find_default.id);
        }
    }, [selected_device, audio_devices, set_device_id]);

    return (
        <div className={style.navbar_devices}>
            {collapsed ? <h4>Devices</h4> : <h3>Devices</h3>}
            {audio_devices_ids.map((audio_device) => {
                const device = audio_device.element;
                return (
                    <div key={audio_device.id}>
                        <AppButton
                            is_active={device.id === selected_device}
                            className={nav_item}
                            onClick={() => set_device_id(() => device.id)}
                        >
                            {collapsed && <CiSpeaker />}
                            {/*
                                FIX: friendly_name mostly likely to overflow.
                                 - add animation that bounces from right to left.
                                 - when expanded.
                            */}
                            <span>{device.friendly_name}</span>
                        </AppButton>
                        {/*{collapsed && <span>{device.friendly_name}</span>}*/}
                    </div>
                );
            })}
        </div>
    );
}
