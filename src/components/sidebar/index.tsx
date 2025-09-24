import { useAtom } from "jotai";
import { useEffect } from "react";
import { AppButton } from "$component/button";
import { audio_session, selected_device_id } from "$model/volume";
import wrapper from "./index.module.less";

export function Sidebar() {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const [audio_devices, refreshable] = useAtom(audio_session);

    useEffect(() => {
        if (typeof selected_device === "undefined" && audio_devices.length) {
            // set either default device or the first device as "selected".
            const find_default =
                audio_devices.find((val) => val.device.is_default) ??
                audio_devices[0];
            set_device_id(find_default.device.id);
        }
    }, [selected_device, audio_devices, set_device_id]);

    return (
        <aside className={wrapper.container}>
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
