import { lazy } from "react";
import { Route, Routes } from "react-router";

import { Notice } from "$base/notice";
import { Navbar } from "$component/navbar";
import { useIsNavigateToSettings, useNavbarState } from "$hook/useNavbar";
import { NavigationType } from "$type/navigation";
import { classnames } from "$util/react";

const Entry = lazy(() => import("$page/home"));
const MainContent = lazy(() => import("$component/mainContent"));
const Settings = lazy(() => import("$page/settings"));

export default function AudioMixer() {
    const navbarState = useNavbarState();
    const isDestinationSettings = useIsNavigateToSettings();

    const main_classname = classnames([
        "navbar-container",
        navbarState.isVertical ? "vertical" : undefined,
    ]);

    const content_classname = classnames([
        "container",
        navbarState.classnames,
        isDestinationSettings ? "no_transition" : undefined,
    ]);

    return (
        <div className={main_classname}>
            <Navbar />
            <main className={content_classname}>
                <Routes>
                    <Route path={NavigationType.HOME} element={<Entry />} />
                    <Route path={NavigationType.SETTINGS} element={<Settings />} />
                    <Route path={NavigationType.MAIN} element={<MainContent />} />
                </Routes>
            </main>
            {false && <Notice>Something went wrong</Notice>}
        </div>
    );
}
