import { useAtom, useAtomValue } from "jotai";
import { useEffect } from "react";

import { BouncyTitle } from "$base/bouncyTitle";
import { AppButton } from "$base/button";
import { device_list, selected_device_id } from "$model/volume";

import style from "./index.module.less";

export function SidebarDevices() {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const audio_devices = useAtomValue(device_list);

    useEffect(() => {
        if (typeof selected_device === "undefined" && audio_devices.length) {
            // set either default device or the first device as "selected".
            const find_default =
                audio_devices.find((val) => val.is_default) ?? audio_devices[0];
            set_device_id(() => find_default.id);
        }
    }, [selected_device, audio_devices, set_device_id]);

    return (
        <div className={style.navbar}>
            {audio_devices.map((audio_device) => {
                return (
                    <AppButton
                        key={audio_device.id}
                        is_active={audio_device.id === selected_device}
                        className={style.navbar_title}
                        onClick={() => set_device_id(() => audio_device.id)}
                    >
                        <BouncyTitle
                            title={audio_device.friendly_name}
                            animate={audio_device.id === selected_device}
                        />
                    </AppButton>
                );
            })}
        </div>
    );
}
