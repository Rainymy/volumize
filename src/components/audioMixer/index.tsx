import { useState } from "react";
import { MainContent } from "$component/mainContent";
import { Sidebar } from "$component/sidebar";
import type { AudioSession } from "$type/volume";
import wrapper from "./index.module.less";

export function AudioMixer({ sessions }: { sessions: AudioSession[] }) {
    const defaultSession =
        sessions.find((val) => val.device.is_default) ?? sessions[0];

    const [selectedDevice, setSelectedDevice] = useState(defaultSession);
    if (sessions.length === 0) {
        return null;
    }

    return (
        <div className={wrapper.container}>
            {/* Sidebar devices */}
            <Sidebar
                devices={sessions.map((val) => val.device)}
                activeID={selectedDevice.device.id}
                onSelectDevice={(id: string) => {
                    const newSession = sessions.find((s) => s.device.id === id);
                    if (newSession) {
                        setSelectedDevice(newSession);
                    }
                }}
            />

            {/* Main content */}
            <MainContent session={selectedDevice}></MainContent>
        </div>
    );
}
