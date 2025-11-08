import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { useEffect } from "react";
import { FaAngleRight, FaArrowLeft, FaHamburger } from "react-icons/fa";
import { FiArrowLeft } from "react-icons/fi";
import { IoSettingsOutline } from "react-icons/io5";
import { useLocation, useNavigate } from "react-router-dom";
import { AppButton, NavButton } from "$base/button";
import { SidebarDevices } from "$component/sidebar";
import { isVeritcalNavbar, NavbarState, navbar_state } from "$model/nav";
import { connection_ready } from "$model/volume";
import { NavigationType } from "$type/navigation";
import { classnames } from "$util/react";
import style from "./index.module.less";

export function Navbar() {
    const location = useLocation();
    const setVerticalNavbar = useSetAtom(isVeritcalNavbar);

    const isMainPath = location.pathname === NavigationType.MAIN;
    // Not really sure why this needs to be in a useEffect.
    useEffect(() => {
        setVerticalNavbar(() => isMainPath);
    }, [isMainPath, setVerticalNavbar]);

    return isMainPath ? <VNavbar /> : <HNavbar />;
}

/**
 * Vertical Navbar
 * @returns
 */
export function VNavbar() {
    const navigate = useNavigate();
    const [navbarState, setNavbarState] = useAtom(navbar_state);

    const classname = classnames([
        style.navbar,
        style.vertical,
        navbarState === NavbarState.COLLAPSED ? style.collapsed : undefined,
        navbarState === NavbarState.EXPANDED ? style.wide : undefined,
        navbarState === NavbarState.HIDDEN ? style.hidden : undefined,
    ]);
    const item_class = classnames([
        style.navbar_title,
        navbarState !== NavbarState.EXPANDED ? style.collapsed : undefined,
    ]);

    function toggleExpanded() {
        if (NavbarState.COLLAPSED === navbarState) {
            setNavbarState(NavbarState.EXPANDED);
            return;
        }
        setNavbarState(NavbarState.COLLAPSED);
    }

    function detectClick() {
        if (navbarState === NavbarState.HIDDEN) {
            toggleExpanded();
        }
    }

    const pop_show_nav = classnames([
        style.hidden_navbar_button,
        navbarState !== NavbarState.HIDDEN ? style.hide : undefined,
    ]);

    return (
        <aside className={classname}>
            <AppButton className={pop_show_nav} onClick={detectClick}>
                <FaAngleRight />
            </AppButton>

            <div>
                <AppButton className={item_class} onClick={() => toggleExpanded()}>
                    <FaHamburger />
                    <span>Menu</span>
                </AppButton>
                <AppButton
                    className={item_class}
                    onClick={() => setNavbarState(NavbarState.HIDDEN)}
                >
                    <FiArrowLeft />
                    <span>Hide</span>
                </AppButton>
            </div>

            <SidebarDevices />

            <NavButton onClick={() => navigate(NavigationType.SETTINGS)}>
                <IoSettingsOutline />
                {navbarState === NavbarState.EXPANDED && <span>Settings</span>}
            </NavButton>
        </aside>
    );
}

/**
 * Horizontal Navbar
 * @returns
 */
export function HNavbar() {
    const location = useLocation();
    const navigate = useNavigate();
    const is_ready = useAtomValue(connection_ready);

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
                <NavButton onClick={() => navigate(NavigationType.HOME)}>
                    <span>Close</span>
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
