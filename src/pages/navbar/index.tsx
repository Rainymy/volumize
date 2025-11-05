import { useAtom, useSetAtom } from "jotai";
import { useEffect } from "react";
import { CiSpeaker } from "react-icons/ci";
import { FaAngleRight, FaArrowLeft, FaHamburger } from "react-icons/fa";
import { FiArrowLeft } from "react-icons/fi";
import { IoSettingsOutline } from "react-icons/io5";
import { NavLink, useLocation, useNavigate } from "react-router-dom";
import { AppButton, NavButton } from "$base/button";
import { useGenerateID } from "$hook/useGenerateID";
import { isVeritcalNavbar, NavbarState, navbar_state } from "$model/nav";
import { audio_session, selected_device_id } from "$model/volume";
import { NavigationType } from "$type/navigation";
import { classnames } from "$util/react";
import style from "./index.module.less";

export function Navbar() {
    const location = useLocation();
    const setVerticalNavbar = useSetAtom(isVeritcalNavbar);

    const isMainPath = location.pathname === NavigationType.MAIN;
    setVerticalNavbar(isMainPath);

    return isMainPath ? <VNavbar /> : <HNavbar />;
}

/**
 * Vertical Navbar
 * @returns
 */
export function VNavbar() {
    const navigate = useNavigate();
    const [navbarState, setNavbarState] = useAtom(navbar_state);

    function toggleExpanded() {
        if (NavbarState.COLLAPSED === navbarState) {
            setNavbarState(NavbarState.EXPANDED);
            return;
        }
        setNavbarState(NavbarState.COLLAPSED);
    }
    function hideNavbar() {
        setNavbarState(NavbarState.HIDDEN);
    }

    const isWide = navbarState === NavbarState.EXPANDED;

    const classname = classnames([
        style.navbar,
        style.vertical,
        navbarState === NavbarState.COLLAPSED ? style.collapsed : undefined,
        navbarState === NavbarState.EXPANDED ? style.wide : undefined,
        navbarState === NavbarState.HIDDEN ? style.hidden : undefined,
        // isWide ? style.wide : style.collapsed,
    ]);
    const nav_item = classnames([style.navbar_title, !isWide ? style.collapsed : ""]);

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
                <AppButton className={nav_item} onClick={() => toggleExpanded()}>
                    <FaHamburger />
                    <span>Menu</span>
                </AppButton>
                <AppButton className={nav_item} onClick={() => hideNavbar()}>
                    <FiArrowLeft />
                    <span>Hide</span>
                </AppButton>
            </div>

            <SidebarDevices collapsed={isWide} />

            {/*<NavLink to={NavigationType.HOME}>
                <NavButton>
                    <span>Home</span>
                </NavButton>
            </NavLink>*/}
            <NavButton onClick={() => navigate(NavigationType.SETTINGS)}>
                <IoSettingsOutline />
                {isWide && <span>Settings</span>}
            </NavButton>
        </aside>
    );
}

function SidebarDevices({ collapsed }: { collapsed: boolean }) {
    const [selected_device, set_device_id] = useAtom(selected_device_id);
    const [audio_devices, _refreshable] = useAtom(audio_session);
    const audio_devices_ids = useGenerateID(audio_devices);

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

/**
 * Horizontal Navbar
 * @returns
 */
export function HNavbar() {
    const location = useLocation();
    const navigate = useNavigate();

    return (
        <nav className={style.navbar}>
            {location.pathname !== NavigationType.HOME && (
                <NavButton onClick={() => navigate(-1)}>
                    <FaArrowLeft />
                    <span>Back</span>
                </NavButton>
            )}

            <div />

            <NavLink to={NavigationType.MAIN}>
                <NavButton>
                    <span>Main</span>
                </NavButton>
            </NavLink>

            {location.pathname !== NavigationType.SETTINGS && (
                <NavButton onClick={() => navigate(NavigationType.SETTINGS)}>
                    <span>Settings</span>
                    <IoSettingsOutline />
                </NavButton>
            )}
        </nav>
    );
}
