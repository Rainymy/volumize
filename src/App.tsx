import { useAtomValue } from "jotai";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { Notice } from "$base/notice";
import { MainContent } from "$component/mainContent";
import { isVeritcalNavbar, NavbarState, navbar_state } from "$model/nav";
// import { AudioMixer } from "$component/audioMixer";
import { Entry } from "$page/home";
import { Navbar } from "$page/navbar";
import { Settings } from "$page/settings";
import { NavigationType } from "$type/navigation";
import { classnames } from "$util/react";

export default function App() {
    const isVerticalNavbar = useAtomValue(isVeritcalNavbar);
    const navbarState = useAtomValue(navbar_state);

    const main_classname = classnames([
        "navbar-container",
        isVerticalNavbar ? "vertical" : "",
    ]);

    const content_classname = classnames([
        "container",
        navbarState === NavbarState.COLLAPSED ? "nav_collapsed" : "",
        navbarState === NavbarState.EXPANDED ? "nav_expanded" : "",
        navbarState === NavbarState.HIDDEN ? "nav_hidden" : "",
    ]);

    // nav_collapsed
    // nav_expanded

    return (
        <div className={main_classname}>
            <BrowserRouter>
                <Navbar />
                <main className={content_classname}>
                    <Routes>
                        <Route path={NavigationType.HOME} element={<Entry />} />
                        <Route path={NavigationType.SETTINGS} element={<Settings />} />
                        <Route path={NavigationType.MAIN} element={<MainContent />} />
                    </Routes>
                </main>
                {false && <ErrorInfoComp />}
            </BrowserRouter>
        </div>
    );
}

function ErrorInfoComp() {
    return <Notice>Something went wrong</Notice>;
}
