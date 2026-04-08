import { useAtom, useAtomValue, useSetAtom } from "jotai";

import { Activity, useCallback, useEffect, useMemo } from "react";
import { FaArrowLeft, FaHamburger } from "react-icons/fa";
import { FaAngleRight } from "react-icons/fa6";
import { FiArrowLeft } from "react-icons/fi";
import { IoSettingsOutline } from "react-icons/io5";
import { useLocation, useNavigate } from "react-router";

import { AppButton, NavButton } from "$base/button";
import { CollapseButton } from "$base/collapseButton";
import { SidebarDevices } from "$component/sidebar";
import { useLogout } from "$hook/useLogout";
import { isVeritcalNavbar, NavbarState, navbar_state } from "$model/nav";
import { connection_state } from "$model/volume";
import { ConnectionState, NavigationType } from "$type/navigation";
import { classnames } from "$util/react";

import style from "./index.module.less";

export function Navbar() {
    const location = useLocation();
    const setVerticalNavbar = useSetAtom(isVeritcalNavbar);
    const [navbarState, setNavbarState] = useAtom(navbar_state);

    const isMainPath = useMemo(
        () => location.pathname === NavigationType.MAIN,
        [location.pathname],
    );
    // Not really sure why this needs to be in a useEffect.
    useEffect(() => {
        setVerticalNavbar(() => isMainPath);
    }, [isMainPath, setVerticalNavbar]);

    const toggleExpanded = useCallback(() => {
        if (NavbarState.COLLAPSED === navbarState) {
            setNavbarState(NavbarState.EXPANDED);
            return;
        }
        setNavbarState(NavbarState.COLLAPSED);
    }, [navbarState, setNavbarState]);

    const is_nav_hidden = navbarState === NavbarState.HIDDEN;

    return (
        <>
            <Activity mode={isMainPath ? "visible" : "hidden"}>
                <AppButton
                    className={classnames([
                        style.hidden_navbar_button,
                        !is_nav_hidden ? style.hide : undefined,
                    ])}
                    onClick={() => is_nav_hidden && toggleExpanded()}
                >
                    <FaAngleRight />
                </AppButton>
                <VNavbar toggleExpanded={toggleExpanded} />
            </Activity>
            <Activity mode={!isMainPath ? "visible" : "hidden"}>
                <HNavbar />
            </Activity>
        </>
    );
}

/**
 * Vertical Navbar
 * @returns
 */
function VNavbar({ toggleExpanded }: { toggleExpanded: () => void }) {
    const navigate = useNavigate();
    const [navbarState, setNavbarState] = useAtom(navbar_state);

    const classname = classnames([
        style.navbar,
        style.vertical,
        navbarState === NavbarState.COLLAPSED ? style.collapsed : undefined,
        navbarState === NavbarState.EXPANDED ? style.wide : undefined,
        navbarState === NavbarState.HIDDEN ? style.hidden : undefined,
    ]);

    return (
        <aside className={classname}>
            <div className={style.navbar_entry}>
                <CollapseButton
                    collapsed={navbarState === NavbarState.COLLAPSED}
                    icon={<FaHamburger />}
                    onClick={() => toggleExpanded()}
                    text="Menu"
                />
                <CollapseButton
                    collapsed={navbarState === NavbarState.COLLAPSED}
                    icon={<FiArrowLeft />}
                    onClick={() => setNavbarState(NavbarState.HIDDEN)}
                    text="Hide"
                />
            </div>

            <div className={style.navbar_entry}>
                <h3>Devices</h3>
                <SidebarDevices />
            </div>

            <div className={style.navbar_entry}>
                <CollapseButton
                    collapsed={navbarState === NavbarState.COLLAPSED}
                    icon={<IoSettingsOutline />}
                    CustomElement={NavButton}
                    onClick={() => navigate(NavigationType.SETTINGS)}
                    text="Settings"
                />
            </div>
        </aside>
    );
}

/**
 * Horizontal Navbar
 * @returns
 */
function HNavbar() {
    const location = useLocation();
    const navigate = useNavigate();

    const state = useAtomValue(connection_state);
    const logout = useLogout();

    const is_ready = state === ConnectionState.CONNECTED;

    return (
        <nav className={style.navbar}>
            {location.pathname !== NavigationType.HOME && (
                <NavButton onClick={() => navigate(-1)}>
                    <FaArrowLeft />
                    <span>Back</span>
                </NavButton>
            )}

            <div />

            {is_ready && (
                <NavButton onClick={logout}>
                    <span>Logout</span>
                </NavButton>
            )}

            {location.pathname !== NavigationType.SETTINGS && (
                <NavButton onClick={() => navigate(NavigationType.SETTINGS)}>
                    <span>Settings</span>
                    <IoSettingsOutline />
                </NavButton>
            )}
        </nav>
    );
}
